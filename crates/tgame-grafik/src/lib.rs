//! Tgame Engine Lite GPU grafik katmanı.

mod ikiboyut;
mod ucboyut;

use std::sync::Arc;

use ikiboyut::IkiBoyutGrafik;
use tgame_cekirdek::{Cozunurluk, OyunHatasi, OyunSonucu};
use tgame_varlik::{Dunya, DunyaBoyutu};
use ucboyut::UcBoyutGrafik;
use winit::window::Window;

/// Pencereye bağlı GPU yüzeyini ve iki/üç boyutlu çizim sunucularını yönetir.
pub struct Grafik {
    pencere: Arc<Window>,
    ornek: wgpu::Instance,
    yuzey: wgpu::Surface<'static>,
    bagdastirici: wgpu::Adapter,
    aygit: wgpu::Device,
    kuyruk: wgpu::Queue,
    yapilandirma: wgpu::SurfaceConfiguration,
    ikiboyut: IkiBoyutGrafik,
    ucboyut: UcBoyutGrafik,
    boyut: Cozunurluk,
}

impl Grafik {
    /// Pencere için yüksek performanslı GPU bağlamı oluşturur.
    ///
    /// # Errors
    ///
    /// GPU yüzeyi, uygun bağdaştırıcı, aygıt veya yüzey yapılandırması
    /// oluşturulamazsa [`OyunHatasi`] döndürür.
    pub async fn yeni(pencere: Arc<Window>) -> OyunSonucu<Self> {
        let fiziksel_boyut = pencere.inner_size();
        let boyut = Cozunurluk::yeni(fiziksel_boyut.width.max(1), fiziksel_boyut.height.max(1));
        let ornek = wgpu::Instance::default();
        let yuzey = ornek
            .create_surface(Arc::clone(&pencere))
            .map_err(|hata| OyunHatasi::yeni(format!("GPU yüzeyi oluşturulamadı: {hata}")))?;
        let bagdastirici = ornek
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&yuzey),
                apply_limit_buckets: true,
            })
            .await
            .map_err(|hata| OyunHatasi::yeni(format!("Uygun GPU bulunamadı: {hata}")))?;
        let (aygit, kuyruk) = bagdastirici
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Tgame GPU Aygıtı"),
                ..Default::default()
            })
            .await
            .map_err(|hata| OyunHatasi::yeni(format!("GPU aygıtı oluşturulamadı: {hata}")))?;
        let yapilandirma = yuzey
            .get_default_config(&bagdastirici, boyut.genislik, boyut.yukseklik)
            .ok_or_else(|| OyunHatasi::yeni("GPU, pencere yüzeyini desteklemiyor."))?;
        let ikiboyut = IkiBoyutGrafik::yeni(&aygit, yapilandirma.format);
        let ucboyut = UcBoyutGrafik::yeni(&aygit, &kuyruk, yapilandirma.format, boyut);

        yuzey.configure(&aygit, &yapilandirma);

        Ok(Self {
            pencere,
            ornek,
            yuzey,
            bagdastirici,
            aygit,
            kuyruk,
            yapilandirma,
            ikiboyut,
            ucboyut,
            boyut,
        })
    }

    /// GPU yüzeyini ve üç boyutlu derinlik dokusunu yeni pencere boyutuna uyarlar.
    pub fn boyutlandir(&mut self, boyut: Cozunurluk) {
        if boyut.genislik == 0 || boyut.yukseklik == 0 {
            return;
        }

        self.boyut = boyut;
        self.yapilandirma.width = boyut.genislik;
        self.yapilandirma.height = boyut.yukseklik;
        self.yuzeyi_yapilandir();
        self.ucboyut.boyutlandir(&self.aygit, boyut);
    }

    /// Dünyayı etkin boyutuna göre 2B veya derinlikli 3B olarak sunar.
    ///
    /// # Errors
    ///
    /// GPU tamponu büyütülemezse, GPU yüzeyi kaybolur ve yeniden oluşturulamazsa
    /// veya yüzey doğrulama hatası oluşursa [`OyunHatasi`] döndürür.
    pub fn ciz(&mut self, dunya: &Dunya) -> OyunSonucu {
        let ornek_sayisi = match dunya.boyut() {
            DunyaBoyutu::IkiBoyut => self.ikiboyut.hazirla(
                &self.aygit,
                &self.kuyruk,
                dunya,
                self.yapilandirma.width,
                self.yapilandirma.height,
            )?,
            DunyaBoyutu::UcBoyut => self.ucboyut.hazirla(
                &self.aygit,
                &self.kuyruk,
                dunya,
                self.yapilandirma.width,
                self.yapilandirma.height,
            )?,
        };

        let (kare, yeniden_yapilandir) = match self.yuzey.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(kare) => (kare, false),
            wgpu::CurrentSurfaceTexture::Suboptimal(kare) => (kare, true),
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.yuzeyi_yapilandir();
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                self.yuzeyi_yenile()?;
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err(OyunHatasi::yeni(
                    "GPU yüzeyinden kare alınırken doğrulama hatası oluştu.",
                ));
            }
        };
        let gorunum = kare
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut komut_kaydedici =
            self.aygit
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Tgame Kare Komutları"),
                });

        match dunya.boyut() {
            DunyaBoyutu::IkiBoyut => {
                self.ikiboyut
                    .kaydet(&mut komut_kaydedici, &gorunum, ornek_sayisi);
            }
            DunyaBoyutu::UcBoyut => {
                self.ucboyut
                    .kaydet(&mut komut_kaydedici, &gorunum, ornek_sayisi);
            }
        }

        self.kuyruk.submit(Some(komut_kaydedici.finish()));
        self.kuyruk.present(kare);

        if yeniden_yapilandir {
            self.yuzeyi_yapilandir();
        }

        Ok(())
    }

    fn yuzeyi_yapilandir(&self) {
        self.yuzey.configure(&self.aygit, &self.yapilandirma);
    }

    fn yuzeyi_yenile(&mut self) -> OyunSonucu {
        let yeni_yuzey = self
            .ornek
            .create_surface(Arc::clone(&self.pencere))
            .map_err(|hata| OyunHatasi::yeni(format!("GPU yüzeyi yenilenemedi: {hata}")))?;
        let yeni_yapilandirma = yeni_yuzey
            .get_default_config(
                &self.bagdastirici,
                self.boyut.genislik,
                self.boyut.yukseklik,
            )
            .ok_or_else(|| OyunHatasi::yeni("Yenilenen GPU yüzeyi desteklenmiyor."))?;

        if yeni_yapilandirma.format != self.yapilandirma.format {
            self.ikiboyut
                .yuzey_bicimini_degistir(&self.aygit, yeni_yapilandirma.format);
            self.ucboyut
                .yuzey_bicimini_degistir(&self.aygit, yeni_yapilandirma.format);
        }

        yeni_yuzey.configure(&self.aygit, &yeni_yapilandirma);
        self.yuzey = yeni_yuzey;
        self.yapilandirma = yeni_yapilandirma;
        self.ucboyut.boyutlandir(&self.aygit, self.boyut);
        Ok(())
    }
}
