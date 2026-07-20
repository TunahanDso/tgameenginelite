mod mesh;
mod pipeline;

use tgame_cekirdek::{Cozunurluk, OyunHatasi, OyunSonucu};
use tgame_varlik::{Dunya, Gorunum3B};

use mesh::KUP_INDEKS_SAYISI;
use pipeline::{
    cizim_hatti_olustur, derinlik_gorunumu_olustur, kamera_yerlesimi_olustur,
    mesh_tamponlari_olustur, ornek_tamponu_olustur,
};

pub(super) const DERINLIK_BICIMI: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const KAMERA_TAMPON_BOYUTU: u64 = 64;
const KAMERA_BAYT_KAPASITESI: usize = 64;
const ORNEK_ADIMI: usize = 80;
pub(super) const ORNEK_ADIMI_GPU: u64 = 80;
const BASLANGIC_ORNEK_BAYT_KAPASITESI: usize = ORNEK_ADIMI * 128;
const BASLANGIC_ORNEK_TAMPON_BOYUTU: u64 = ORNEK_ADIMI_GPU * 128;
pub(super) const ORNEK_NITELIKLERI: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
    2 => Float32x4,
    3 => Float32x4,
    4 => Float32x4,
    5 => Float32x4,
    6 => Float32x4
];

pub(super) struct UcBoyutGrafik {
    kamera_yerlesimi: wgpu::BindGroupLayout,
    kamera_tamponu: wgpu::Buffer,
    kamera_grubu: wgpu::BindGroup,
    tepe_tamponu: wgpu::Buffer,
    indeks_tamponu: wgpu::Buffer,
    ornek_tamponu: wgpu::Buffer,
    ornek_tampon_kapasitesi: u64,
    ornek_baytlari: Vec<u8>,
    cizim_hatti: wgpu::RenderPipeline,
    derinlik_gorunumu: wgpu::TextureView,
}

