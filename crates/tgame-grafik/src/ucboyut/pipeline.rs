use tgame_cekirdek::Cozunurluk;

use super::mesh::{
    indeks_baytlari, tepe_baytlari, KUP_INDEKS_TAMPON_BOYUTU, KUP_TEPE_TAMPON_BOYUTU,
    TEPE_ADIMI_GPU, TEPE_NITELIKLERI,
};
use super::{DERINLIK_BICIMI, ORNEK_ADIMI_GPU, ORNEK_NITELIKLERI};

const KUP_GOLGELENDIRICISI: &str = include_str!("../kup.wgsl");

pub(super) struct MeshTamponlari {
    pub(super) tepe: wgpu::Buffer,
    pub(super) indeks: wgpu::Buffer,
}

pub(super) fn kamera_yerlesimi_olustur(aygit: &wgpu::Device) -> wgpu::BindGroupLayout {
    aygit.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Tgame 3B Kamera Yerleşimi"),
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

pub(super) fn mesh_tamponlari_olustur(
    aygit: &wgpu::Device,
    kuyruk: &wgpu::Queue,
) -> MeshTamponlari {
    let tepe_baytlari = tepe_baytlari();
    let tepe = aygit.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Tgame 3B Küp Tepe Tamponu"),
        size: KUP_TEPE_TAMPON_BOYUTU,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    kuyruk.write_buffer(&tepe, 0, &tepe_baytlari);

    let indeks_baytlari = indeks_baytlari();
    let indeks = aygit.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Tgame 3B Küp İndeks Tamponu"),
        size: KUP_INDEKS_TAMPON_BOYUTU,
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    kuyruk.write_buffer(&indeks, 0, &indeks_baytlari);

    MeshTamponlari { tepe, indeks }
}

pub(super) fn ornek_tamponu_olustur(aygit: &wgpu::Device, boyut: u64) -> wgpu::Buffer {
    aygit.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Tgame 3B Küp Örnek Tamponu"),
        size: boyut.max(ORNEK_ADIMI_GPU),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

pub(super) fn derinlik_gorunumu_olustur(
    aygit: &wgpu::Device,
    boyut: Cozunurluk,
) -> wgpu::TextureView {
    let doku = aygit.create_texture(&wgpu::TextureDescriptor {
        label: Some("Tgame 3B Derinlik Dokusu"),
        size: wgpu::Extent3d {
            width: boyut.genislik.max(1),
            height: boyut.yukseklik.max(1),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DERINLIK_BICIMI,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    doku.create_view(&wgpu::TextureViewDescriptor::default())
}

pub(super) fn cizim_hatti_olustur(
    aygit: &wgpu::Device,
    yuzey_bicimi: wgpu::TextureFormat,
    kamera_yerlesimi: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let golgelendirici = aygit.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Tgame 3B Küp Gölgelendiricisi"),
        source: wgpu::ShaderSource::Wgsl(KUP_GOLGELENDIRICISI.into()),
    });
    let cizim_hatti_yerlesimi = aygit.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Tgame 3B Çizim Hattı Yerleşimi"),
        bind_group_layouts: &[Some(kamera_yerlesimi)],
        immediate_size: 0,
    });
    let renk_hedefleri = [Some(wgpu::ColorTargetState {
        format: yuzey_bicimi,
        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
        write_mask: wgpu::ColorWrites::ALL,
    })];
    let tepe_yerlesimi = wgpu::VertexBufferLayout {
        array_stride: TEPE_ADIMI_GPU,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &TEPE_NITELIKLERI,
    };
    let ornek_yerlesimi = wgpu::VertexBufferLayout {
        array_stride: ORNEK_ADIMI_GPU,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &ORNEK_NITELIKLERI,
    };

    aygit.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Tgame Derinlikli Toplu Küp Çizim Hattı"),
        layout: Some(&cizim_hatti_yerlesimi),
        vertex: wgpu::VertexState {
            module: &golgelendirici,
            entry_point: Some("tepe_ana"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[Some(tepe_yerlesimi), Some(ornek_yerlesimi)],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DERINLIK_BICIMI,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Less),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
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
    use super::KUP_GOLGELENDIRICISI;

    #[test]
    fn kup_golgelendiricisi_perspektif_ve_aydinlatma_icerir() {
        assert!(KUP_GOLGELENDIRICISI.contains("gorunum_izdusum"));
        assert!(KUP_GOLGELENDIRICISI.contains("isik_yonu"));
    }
}
