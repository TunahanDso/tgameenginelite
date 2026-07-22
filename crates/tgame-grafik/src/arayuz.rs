use glyphon::{
    Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache,
    TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use tgame_arayuz::Arayuz;
use tgame_cekirdek::{OyunHatasi, OyunSonucu};
use wgpu::util::DeviceExt;

const PANEL_ORNEK_BOYUTU: wgpu::BufferAddress = 32;

pub(super) struct ArayuzGrafik {
    panel_pipeline: wgpu::RenderPipeline,
    panel_tamponu: Option<wgpu::Buffer>,
    panel_sayisi: u32,
    font_sistemi: FontSystem,
    swash_onbellegi: SwashCache,
    metin_onbellegi: Cache,
    metin_atlasi: TextAtlas,
    gorus_alani: Viewport,
    metin_cizici: TextRenderer,
}

impl ArayuzGrafik {
    pub(super) fn yeni(
        aygit: &wgpu::Device,
        kuyruk: &wgpu::Queue,
        yuzey_bicimi: wgpu::TextureFormat,
    ) -> Self {
        let golgelendirici = aygit.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Tgame Arayüz Panel Shaderı"),
            source: wgpu::ShaderSource::Wgsl(include_str!("arayuz.wgsl").into()),
        });
        let pipeline_yerlesimi = aygit.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Tgame Arayüz Panel Pipeline Yerleşimi"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let panel_pipeline = aygit.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Tgame Arayüz Panel Pipeline"),
            layout: Some(&pipeline_yerlesimi),
            vertex: wgpu::VertexState {
                module: &golgelendirici,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: PANEL_ORNEK_BOYUTU,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 16,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &golgelendirici,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: yuzey_bicimi,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let metin_onbellegi = Cache::new(aygit);
        let mut metin_atlasi = TextAtlas::new(aygit, kuyruk, &metin_onbellegi, yuzey_bicimi);
        let gorus_alani = Viewport::new(aygit, &metin_onbellegi);
        let metin_cizici = TextRenderer::new(
            &mut metin_atlasi,
            aygit,
            wgpu::MultisampleState::default(),
            None,
        );

        Self {
            panel_pipeline,
            panel_tamponu: None,
            panel_sayisi: 0,
            font_sistemi: FontSystem::new(),
            swash_onbellegi: SwashCache::new(),
            metin_onbellegi,
            metin_atlasi,
            gorus_alani,
            metin_cizici,
        }
    }

    pub(super) fn hazirla(
        &mut self,
        aygit: &wgpu::Device,
        kuyruk: &wgpu::Queue,
        arayuz: &Arayuz,
        genislik: u32,
        yukseklik: u32,
    ) -> OyunSonucu {
        self.panel_sayisi = u32::try_from(arayuz.paneller().len())
            .map_err(|_| OyunHatasi::yeni("Arayüz panel sayısı desteklenen sınırı aştı."))?;
        self.panel_tamponu = if arayuz.paneller().is_empty() {
            None
        } else {
            let mut ham = Vec::with_capacity(arayuz.paneller().len() * 32);
            let ekran_genisligi = genislik.max(1) as f32;
            let ekran_yuksekligi = yukseklik.max(1) as f32;
            for panel in arayuz.paneller() {
                let alan = panel.alan();
                let renk = panel.renk();
                let degerler = [
                    alan.sol * 2.0 / ekran_genisligi - 1.0,
                    1.0 - alan.ust * 2.0 / ekran_yuksekligi,
                    alan.genislik * 2.0 / ekran_genisligi,
                    alan.yukseklik * 2.0 / ekran_yuksekligi,
                    renk.kirmizi,
                    renk.yesil,
                    renk.mavi,
                    renk.alfa,
                ];
                ham.extend(degerler.iter().flat_map(|deger| deger.to_ne_bytes()));
            }
            Some(aygit.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Tgame Arayüz Panel Örnek Tamponu"),
                contents: &ham,
                usage: wgpu::BufferUsages::VERTEX,
            }))
        };

        self.gorus_alani.update(
            kuyruk,
            Resolution {
                width: genislik.max(1),
                height: yukseklik.max(1),
            },
        );

        let mut tamponlar = Vec::with_capacity(arayuz.metinler().len());
        for metin in arayuz.metinler() {
            let alan = metin.alan();
            let mut tampon = Buffer::new(
                &mut self.font_sistemi,
                Metrics::new(metin.punto(), metin.satir_yuksekligi_degeri()),
            );
            tampon.set_size(Some(alan.genislik.max(1.0)), Some(alan.yukseklik.max(1.0)));
            tampon.set_text(
                metin.metin(),
                &Attrs::new().family(Family::SansSerif),
                Shaping::Advanced,
                None,
            );
            tampon.shape_until_scroll(&mut self.font_sistemi, false);
            tamponlar.push(tampon);
        }

        let alanlar = arayuz
            .metinler()
            .iter()
            .zip(&tamponlar)
            .map(|(metin, tampon)| {
                let alan = metin.alan();
                let renk = metin.renk_degeri();
                TextArea {
                    buffer: tampon,
                    left: alan.sol,
                    top: alan.ust,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: alan.sol.max(0.0) as i32,
                        top: alan.ust.max(0.0) as i32,
                        right: (alan.sol + alan.genislik).min(genislik as f32) as i32,
                        bottom: (alan.ust + alan.yukseklik).min(yukseklik as f32) as i32,
                    },
                    default_color: Color::rgba(
                        renk.kirmizi_8(),
                        renk.yesil_8(),
                        renk.mavi_8(),
                        renk.alfa_8(),
                    ),
                    angle: 0.0,
                    rotation_origin: None,
                }
            });
        self.metin_cizici
            .prepare(
                aygit,
                kuyruk,
                &mut self.font_sistemi,
                &mut self.metin_atlasi,
                &self.gorus_alani,
                alanlar,
                &mut self.swash_onbellegi,
            )
            .map_err(|hata| OyunHatasi::yeni(format!("Ekran metni hazırlanamadı: {hata}")))?;
        Ok(())
    }

    pub(super) fn kaydet(
        &self,
        komut_kaydedici: &mut wgpu::CommandEncoder,
        gorunum: &wgpu::TextureView,
    ) -> OyunSonucu {
        let mut gecis = komut_kaydedici.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Tgame Arayüz Geçişi"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: gorunum,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        if let Some(tampon) = &self.panel_tamponu {
            gecis.set_pipeline(&self.panel_pipeline);
            gecis.set_vertex_buffer(0, tampon.slice(..));
            gecis.draw(0..6, 0..self.panel_sayisi);
        }
        self.metin_cizici
            .render(&self.metin_atlasi, &self.gorus_alani, &mut gecis)
            .map_err(|hata| OyunHatasi::yeni(format!("Ekran metni çizilemedi: {hata}")))?;
        Ok(())
    }

    pub(super) fn yuzey_bicimini_degistir(
        &mut self,
        aygit: &wgpu::Device,
        kuyruk: &wgpu::Queue,
        yuzey_bicimi: wgpu::TextureFormat,
    ) {
        *self = Self::yeni(aygit, kuyruk, yuzey_bicimi);
    }
}