impl UcBoyutGrafik {
    pub(super) fn yeni(
        aygit: &wgpu::Device,
        kuyruk: &wgpu::Queue,
        yuzey_bicimi: wgpu::TextureFormat,
        boyut: Cozunurluk,
    ) -> Self {
        let kamera_yerlesimi = kamera_yerlesimi_olustur(aygit);
        let kamera_tamponu = aygit.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Tgame 3B Kamera Uniform Tamponu"),
            size: KAMERA_TAMPON_BOYUTU,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let kamera_grubu = aygit.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Tgame 3B Kamera Bağlama Grubu"),
            layout: &kamera_yerlesimi,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: kamera_tamponu.as_entire_binding(),
            }],
        });
        let mesh_tamponlari = mesh_tamponlari_olustur(aygit, kuyruk);
        let ornek_tamponu = ornek_tamponu_olustur(aygit, BASLANGIC_ORNEK_TAMPON_BOYUTU);
        let cizim_hatti = cizim_hatti_olustur(aygit, yuzey_bicimi, &kamera_yerlesimi);
        let derinlik_gorunumu = derinlik_gorunumu_olustur(aygit, boyut);

        Self {
            kamera_yerlesimi,
            kamera_tamponu,
            kamera_grubu,
            tepe_tamponu: mesh_tamponlari.tepe,
            indeks_tamponu: mesh_tamponlari.indeks,
            ornek_tamponu,
            ornek_tampon_kapasitesi: BASLANGIC_ORNEK_TAMPON_BOYUTU,
            ornek_baytlari: Vec::with_capacity(BASLANGIC_ORNEK_BAYT_KAPASITESI),
            cizim_hatti,
            derinlik_gorunumu,
        }
    }

    pub(super) fn hazirla(
        &mut self,
        aygit: &wgpu::Device,
        kuyruk: &wgpu::Queue,
        dunya: &Dunya,
        genislik: u32,
        yukseklik: u32,
    ) -> OyunSonucu<u32> {
        self.ornek_baytlari.clear();

        for varlik in dunya.varliklar().iter().filter(|varlik| varlik.etkin_mi()) {
            let Some(Gorunum3B::Kup { renk }) = varlik.gorunumu3b() else {
                continue;
            };
            for deger in varlik.donusumu3b().model_matrisi().degerler() {
                f32_yaz(&mut self.ornek_baytlari, deger);
            }
            f32_yaz(&mut self.ornek_baytlari, renk.kirmizi);
            f32_yaz(&mut self.ornek_baytlari, renk.yesil);
            f32_yaz(&mut self.ornek_baytlari, renk.mavi);
            f32_yaz(&mut self.ornek_baytlari, renk.alfa);
        }

        let gerekli_boyut = u64::try_from(self.ornek_baytlari.len())
            .map_err(|_| OyunHatasi::yeni("GPU 3B örnek verisi desteklenen boyutu aştı."))?;
        if gerekli_boyut > self.ornek_tampon_kapasitesi {
            let yeni_kapasite = gerekli_boyut
                .checked_next_power_of_two()
                .unwrap_or(gerekli_boyut);
            self.ornek_tamponu = ornek_tamponu_olustur(aygit, yeni_kapasite);
            self.ornek_tampon_kapasitesi = yeni_kapasite;
        }
        if !self.ornek_baytlari.is_empty() {
            kuyruk.write_buffer(&self.ornek_tamponu, 0, &self.ornek_baytlari);
        }

        let en_boy_orani = piksel_f32(genislik) / piksel_f32(yukseklik);
        let kamera_matrisi = dunya.kamera3b().gorunum_izdusum(en_boy_orani);
        let mut kamera_baytlari = Vec::with_capacity(KAMERA_BAYT_KAPASITESI);
        for deger in kamera_matrisi.degerler() {
            f32_yaz(&mut kamera_baytlari, deger);
        }
        kuyruk.write_buffer(&self.kamera_tamponu, 0, &kamera_baytlari);

        let ornek_sayisi = self.ornek_baytlari.len() / ORNEK_ADIMI;
        u32::try_from(ornek_sayisi)
            .map_err(|_| OyunHatasi::yeni("Tek karede desteklenenden fazla 3B varlık çiziliyor."))
    }

    pub(super) fn kaydet(
        &self,
        komut_kaydedici: &mut wgpu::CommandEncoder,
        gorunum: &wgpu::TextureView,
        ornek_sayisi: u32,
    ) {
        let renk_eklentileri = [Some(wgpu::RenderPassColorAttachment {
            view: gorunum,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.018,
                    g: 0.025,
                    b: 0.045,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        })];
        let derinlik_eklentisi = wgpu::RenderPassDepthStencilAttachment {
            view: &self.derinlik_gorunumu,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };
        let mut cizim_gecisi = komut_kaydedici.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Tgame 3B Çizim Geçişi"),
            color_attachments: &renk_eklentileri,
            depth_stencil_attachment: Some(derinlik_eklentisi),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        cizim_gecisi.set_pipeline(&self.cizim_hatti);
        cizim_gecisi.set_bind_group(0, &self.kamera_grubu, &[]);
        cizim_gecisi.set_vertex_buffer(0, self.tepe_tamponu.slice(..));
        cizim_gecisi.set_vertex_buffer(1, self.ornek_tamponu.slice(..));
        cizim_gecisi.set_index_buffer(self.indeks_tamponu.slice(..), wgpu::IndexFormat::Uint16);
        cizim_gecisi.draw_indexed(0..KUP_INDEKS_SAYISI, 0, 0..ornek_sayisi);
    }

    pub(super) fn boyutlandir(&mut self, aygit: &wgpu::Device, boyut: Cozunurluk) {
        self.derinlik_gorunumu = derinlik_gorunumu_olustur(aygit, boyut);
    }

    pub(super) fn yuzey_bicimini_degistir(
        &mut self,
        aygit: &wgpu::Device,
        yuzey_bicimi: wgpu::TextureFormat,
    ) {
        self.cizim_hatti = cizim_hatti_olustur(aygit, yuzey_bicimi, &self.kamera_yerlesimi);
    }
}

fn piksel_f32(deger: u32) -> f32 {
    f32::from(u16::try_from(deger).unwrap_or(u16::MAX))
}

fn f32_yaz(hedef: &mut Vec<u8>, deger: f32) {
    hedef.extend_from_slice(&deger.to_le_bytes());
}

#[cfg(test)]
mod testler {
    use super::ORNEK_ADIMI;

    #[test]
    fn uc_boyut_ornegi_model_matrisi_ve_renkten_olusur() {
        assert_eq!(ORNEK_ADIMI, 80);
    }
}
