//! Tgame Engine Lite pencere ve işletim sistemi olay döngüsü.

use std::sync::Arc;

use num_traits::ToPrimitive;
use tgame_cekirdek::{Cozunurluk, OyunHatasi, OyunSonucu};
use tgame_girdi::{Girdi, Tus};
use tgame_grafik::Grafik;
use tgame_varlik::Dunya;
use tgame_zaman::{Zaman, ZamanYoneticisi};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{DeviceEvent, DeviceId, ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window, WindowId},
};

/// Her kare sonunda motorun nasıl devam edeceğini belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OyunAkisi {
    /// Oyun döngüsünün çalışmaya devam etmesini sağlar.
    #[default]
    DevamEt,
    /// Oyun döngüsünü kontrollü biçimde kapatır.
    Kapat,
}

/// Oyun geliştiricisinin her karede çalıştırdığı Türkçe güncelleme görevidir.
pub type KareGorevi = Box<dyn FnMut(&Girdi, &Zaman, &mut Dunya) -> OyunAkisi>;

/// Oluşturulacak oyun penceresinin ayarlarını taşır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PencereAyarlari {
    baslik: String,
    cozunurluk: Cozunurluk,
}

impl PencereAyarlari {
    /// Yeni pencere ayarları oluşturur.
    #[must_use]
    pub fn yeni(baslik: impl Into<String>, cozunurluk: Cozunurluk) -> Self {
        Self {
            baslik: baslik.into(),
            cozunurluk,
        }
    }

    /// Pencere başlığını döndürür.
    #[must_use]
    pub fn baslik(&self) -> &str {
        &self.baslik
    }

    /// Pencerenin iç çözünürlüğünü döndürür.
    #[must_use]
    pub const fn cozunurluk(&self) -> Cozunurluk {
        self.cozunurluk
    }
}

/// İşletim sistemi penceresini oluşturur ve olay döngüsünü çalıştırır.
///
/// # Errors
///
/// Çözünürlük geçersizse, olay döngüsü, pencere veya GPU grafik sistemi
/// oluşturulamazsa [`OyunHatasi`] döndürür.
pub fn calistir(ayarlar: PencereAyarlari, dunya: Dunya, kare_gorevi: KareGorevi) -> OyunSonucu {
    ayarlar.cozunurluk.dogrula()?;

    let olay_dongusu = EventLoop::new()
        .map_err(|hata| OyunHatasi::yeni(format!("Pencere olay döngüsü oluşturulamadı: {hata}")))?;
    let mut uygulama = Uygulama::yeni(ayarlar, dunya, kare_gorevi);

    olay_dongusu
        .run_app(&mut uygulama)
        .map_err(|hata| OyunHatasi::yeni(format!("Pencere olay döngüsü durdu: {hata}")))?;

    if let Some(hata) = uygulama.hata {
        return Err(hata);
    }

    Ok(())
}

struct Uygulama {
    ayarlar: PencereAyarlari,
    pencere: Option<Arc<Window>>,
    grafik: Option<Grafik>,
    dunya: Dunya,
    girdi: Girdi,
    zaman: ZamanYoneticisi,
    kare_gorevi: KareGorevi,
    hata: Option<OyunHatasi>,
    odakli: bool,
}

impl Uygulama {
    fn yeni(ayarlar: PencereAyarlari, dunya: Dunya, kare_gorevi: KareGorevi) -> Self {
        Self {
            ayarlar,
            pencere: None,
            grafik: None,
            dunya,
            girdi: Girdi::yeni(),
            zaman: ZamanYoneticisi::yeni(),
            kare_gorevi,
            hata: None,
            odakli: true,
        }
    }

    fn hata_ile_dur(&mut self, olay_dongusu: &ActiveEventLoop, hata: OyunHatasi) {
        self.hata = Some(hata);
        olay_dongusu.exit();
    }
}

