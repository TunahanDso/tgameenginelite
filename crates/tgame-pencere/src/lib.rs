//! Tgame Engine Lite pencere ve işletim sistemi olay döngüsü.

use tgame_cekirdek::{Cozunurluk, OyunHatasi, OyunSonucu};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

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
/// Çözünürlük geçersizse, olay döngüsü oluşturulamazsa veya işletim sistemi
/// pencere oluşturmayı reddederse [`OyunHatasi`] döndürür.
pub fn calistir(ayarlar: PencereAyarlari) -> OyunSonucu {
    ayarlar.cozunurluk.dogrula()?;

    let olay_dongusu = EventLoop::new()
        .map_err(|hata| OyunHatasi::yeni(format!("Pencere olay döngüsü oluşturulamadı: {hata}")))?;
    let mut uygulama = Uygulama::yeni(ayarlar);

    olay_dongusu
        .run_app(&mut uygulama)
        .map_err(|hata| OyunHatasi::yeni(format!("Pencere olay döngüsü durdu: {hata}")))?;

    if let Some(hata) = uygulama.hata {
        return Err(hata);
    }

    Ok(())
}

#[derive(Debug)]
struct Uygulama {
    ayarlar: PencereAyarlari,
    pencere: Option<Window>,
    hata: Option<OyunHatasi>,
}

impl Uygulama {
    fn yeni(ayarlar: PencereAyarlari) -> Self {
        Self {
            ayarlar,
            pencere: None,
            hata: None,
        }
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

        match olay_dongusu.create_window(nitelikler) {
            Ok(pencere) => self.pencere = Some(pencere),
            Err(hata) => {
                self.hata = Some(OyunHatasi::yeni(format!(
                    "Oyun penceresi oluşturulamadı: {hata}"
                )));
                olay_dongusu.exit();
            }
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

        if olay == WindowEvent::CloseRequested {
            olay_dongusu.exit();
        }
    }
}

#[cfg(test)]
mod testler {
    use super::PencereAyarlari;
    use tgame_cekirdek::Cozunurluk;

    #[test]
    fn pencere_ayarlari_degerleri_korur() {
        let ayarlar = PencereAyarlari::yeni("Tgame", Cozunurluk::yeni(800, 600));

        assert_eq!(ayarlar.baslik(), "Tgame");
        assert_eq!(ayarlar.cozunurluk(), Cozunurluk::yeni(800, 600));
    }
}
