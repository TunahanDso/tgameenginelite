//! Tgame Engine Lite GPU grafik katmanı.

use std::sync::Arc;

use tgame_cekirdek::{Cozunurluk, OyunHatasi, OyunSonucu};
use tgame_varlik::{Dunya, Gorunum2B};
use winit::window::Window;

const UCGEN_GOLGELENDIRICISI: &str = include_str!("ucgen.wgsl");
const KAMERA_TAMPON_BOYUTU: u64 = 16;
const KAMERA_BAYT_KAPASITESI: usize = 16;
const ORNEK_ADIMI: usize = 36;
const ORNEK_ADIMI_GPU: u64 = 36;
const BASLANGIC_ORNEK_BAYT_KAPASITESI: usize = ORNEK_ADIMI * 64;
const BASLANGIC_ORNEK_TAMPON_BOYUTU: u64 = ORNEK_ADIMI_GPU * 64;
const ORNEK_NITELIKLERI: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
    0 => Float32x2,
    1 => Float32x2,
    2 => Float32,
    3 => Float32x4
];

/// Pencereye bağlı GPU yüzeyini ve toplu varlık çizim kaynaklarını yönetir.
pub struct Grafik {
    pencere: Arc<Window>,
    ornek: wgpu::Instance,
    yuzey: wgpu::Surface<'static>,
    bagdastirici: wgpu::Adapter,
    aygit: wgpu::Device,
    kuyruk: wgpu::Queue,
    yapilandirma: wgpu::SurfaceConfiguration,
    kamera_yerlesimi: wgpu::BindGroupLayout,
    kamera_tamponu: wgpu::Buffer,
    kamera_grubu: wgpu::BindGroup,
    ornek_tamponu: wgpu::Buffer,
    ornek_tampon_kapasitesi: u64,
    ornek_baytlari: Vec<u8>,
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
        let kamera_yerlesimi = kamera_yerlesimi_olustur(&aygit);
        let kamera_tamponu = aygit.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Tgame Kamera Uniform Tamponu"),
            size: KAMERA_TAMPON_BOYUTU,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let kamera_grubu = aygit.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Tgame Kamera Bağlama Grubu"),
            layout: &kamera_yerlesimi,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: kamera_tamponu.as_entire_binding(),
            }],
        });
        let ornek_tamponu = ornek_tamponu_olustur(&aygit, BASLANGIC_ORNEK_TAMPON_BOYUTU);
        let cizim_hatti = cizim_hatti_olustur(&aygit, yapilandirma.format, &kamera_yerlesimi);

        yuzey.configure(&aygit, &yapilandirma);

        Ok(Self {
            pencere,
            ornek,
            yuzey,
            bagdastirici,
            aygit,
            kuyruk,
            yapilandirma,
            kamera_yerlesimi,
            kamera_tamponu,
            kamera_grubu,
            ornek_tamponu,
            ornek_tampon_kapasitesi: BASLANGIC_ORNEK_TAMPON_BOYUTU,
            ornek_baytlari: Vec::with_capacity(BASLANGIC_ORNEK_BAYT_KAPASITESI),
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

    /// Dünyadaki etkin üçgen varlıkları tek toplu çizim çağrısıyla sunar.
    ///
    /// # Errors
    ///
    /// GPU tamponu büyütülemezse, GPU yüzeyi kaybolur ve yeniden oluşturulamazsa
    /// veya yüzey doğrulama hatası oluşursa [`OyunHatasi`] döndürür.
    pub fn ciz(&mut self, dunya: &Dunya) -> OyunSonucu {
        let ornek_sayisi = self.ornekleri_hazirla(dunya)?;
        self.kamerayi_yaz(dunya);

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
            cizim_gecisi.set_bind_group(0, &self.kamera_grubu, &[]);
            cizim_gecisi.set_vertex_buffer(0, self.ornek_tamponu.slice(..));
            cizim_gecisi.draw(0..3, 0..ornek_sayisi);
        }

        self.kuyruk.submit(Some(komut_kaydedici.finish()));
        self.kuyruk.present(kare);

        if yeniden_yapilandir {
            self.yuzeyi_yapilandir();
        }

        Ok(())
    }

    fn ornekleri_hazirla(&mut self, dunya: &Dunya) -> OyunSonucu<u32> {
        self.ornek_baytlari.clear();

        for varlik in dunya.varliklar().iter().filter(|varlik| varlik.etkin_mi()) {
            let Some(Gorunum2B::Ucgen { renk }) = varlik.gorunumu() else {
                continue;
            };
            let donusum = varlik.donusumu();
            f32_yaz(&mut self.ornek_baytlari, donusum.konum.x);
            f32_yaz(&mut self.ornek_baytlari, donusum.konum.y);
            f32_yaz(&mut self.ornek_baytlari, donusum.olcek.x);
            f32_yaz(&mut self.ornek_baytlari, donusum.olcek.y);
            f32_yaz(&mut self.ornek_baytlari, donusum.donus_radyan);
            f32_yaz(&mut self.ornek_baytlari, renk.kirmizi);
            f32_yaz(&mut self.ornek_baytlari, renk.yesil);
            f32_yaz(&mut self.ornek_baytlari, renk.mavi);
            f32_yaz(&mut self.ornek_baytlari, renk.alfa);
        }

        let gerekli_boyut = u64::try_from(self.ornek_baytlari.len())
            .map_err(|_| OyunHatasi::yeni("GPU örnek verisi desteklenen boyutu aştı."))?;
        if gerekli_boyut > self.ornek_tampon_kapasitesi {
            let yeni_kapasite = gerekli_boyut
                .checked_next_power_of_two()
                .unwrap_or(gerekli_boyut);
            self.ornek_tamponu = ornek_tamponu_olustur(&self.aygit, yeni_kapasite);
            self.ornek_tampon_kapasitesi = yeni_kapasite;
        }
        if !self.ornek_baytlari.is_empty() {
            self.kuyruk
                .write_buffer(&self.ornek_tamponu, 0, &self.ornek_baytlari);
        }

        let ornek_sayisi = self.ornek_baytlari.len() / ORNEK_ADIMI;
        u32::try_from(ornek_sayisi)
            .map_err(|_| OyunHatasi::yeni("Tek karede desteklenenden fazla varlık çiziliyor."))
    }

    fn kamerayi_yaz(&self, dunya: &Dunya) {
        let kamera = dunya.kamera();
        let en_boy_orani =
            piksel_f32(self.yapilandirma.width) / piksel_f32(self.yapilandirma.height);
        let mut baytlar = Vec::with_capacity(KAMERA_BAYT_KAPASITESI);
        f32_yaz(&mut baytlar, kamera.konum.x);
        f32_yaz(&mut baytlar, kamera.konum.y);
        f32_yaz(&mut baytlar, kamera.gorus_yuksekligi * 0.5);
        f32_yaz(&mut baytlar, en_boy_orani);
        self.kuyruk.write_buffer(&self.kamera_tamponu, 0, &baytlar);
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
            self.cizim_hatti = cizim_hatti_olustur(
                &self.aygit,
                yeni_yapilandirma.format,
                &self.kamera_yerlesimi,
            );
        }

        yeni_yuzey.configure(&self.aygit, &yeni_yapilandirma);
        self.yuzey = yeni_yuzey;
        self.yapilandirma = yeni_yapilandirma;
        Ok(())
    }
}