impl ApplicationHandler for Uygulama {
    fn resumed(&mut self, olay_dongusu: &ActiveEventLoop) {
        if self.pencere.is_some() {
            return;
        }

        let cozunurluk = self.ayarlar.cozunurluk;
        let boyut = LogicalSize::new(
            f64::from(cozunurluk.genislik),
            f64::from(cozunurluk.yukseklik),
        );
        let nitelikler = Window::default_attributes()
            .with_title(self.ayarlar.baslik.clone())
            .with_inner_size(boyut);
        let pencere = match olay_dongusu.create_window(nitelikler) {
            Ok(pencere) => Arc::new(pencere),
            Err(hata) => {
                self.hata_ile_dur(
                    olay_dongusu,
                    OyunHatasi::yeni(format!("Oyun penceresi oluşturulamadı: {hata}")),
                );
                return;
            }
        };
        let grafik = match pollster::block_on(Grafik::yeni(Arc::clone(&pencere))) {
            Ok(grafik) => grafik,
            Err(hata) => {
                self.hata_ile_dur(olay_dongusu, hata);
                return;
            }
        };

        fareyi_yakala(&pencere, true);
        pencere.request_redraw();
        self.pencere = Some(pencere);
        self.grafik = Some(grafik);
    }

    fn device_event(
        &mut self,
        _olay_dongusu: &ActiveEventLoop,
        _aygit_kimligi: DeviceId,
        olay: DeviceEvent,
    ) {
        if !self.odakli {
            return;
        }

        if let DeviceEvent::MouseMotion { delta } = olay {
            let x = delta.0.to_f32().unwrap_or_default();
            let y = delta.1.to_f32().unwrap_or_default();
            self.girdi.fare_hareketini_ekle(x, y);
        }
    }

    fn window_event(
        &mut self,
        olay_dongusu: &ActiveEventLoop,
        pencere_kimligi: WindowId,
        olay: WindowEvent,
    ) {
        let Some(pencere) = self.pencere.as_ref() else {
            return;
        };

        if pencere.id() != pencere_kimligi {
            return;
        }

        match olay {
            WindowEvent::CloseRequested => olay_dongusu.exit(),
            WindowEvent::Focused(odakli) => {
                self.odakli = odakli;
                fareyi_yakala(pencere, odakli);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(kod) = event.physical_key {
                    if let Some(tus) = tusa_cevir(kod) {
                        self.girdi
                            .tus_durumunu_guncelle(tus, event.state == ElementState::Pressed);
                    }
                }
            }
            WindowEvent::Resized(boyut) => {
                if let Some(grafik) = self.grafik.as_mut() {
                    grafik.boyutlandir(Cozunurluk::yeni(boyut.width, boyut.height));
                }
                pencere.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let zaman = self.zaman.kareyi_baslat();
                let akis = (self.kare_gorevi)(&self.girdi, &zaman, &mut self.dunya);

                if akis == OyunAkisi::Kapat {
                    self.girdi.kareyi_bitir();
                    olay_dongusu.exit();
                    return;
                }

                let Some(grafik) = self.grafik.as_mut() else {
                    return;
                };

                if let Err(hata) = grafik.ciz(&self.dunya) {
                    self.hata = Some(hata);
                    olay_dongusu.exit();
                    return;
                }

                self.girdi.kareyi_bitir();
                pencere.request_redraw();
            }
            _ => {}
        }
    }
}

fn fareyi_yakala(pencere: &Window, yakala: bool) {
    if yakala {
        if pencere.set_cursor_grab(CursorGrabMode::Locked).is_err() {
            let _ = pencere.set_cursor_grab(CursorGrabMode::Confined);
        }
        pencere.set_cursor_visible(false);
    } else {
        let _ = pencere.set_cursor_grab(CursorGrabMode::None);
        pencere.set_cursor_visible(true);
    }
}

