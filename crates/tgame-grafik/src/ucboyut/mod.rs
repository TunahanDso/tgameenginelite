mod gpu_doku;
mod gpu_malzeme;
mod gpu_mesh;
mod mesh;
mod pipeline;

use std::{collections::BTreeMap, ops::Range};

use tgame_cekirdek::{Cozunurluk, OyunHatasi, OyunSonucu};
use tgame_matematik::{Renk, Vektor3};
use tgame_model::SinirKuresi;
use tgame_varlik::{
    Dunya, Gorunum3B, MalzemeKimligi, MeshKimligi, Varlik,
};

use gpu_doku::GpuDoku;
use gpu_malzeme::{GpuMalzeme, malzeme_yerlesimi_olustur};
use gpu_mesh::GpuMesh;
use pipeline::{
    cizim_hatti_olustur, derinlik_gorunumu_olustur, kamera_yerlesimi_olustur,
    ornek_tamponu_olustur,
};

pub(super) const DERINLIK_BICIMI: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const KAMERA_TAMPON_BOYUTU: u64 = 64;
const KAMERA_BAYT_KAPASITESI: usize = 64;
const ORNEK_ADIMI: usize = 80;
pub(super) const ORNEK_ADIMI_GPU: u64 = 80;
const BASLANGIC_ORNEK_BAYT_KAPASITESI: usize = ORNEK_ADIMI * 128;
const BASLANGIC_ORNEK_TAMPON_BOYUTU: u64 = ORNEK_ADIMI_GPU * 128;
const KUP_SINIR_YARICAPI: f32 = 0.866_025_4;
pub(super) const ORNEK_NITELIKLERI: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
    2 => Float32x4,
    3 => Float32x4,
    4 => Float32x4,
    5 => Float32x4,
    6 => Float32x4
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MeshAnahtari {
    Kup,
    Kayitli(MeshKimligi),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MalzemeAnahtari {
    Kup,
    Kayitli(MalzemeKimligi),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CizimAnahtari {
    malzeme: MalzemeAnahtari,
    mesh: MeshAnahtari,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CizimGrubu {
    anahtar: CizimAnahtari,
    ornekler: Range<u32>,
}

pub(super) struct UcBoyutGrafik {
    kamera_yerlesimi: wgpu::BindGroupLayout,
    malzeme_yerlesimi: wgpu::BindGroupLayout,
    kamera_tamponu: wgpu::Buffer,
    kamera_grubu: wgpu::BindGroup,
    _beyaz_doku: GpuDoku,
    kup_malzeme: GpuMalzeme,
    kup_mesh: GpuMesh,
    kayitli_dokular: Vec<GpuDoku>,
    kayitli_malzemeler: Vec<GpuMalzeme>,
    kayitli_meshler: Vec<GpuMesh>,
    ornek_tamponu: wgpu::Buffer,
    ornek_tampon_kapasitesi: u64,
    ornek_baytlari: Vec<u8>,
    cizim_gruplari: Vec<CizimGrubu>,
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
        let malzeme_yerlesimi = malzeme_yerlesimi_olustur(aygit);
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
        let beyaz_doku = GpuDoku::beyaz(aygit, kuyruk);
        let kup_malzeme = GpuMalzeme::yeni(aygit, &malzeme_yerlesimi, &beyaz_doku);
        let kup_mesh = GpuMesh::kup(aygit, kuyruk);
        let ornek_tamponu = ornek_tamponu_olustur(aygit, BASLANGIC_ORNEK_TAMPON_BOYUTU);
        let cizim_hatti =
            cizim_hatti_olustur(aygit, yuzey_bicimi, &kamera_yerlesimi, &malzeme_yerlesimi);
        let derinlik_gorunumu = derinlik_gorunumu_olustur(aygit, boyut);

        Self {
            kamera_yerlesimi,
            malzeme_yerlesimi,
            kamera_tamponu,
            kamera_grubu,
            _beyaz_doku: beyaz_doku,
            kup_malzeme,
            kup_mesh,
            kayitli_dokular: Vec::new(),
            kayitli_malzemeler: Vec::new(),
            kayitli_meshler: Vec::new(),
            ornek_tamponu,
            ornek_tampon_kapasitesi: BASLANGIC_ORNEK_TAMPON_BOYUTU,
            ornek_baytlari: Vec::with_capacity(BASLANGIC_ORNEK_BAYT_KAPASITESI),
            cizim_gruplari: Vec::new(),
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
    ) -> OyunSonucu {
        self.kaynaklari_esitle(aygit, kuyruk, dunya)?;
        let en_boy_orani = piksel_f32(genislik) / piksel_f32(yukseklik);
        self.ornekleri_hazirla(dunya, en_boy_orani)?;
        self.ornek_tamponunu_yaz(aygit, kuyruk)?;
        self.kamerayi_yaz(kuyruk, dunya, en_boy_orani);
        Ok(())
    }

    fn kaynaklari_esitle(
        &mut self,
        aygit: &wgpu::Device,
        kuyruk: &wgpu::Queue,
        dunya: &Dunya,
    ) -> OyunSonucu {
        if self.kayitli_dokular.len() > dunya.dokular().len()
            || self.kayitli_malzemeler.len() > dunya.malzemeler().len()
            || self.kayitli_meshler.len() > dunya.meshler().len()
        {
            self.kayitli_dokular.clear();
            self.kayitli_malzemeler.clear();
            self.kayitli_meshler.clear();
        }

        for doku in &dunya.dokular()[self.kayitli_dokular.len()..] {
            self.kayitli_dokular.push(GpuDoku::yeni(aygit, kuyruk, doku));
        }
        for malzeme in &dunya.malzemeler()[self.kayitli_malzemeler.len()..] {
            let doku = if let Some(kimlik) = malzeme.temel_doku() {
                self.kayitli_dokular.get(kimlik.deger()).ok_or_else(|| {
                    OyunHatasi::yeni(
                        "GPU malzemesi, kayıt defterinde bulunmayan bir doku kullanıyor.",
                    )
                })?
            } else {
                &self._beyaz_doku
            };
            self.kayitli_malzemeler
                .push(GpuMalzeme::yeni(aygit, &self.malzeme_yerlesimi, doku));
        }
        for mesh in &dunya.meshler()[self.kayitli_meshler.len()..] {
            self.kayitli_meshler
                .push(GpuMesh::kayitli(aygit, kuyruk, mesh)?);
        }
        Ok(())
    }

    fn ornekleri_hazirla(&mut self, dunya: &Dunya, en_boy_orani: f32) -> OyunSonucu {
        let mut kumeler = BTreeMap::<CizimAnahtari, Vec<(&Varlik, Renk)>>::new();
        for varlik in dunya.varliklar().iter().filter(|varlik| varlik.etkin_mi()) {
            let Some((anahtar, renk, sinir_kuresi)) = gorunum_bilgisi(varlik, dunya)? else {
                continue;
            };
            self.cizim_kaynagini_dogrula(anahtar)?;
            let dunya_kuresi = sinir_kuresi.donustur(varlik.model_matrisi());
            if !varlik.gorunurluk_kirpmasini_atlar_mi()
                && !dunya.kamera3b().kure_gorunur_mu(
                    dunya_kuresi.merkez(),
                    dunya_kuresi.yaricap(),
                    en_boy_orani,
                )
            {
                continue;
            }
            kumeler.entry(anahtar).or_default().push((varlik, renk));
        }

        self.ornek_baytlari.clear();
        self.cizim_gruplari.clear();
        for (anahtar, varliklar) in kumeler {
            let baslangic = ornek_sayisini_cevir(self.ornek_baytlari.len() / ORNEK_ADIMI)?;
            for (varlik, renk) in &varliklar {
                ornegi_yaz(&mut self.ornek_baytlari, varlik, *renk);
            }
            let sayi = ornek_sayisini_cevir(varliklar.len())?;
            let son = baslangic.checked_add(sayi).ok_or_else(|| {
                OyunHatasi::yeni("3B çizim grubunun örnek aralığı desteklenen sınırı aştı.")
            })?;
            self.cizim_gruplari.push(CizimGrubu {
                anahtar,
                ornekler: baslangic..son,
            });
        }
        Ok(())
    }

    fn cizim_kaynagini_dogrula(&self, anahtar: CizimAnahtari) -> OyunSonucu {
        if let MeshAnahtari::Kayitli(kimlik) = anahtar.mesh {
            if kimlik.deger() >= self.kayitli_meshler.len() {
                return Err(OyunHatasi::yeni(
                    "3B varlık, GPU kayıt defterinde bulunmayan bir mesh kullanıyor.",
                ));
            }
        }
        if let MalzemeAnahtari::Kayitli(kimlik) = anahtar.malzeme {
            if kimlik.deger() >= self.kayitli_malzemeler.len() {
                return Err(OyunHatasi::yeni(
                    "3B varlık, GPU kayıt defterinde bulunmayan bir malzeme kullanıyor.",
                ));
            }
        }
        Ok(())
    }

    fn ornek_tamponunu_yaz(&mut self, aygit: &wgpu::Device, kuyruk: &wgpu::Queue) -> OyunSonucu {
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
        Ok(())
    }

    fn kamerayi_yaz(&self, kuyruk: &wgpu::Queue, dunya: &Dunya, en_boy_orani: f32) {
        let kamera_matrisi = dunya.kamera3b().gorunum_izdusum(en_boy_orani);
        let mut kamera_baytlari = Vec::with_capacity(KAMERA_BAYT_KAPASITESI);
        for deger in kamera_matrisi.degerler() {
            f32_yaz(&mut kamera_baytlari, deger);
        }
        kuyruk.write_buffer(&self.kamera_tamponu, 0, &kamera_baytlari);
    }

    pub(super) fn kaydet(
        &self,
        komut_kaydedici: &mut wgpu::CommandEncoder,
        gorunum: &wgpu::TextureView,
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
            label: Some("Tgame 3B Dokulu Mesh Çizim Geçişi"),
            color_attachments: &renk_eklentileri,
            depth_stencil_attachment: Some(derinlik_eklentisi),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        cizim_gecisi.set_pipeline(&self.cizim_hatti);
        cizim_gecisi.set_bind_group(0, &self.kamera_grubu, &[]);
        cizim_gecisi.set_vertex_buffer(1, self.ornek_tamponu.slice(..));

        let mut onceki_malzeme = None;
        for grup in &self.cizim_gruplari {
            if onceki_malzeme != Some(grup.anahtar.malzeme) {
                cizim_gecisi.set_bind_group(
                    1,
                    &self.gpu_malzeme(grup.anahtar.malzeme).grup,
                    &[],
                );
                onceki_malzeme = Some(grup.anahtar.malzeme);
            }
            let mesh = self.gpu_mesh(grup.anahtar.mesh);
            cizim_gecisi.set_vertex_buffer(0, mesh.tepe.slice(..));
            cizim_gecisi.set_index_buffer(mesh.indeks.slice(..), mesh.indeks_bicimi);
            cizim_gecisi.draw_indexed(0..mesh.indeks_sayisi, 0, grup.ornekler.clone());
        }
    }

    fn gpu_mesh(&self, anahtar: MeshAnahtari) -> &GpuMesh {
        match anahtar {
            MeshAnahtari::Kup => &self.kup_mesh,
            MeshAnahtari::Kayitli(kimlik) => &self.kayitli_meshler[kimlik.deger()],
        }
    }

    fn gpu_malzeme(&self, anahtar: MalzemeAnahtari) -> &GpuMalzeme {
        match anahtar {
            MalzemeAnahtari::Kup => &self.kup_malzeme,
            MalzemeAnahtari::Kayitli(kimlik) => &self.kayitli_malzemeler[kimlik.deger()],
        }
    }

    pub(super) fn boyutlandir(&mut self, aygit: &wgpu::Device, boyut: Cozunurluk) {
        self.derinlik_gorunumu = derinlik_gorunumu_olustur(aygit, boyut);
    }

    pub(super) fn yuzey_bicimini_degistir(
        &mut self,
        aygit: &wgpu::Device,
        yuzey_bicimi: wgpu::TextureFormat,
    ) {
        self.cizim_hatti = cizim_hatti_olustur(
            aygit,
            yuzey_bicimi,
            &self.kamera_yerlesimi,
            &self.malzeme_yerlesimi,
        );
    }
}

fn gorunum_bilgisi(
    varlik: &Varlik,
    dunya: &Dunya,
) -> OyunSonucu<Option<(CizimAnahtari, Renk, SinirKuresi)>> {
    match varlik.gorunumu3b() {
        Some(Gorunum3B::Kup { renk }) => Ok(Some((
            CizimAnahtari {
                malzeme: MalzemeAnahtari::Kup,
                mesh: MeshAnahtari::Kup,
            },
            renk,
            SinirKuresi::yeni(Vektor3::SIFIR, KUP_SINIR_YARICAPI),
        ))),
        Some(Gorunum3B::Mesh {
            mesh,
            malzeme,
            renk,
        }) => {
            let kaynak = dunya.mesh(mesh).ok_or_else(|| {
                OyunHatasi::yeni(
                    "3B varlık, dünya kayıt defterinde bulunmayan bir mesh kullanıyor.",
                )
            })?;
            let malzeme = malzeme
                .or_else(|| dunya.mesh_malzemesi(mesh))
                .ok_or_else(|| OyunHatasi::yeni("Mesh için varsayılan malzeme bulunamadı."))?;
            let malzeme_kaydi = dunya.malzeme(malzeme).ok_or_else(|| {
                OyunHatasi::yeni(
                    "3B varlık, dünya kayıt defterinde bulunmayan bir malzeme kullanıyor.",
                )
            })?;
            Ok(Some((
                CizimAnahtari {
                    malzeme: MalzemeAnahtari::Kayitli(malzeme),
                    mesh: MeshAnahtari::Kayitli(mesh),
                },
                renkleri_carp(renk, malzeme_kaydi.temel_renk()),
                kaynak.sinir_kuresi(),
            )))
        }
        None => Ok(None),
    }
}

const fn renkleri_carp(sol: Renk, sag: Renk) -> Renk {
    Renk::yeni(
        sol.kirmizi * sag.kirmizi,
        sol.yesil * sag.yesil,
        sol.mavi * sag.mavi,
        sol.alfa * sag.alfa,
    )
}

fn ornegi_yaz(hedef: &mut Vec<u8>, varlik: &Varlik, renk: Renk) {
    for deger in varlik.model_matrisi().degerler() {
        f32_yaz(hedef, deger);
    }
    for deger in [renk.kirmizi, renk.yesil, renk.mavi, renk.alfa] {
        f32_yaz(hedef, deger);
    }
}

fn ornek_sayisini_cevir(sayi: usize) -> OyunSonucu<u32> {
    u32::try_from(sayi)
        .map_err(|_| OyunHatasi::yeni("Tek karede desteklenenden fazla 3B varlık çiziliyor."))
}

fn piksel_f32(deger: u32) -> f32 {
    f32::from(u16::try_from(deger).unwrap_or(u16::MAX))
}

fn f32_yaz(hedef: &mut Vec<u8>, deger: f32) {
    hedef.extend_from_slice(&deger.to_le_bytes());
}

#[cfg(test)]
mod testler {
    use tgame_matematik::Renk;

    use super::{KUP_SINIR_YARICAPI, ORNEK_ADIMI, renkleri_carp};

    #[test]
    fn uc_boyut_ornegi_model_matrisi_ve_renkten_olusur() {
        assert_eq!(ORNEK_ADIMI, 80);
        assert!(KUP_SINIR_YARICAPI > 0.86);
    }

    #[test]
    fn varlik_ve_malzeme_renkleri_carpilir() {
        let renk = renkleri_carp(
            Renk::yeni(0.5, 1.0, 0.25, 0.8),
            Renk::yeni(0.4, 0.5, 1.0, 0.5),
        );

        assert!((renk.kirmizi - 0.2).abs() < 0.000_01);
        assert!((renk.yesil - 0.5).abs() < 0.000_01);
        assert!((renk.mavi - 0.25).abs() < 0.000_01);
        assert!((renk.alfa - 0.4).abs() < 0.000_01);
    }
}
