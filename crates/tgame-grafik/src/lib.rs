//! Tgame Engine Lite GPU grafik katmanı.

use std::sync::Arc;

use tgame_cekirdek::{Cozunurluk, OyunHatasi, OyunSonucu};
use winit::window::Window;

const UCGEN_GOLGELENDIRICISI: &str = include_str!("ucgen.wgsl");

/// Pencereye bağlı GPU yüzeyini ve çizim kaynaklarını yönetir.
pub struct Grafik {
    pencere: Arc<Window>,
    ornek: wgpu::Instance,
    yuzey: wgpu::Surface<'static>,
    bagdastirici: wgpu::Adapter,
    aygit: wgpu::Device,
    kuyruk: wgpu::Queue,
    yapilandirma: wgpu::SurfaceConfiguration,
    cizim_hatti: wgpu::RenderPipeline,
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
        let cizim_hatti = cizim_hatti_olustur(&aygit, yapilandirma.format);

        yuzey.configure(&aygit, &yapilandirma);

        Ok(Self {
            pencere,
            ornek,
            yuzey,
            bagdastirici,
            aygit,
            kuyruk,
            yapilandirma,
            cizim_hatti,
            boyut,
        })
    }

    /// GPU yüzeyini yeni pencere boyutuna uyarlar.
    pub fn boyutlandir(&mut self, boyut: Cozunurluk) {
        if boyut.genislik == 0 || boyut.yukseklik == 0 {
            return;
        }

        self.boyut = boyut;
        self.yapilandirma.width = boyut.genislik;
        self.yapilandirma.height = boyut.yukseklik;
        self.yuzeyi_yapilandir();
    }

    /// Bir kare çizip pencere yüzeyine sunar.
    ///
    /// # Errors
    ///
    /// GPU yüzeyi kaybolur ve yeniden oluşturulamazsa veya yüzey doğrulama
    /// hatası oluşursa [`OyunHatasi`] döndürür.
    pub fn ciz(&mut self) -> OyunSonucu {
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
        let renk_eklentileri = [Some(wgpu::RenderPassColorAttachment {
            view: &gorunum,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.025,
                    g: 0.035,
                    b: 0.060,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        })];

        {
            let mut cizim_gecisi = komut_kaydedici.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Tgame Ana Çizim Geçişi"),
                color_attachments: &renk_eklentileri,
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            cizim_gecisi.set_pipeline(&self.cizim_hatti);
            cizim_gecisi.draw(0..3, 0..1);
        }

        self.kuyruk.submit(Some(komut_kaydedici.finish()));
        kare.present();

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
            self.cizim_hatti = cizim_hatti_olustur(&self.aygit, yeni_yapilandirma.format);
        }

        yeni_yuzey.configure(&self.aygit, &yeni_yapilandirma);
        self.yuzey = yeni_yuzey;
        self.yapilandirma = yeni_yapilandirma;
        Ok(())
    }
}

fn cizim_hatti_olustur(
    aygit: &wgpu::Device,
    yuzey_bicimi: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let golgelendirici = aygit.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Tgame İlk Üçgen Gölgelendiricisi"),
        source: wgpu::ShaderSource::Wgsl(UCGEN_GOLGELENDIRICISI.into()),
    });
    let renk_hedefleri = [Some(wgpu::ColorTargetState {
        format: yuzey_bicimi,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
    })];

    aygit.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Tgame İlk Üçgen Çizim Hattı"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &golgelendirici,
            entry_point: Some("tepe_ana"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &golgelendirici,
            entry_point: Some("parca_ana"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &renk_hedefleri,
        }),
        multiview_mask: None,
        cache: None,
    })
}

#[cfg(test)]
mod testler {
    use super::UCGEN_GOLGELENDIRICISI;

    #[test]
    fn ucgen_golgelendiricisi_giris_noktalarini_icerir() {
        assert!(UCGEN_GOLGELENDIRICISI.contains("@vertex"));
        assert!(UCGEN_GOLGELENDIRICISI.contains("tepe_ana"));
        assert!(UCGEN_GOLGELENDIRICISI.contains("@fragment"));
        assert!(UCGEN_GOLGELENDIRICISI.contains("parca_ana"));
    }
}