fn kamera_yerlesimi_olustur(aygit: &wgpu::Device) -> wgpu::BindGroupLayout {
    aygit.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Tgame Kamera Yerleşimi"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    })
}

fn ornek_tamponu_olustur(aygit: &wgpu::Device, boyut: u64) -> wgpu::Buffer {
    aygit.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Tgame Üçgen Örnek Tamponu"),
        size: boyut.max(ORNEK_ADIMI_GPU),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn cizim_hatti_olustur(
    aygit: &wgpu::Device,
    yuzey_bicimi: wgpu::TextureFormat,
    kamera_yerlesimi: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let golgelendirici = aygit.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Tgame Varlık Gölgelendiricisi"),
        source: wgpu::ShaderSource::Wgsl(UCGEN_GOLGELENDIRICISI.into()),
    });
    let cizim_hatti_yerlesimi = aygit.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Tgame Varlık Çizim Hattı Yerleşimi"),
        bind_group_layouts: &[Some(kamera_yerlesimi)],
        immediate_size: 0,
    });
    let renk_hedefleri = [Some(wgpu::ColorTargetState {
        format: yuzey_bicimi,
        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
        write_mask: wgpu::ColorWrites::ALL,
    })];
    let ornek_yerlesimi = wgpu::VertexBufferLayout {
        array_stride: ORNEK_ADIMI_GPU,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &ORNEK_NITELIKLERI,
    };

    aygit.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Tgame Toplu Üçgen Çizim Hattı"),
        layout: Some(&cizim_hatti_yerlesimi),
        vertex: wgpu::VertexState {
            module: &golgelendirici,
            entry_point: Some("tepe_ana"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[Some(ornek_yerlesimi)],
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

fn piksel_f32(deger: u32) -> f32 {
    f32::from(u16::try_from(deger).unwrap_or(u16::MAX))
}

fn f32_yaz(hedef: &mut Vec<u8>, deger: f32) {
    hedef.extend_from_slice(&deger.to_le_bytes());
}

#[cfg(test)]
mod testler {
    use super::{ORNEK_ADIMI, UCGEN_GOLGELENDIRICISI, f32_yaz, piksel_f32};

    #[test]
    fn varlik_golgelendiricisi_kamera_ve_ornek_girdilerini_icerir() {
        assert!(UCGEN_GOLGELENDIRICISI.contains("@group(0) @binding(0)"));
        assert!(UCGEN_GOLGELENDIRICISI.contains("varlik_konumu"));
        assert!(UCGEN_GOLGELENDIRICISI.contains("varlik_rengi"));
    }

    #[test]
    fn bir_ornek_dokuz_f32_degerinden_olusur() {
        let mut baytlar = Vec::new();
        for deger in 0_u8..9 {
            f32_yaz(&mut baytlar, f32::from(deger));
        }

        assert_eq!(baytlar.len(), ORNEK_ADIMI);
    }

    #[test]
    fn cok_buyuk_piksel_degeri_guvenli_sinirlanir() {
        assert_eq!(piksel_f32(u32::MAX), f32::from(u16::MAX));
    }
}
