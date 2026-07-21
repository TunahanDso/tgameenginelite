use tgame_model::{DokuFiltresi, DokuSarmasi, MalzemeVerisi, OrnekleyiciVerisi};

pub(super) struct GpuMalzeme {
    pub(super) grup: wgpu::BindGroup,
    _doku: wgpu::Texture,
    _gorunum: wgpu::TextureView,
    _ornekleyici: wgpu::Sampler,
}

impl GpuMalzeme {
    pub(super) fn yeni(
        aygit: &wgpu::Device,
        kuyruk: &wgpu::Queue,
        yerlesim: &wgpu::BindGroupLayout,
        malzeme: &MalzemeVerisi,
    ) -> Self {
        let beyaz = [u8::MAX; 4];
        let (genislik, yukseklik, rgba8, ornekleyici) = malzeme.temel_dokusu().map_or(
            (1, 1, beyaz.as_slice(), OrnekleyiciVerisi::default()),
            |doku| {
                (
                    doku.genislik(),
                    doku.yukseklik(),
                    doku.rgba8(),
                    doku.ornekleyici(),
                )
            },
        );
        let doku = aygit.create_texture(&wgpu::TextureDescriptor {
            label: Some("Tgame Taban Renk Dokusu"),
            size: wgpu::Extent3d {
                width: genislik,
                height: yukseklik,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        kuyruk.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &doku,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba8,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(genislik.saturating_mul(4)),
                rows_per_image: Some(yukseklik),
            },
            wgpu::Extent3d {
                width: genislik,
                height: yukseklik,
                depth_or_array_layers: 1,
            },
        );
        let gorunum = doku.create_view(&wgpu::TextureViewDescriptor::default());
        let ornekleyici = aygit.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Tgame Malzeme Örnekleyicisi"),
            address_mode_u: sarma_modu(ornekleyici.sarma_u),
            address_mode_v: sarma_modu(ornekleyici.sarma_v),
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: filtre_modu(ornekleyici.buyutme),
            min_filter: filtre_modu(ornekleyici.kucultme),
            mipmap_filter: mipmap_filtre_modu(ornekleyici.kucultme),
            ..Default::default()
        });
        let grup = aygit.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Tgame GPU Malzeme Grubu"),
            layout: yerlesim,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&gorunum),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&ornekleyici),
                },
            ],
        });

        Self {
            grup,
            _doku: doku,
            _gorunum: gorunum,
            _ornekleyici: ornekleyici,
        }
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

const fn filtre_modu(filtre: DokuFiltresi) -> wgpu::FilterMode {
    match filtre {
        DokuFiltresi::EnYakin => wgpu::FilterMode::Nearest,
        DokuFiltresi::Dogrusal => wgpu::FilterMode::Linear,
    }
}

const fn mipmap_filtre_modu(filtre: DokuFiltresi) -> wgpu::MipmapFilterMode {
    match filtre {
        DokuFiltresi::EnYakin => wgpu::MipmapFilterMode::Nearest,
        DokuFiltresi::Dogrusal => wgpu::MipmapFilterMode::Linear,
    }
}

const fn sarma_modu(sarma: DokuSarmasi) -> wgpu::AddressMode {
    match sarma {
        DokuSarmasi::KenaraSabitle => wgpu::AddressMode::ClampToEdge,
        DokuSarmasi::AynalayarakTekrarla => wgpu::AddressMode::MirrorRepeat,
        DokuSarmasi::Tekrarla => wgpu::AddressMode::Repeat,
    }
}

#[cfg(test)]
mod testler {
    use super::{filtre_modu, mipmap_filtre_modu, sarma_modu};
    use tgame_model::{DokuFiltresi, DokuSarmasi};

    #[test]
    fn sampler_degerleri_wgpuya_cevrilir() {
        assert_eq!(
            filtre_modu(DokuFiltresi::EnYakin),
            wgpu::FilterMode::Nearest
        );
        assert_eq!(
            mipmap_filtre_modu(DokuFiltresi::Dogrusal),
            wgpu::MipmapFilterMode::Linear
        );
        assert_eq!(
            sarma_modu(DokuSarmasi::AynalayarakTekrarla),
            wgpu::AddressMode::MirrorRepeat
        );
    }
}