fn tusa_cevir(kod: KeyCode) -> Option<Tus> {
    match kod {
        KeyCode::KeyA => Some(Tus::A),
        KeyCode::KeyB => Some(Tus::B),
        KeyCode::KeyC => Some(Tus::C),
        KeyCode::KeyD => Some(Tus::D),
        KeyCode::KeyE => Some(Tus::E),
        KeyCode::KeyF => Some(Tus::F),
        KeyCode::KeyG => Some(Tus::G),
        KeyCode::KeyH => Some(Tus::H),
        KeyCode::KeyI => Some(Tus::I),
        KeyCode::KeyJ => Some(Tus::J),
        KeyCode::KeyK => Some(Tus::K),
        KeyCode::KeyL => Some(Tus::L),
        KeyCode::KeyM => Some(Tus::M),
        KeyCode::KeyN => Some(Tus::N),
        KeyCode::KeyO => Some(Tus::O),
        KeyCode::KeyP => Some(Tus::P),
        KeyCode::KeyQ => Some(Tus::Q),
        KeyCode::KeyR => Some(Tus::R),
        KeyCode::KeyS => Some(Tus::S),
        KeyCode::KeyT => Some(Tus::T),
        KeyCode::KeyU => Some(Tus::U),
        KeyCode::KeyV => Some(Tus::V),
        KeyCode::KeyW => Some(Tus::W),
        KeyCode::KeyX => Some(Tus::X),
        KeyCode::KeyY => Some(Tus::Y),
        KeyCode::KeyZ => Some(Tus::Z),
        KeyCode::Digit0 => Some(Tus::Sayi0),
        KeyCode::Digit1 => Some(Tus::Sayi1),
        KeyCode::Digit2 => Some(Tus::Sayi2),
        KeyCode::Digit3 => Some(Tus::Sayi3),
        KeyCode::Digit4 => Some(Tus::Sayi4),
        KeyCode::Digit5 => Some(Tus::Sayi5),
        KeyCode::Digit6 => Some(Tus::Sayi6),
        KeyCode::Digit7 => Some(Tus::Sayi7),
        KeyCode::Digit8 => Some(Tus::Sayi8),
        KeyCode::Digit9 => Some(Tus::Sayi9),
        KeyCode::ArrowUp => Some(Tus::Yukari),
        KeyCode::ArrowDown => Some(Tus::Asagi),
        KeyCode::ArrowLeft => Some(Tus::Sol),
        KeyCode::ArrowRight => Some(Tus::Sag),
        KeyCode::Space => Some(Tus::Bosluk),
        KeyCode::Enter => Some(Tus::Enter),
        KeyCode::Escape => Some(Tus::Kacis),
        KeyCode::Tab => Some(Tus::Sekme),
        KeyCode::Backspace => Some(Tus::GeriSil),
        KeyCode::ShiftLeft => Some(Tus::SolShift),
        KeyCode::ShiftRight => Some(Tus::SagShift),
        KeyCode::ControlLeft => Some(Tus::SolKontrol),
        KeyCode::ControlRight => Some(Tus::SagKontrol),
        KeyCode::AltLeft => Some(Tus::SolAlt),
        KeyCode::AltRight => Some(Tus::SagAlt),
        KeyCode::F1 => Some(Tus::F1),
        KeyCode::F2 => Some(Tus::F2),
        KeyCode::F3 => Some(Tus::F3),
        KeyCode::F4 => Some(Tus::F4),
        KeyCode::F5 => Some(Tus::F5),
        KeyCode::F6 => Some(Tus::F6),
        KeyCode::F7 => Some(Tus::F7),
        KeyCode::F8 => Some(Tus::F8),
        KeyCode::F9 => Some(Tus::F9),
        KeyCode::F10 => Some(Tus::F10),
        KeyCode::F11 => Some(Tus::F11),
        KeyCode::F12 => Some(Tus::F12),
        _ => None,
    }
}

#[cfg(test)]
mod testler {
    use super::{OyunAkisi, PencereAyarlari, tusa_cevir};
    use tgame_cekirdek::Cozunurluk;
    use tgame_girdi::Tus;
    use winit::keyboard::KeyCode;

    #[test]
    fn pencere_ayarlari_degerleri_korur() {
        let ayarlar = PencereAyarlari::yeni("Tgame", Cozunurluk::yeni(800, 600));

        assert_eq!(ayarlar.baslik(), "Tgame");
        assert_eq!(ayarlar.cozunurluk(), Cozunurluk::yeni(800, 600));
    }

    #[test]
    fn fiziksel_tuslar_turkce_tuslara_cevrilir() {
        assert_eq!(tusa_cevir(KeyCode::KeyW), Some(Tus::W));
        assert_eq!(tusa_cevir(KeyCode::Escape), Some(Tus::Kacis));
        assert_eq!(tusa_cevir(KeyCode::ArrowLeft), Some(Tus::Sol));
        assert_eq!(tusa_cevir(KeyCode::CapsLock), None);
    }

    #[test]
    fn varsayilan_oyun_akisi_devam_eder() {
        assert_eq!(OyunAkisi::default(), OyunAkisi::DevamEt);
    }
}
