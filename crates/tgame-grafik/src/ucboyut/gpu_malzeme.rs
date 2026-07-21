use super::gpu_doku::GpuDoku;

pub(super) struct GpuMalzeme {
    pub(super) grup: wgpu::BindGroup,
}

impl GpuMalzeme {
    pub(super) fn yeni(
        aygit: &wgpu::Device,
        yerlesim: &wgpu::BindGroupLayout,
        doku: &GpuDoku,
    ) -> Self {
        let grup = aygit.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Tgame GPU Malzeme Grubu"),
            layout: yerlesim,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&doku.gorunum),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&doku.ornekleyici),
                },
            ],
        });
        Self { grup }
    }
}

pub(super) fn malzeme_yerlesimi_olustur(aygit: &wgpu::Device) -> wgpu::BindGroupLayout {
    aygit.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Tgame 3B Malzeme Yerleşimi"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}
