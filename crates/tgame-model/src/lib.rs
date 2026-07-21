//! Tgame Engine Lite genel mesh, malzeme, doku ve glTF/GLB yükleme katmanı.

use std::path::Path;

use gltf::{
    image::{Data as GltfResmi, Format as GltfResimBicimi},
    mesh::Mode,
    texture::{MagFilter, MinFilter, WrappingMode},
};
use tgame_cekirdek::{OyunHatasi, OyunSonucu};
use tgame_matematik::{Renk, Vektor2, Vektor3};

/// Doku örneklemesinde kullanılacak filtre türü.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DokuFiltresi {
    /// En yakın texel'i seçer.
    EnYakin,
    /// Komşu texel'ler arasında doğrusal geçiş yapar.
    #[default]
    Dogrusal,
}

/// UV koordinatlarının doku sınırlarının dışına çıktığında nasıl davranacağını belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DokuSarmasi {
    /// Doku kenarındaki texel'i uzatır.
    KenaraSabitle,
    /// Dokuyu aynalayarak tekrarlar.
    AynalayarakTekrarla,
    /// Dokuyu normal yönde tekrarlar.
    #[default]
    Tekrarla,
}

/// GPU sampler ayarlarının aygıttan bağımsız karşılığıdır.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OrnekleyiciVerisi {
    /// Doku büyütülürken kullanılacak filtre.
    pub buyutme: DokuFiltresi,
    /// Doku küçültülürken kullanılacak filtre.
    pub kucultme: DokuFiltresi,
    /// U eksenindeki sarma davranışı.
    pub sarma_u: DokuSarmasi,
    /// V eksenindeki sarma davranışı.
    pub sarma_v: DokuSarmasi,
}

/// GPU'ya aktarılmaya hazır RGBA8 doku verisidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DokuVerisi {
    genislik: u32,
    yukseklik: u32,
    rgba8: Vec<u8>,
    ornekleyici: OrnekleyiciVerisi,
}

impl DokuVerisi {
    /// Boyutları ve RGBA8 pikselleri doğrulanmış yeni doku oluşturur.
    ///
    /// # Errors
    ///
    /// Boyutlardan biri sıfırsa, piksel boyutu taşarsa veya bayt sayısı
    /// `genişlik × yükseklik × 4` değerine eşit değilse [`OyunHatasi`] döndürür.
    pub fn yeni_rgba8(
        genislik: u32,
        yukseklik: u32,
        rgba8: Vec<u8>,
        ornekleyici: OrnekleyiciVerisi,
    ) -> OyunSonucu<Self> {
        if genislik == 0 || yukseklik == 0 {
            return Err(OyunHatasi::yeni("Doku genişliği ve yüksekliği sıfır olamaz."));
        }
        let piksel_sayisi = genislik
            .checked_mul(yukseklik)
            .and_then(|deger| deger.checked_mul(4))
            .ok_or_else(|| OyunHatasi::yeni("Doku piksel boyutu desteklenen sınırı aştı."))?;
        let beklenen = usize::try_from(piksel_sayisi)
            .map_err(|_| OyunHatasi::yeni("Doku boyutu bu platformda temsil edilemiyor."))?;
        if rgba8.len() != beklenen {
            return Err(OyunHatasi::yeni(
                "Doku RGBA8 bayt sayısı genişlik ve yükseklikle eşleşmiyor.",
            ));
        }

        Ok(Self {
            genislik,
            yukseklik,
            rgba8,
            ornekleyici,
        })
    }

    /// Doku genişliğini döndürür.
    #[must_use]
    pub const fn genislik(&self) -> u32 {
        self.genislik
    }

    /// Doku yüksekliğini döndürür.
    #[must_use]
    pub const fn yukseklik(&self) -> u32 {
        self.yukseklik
    }

    /// RGBA8 piksel baytlarını döndürür.
    #[must_use]
    pub fn rgba8(&self) -> &[u8] {
        &self.rgba8
    }

    /// Sampler ayarlarını döndürür.
    #[must_use]
    pub const fn ornekleyici(&self) -> OrnekleyiciVerisi {
        self.ornekleyici
    }
}

/// Bir mesh'in temel renk ve isteğe bağlı taban dokusunu taşır.
#[derive(Debug, Clone, PartialEq)]
pub struct MalzemeVerisi {
    temel_renk: Renk,
    temel_doku: Option<DokuVerisi>,
}

impl MalzemeVerisi {
    /// Dokusuz yeni malzeme oluşturur.
    #[must_use]
    pub const fn yeni(temel_renk: Renk) -> Self {
        Self {
            temel_renk,
            temel_doku: None,
        }
    }

    /// Malzemeye taban renk dokusu ekler.
    #[must_use]
    pub fn temel_doku(mut self, doku: DokuVerisi) -> Self {
        self.temel_doku = Some(doku);
        self
    }

    /// Malzemenin doğrusal temel rengini döndürür.
    #[must_use]
    pub const fn temel_renk(&self) -> Renk {
        self.temel_renk
    }

    /// Malzemenin taban renk dokusunu döndürür.
    #[must_use]
    pub const fn temel_dokusu(&self) -> Option<&DokuVerisi> {
        self.temel_doku.as_ref()
    }
}

impl Default for MalzemeVerisi {
    fn default() -> Self {
        Self::yeni(Renk::BEYAZ)
    }
}

/// GPU'ya aktarılmaya hazır tek bir üçgen mesh'in ham verisidir.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeshVerisi {
    konumlar: Vec<Vektor3>,
    normaller: Vec<Vektor3>,
    uvler: Vec<Vektor2>,
    indeksler: Vec<u32>,
    malzeme: MalzemeVerisi,
}

impl MeshVerisi {
    /// Konum, normal ve üçgen indekslerinden dokusuz mesh oluşturur.
    ///
    /// UV koordinatları sıfır, malzeme beyaz olarak atanır.
    ///
    /// # Errors
    ///
    /// Mesh verisi geçersizse [`OyunHatasi`] döndürür.
    pub fn yeni(
        konumlar: Vec<Vektor3>,
        normaller: Vec<Vektor3>,
        indeksler: Vec<u32>,
    ) -> OyunSonucu<Self> {
        let uvler = vec![Vektor2::SIFIR; konumlar.len()];
        Self::yeni_malzemeli(
            konumlar,
            normaller,
            uvler,
            indeksler,
            MalzemeVerisi::default(),
        )
    }

    /// Konum, normal, UV, indeks ve malzemeden mesh oluşturur.
    ///
    /// # Errors
    ///
    /// Mesh boşsa, tepe niteliklerinin sayıları eşleşmezse, indeks sayısı üçün
    /// katı değilse veya indekslerden biri tepe sınırını aşarsa [`OyunHatasi`] döndürür.
    pub fn yeni_malzemeli(
        konumlar: Vec<Vektor3>,
        normaller: Vec<Vektor3>,
        uvler: Vec<Vektor2>,
        indeksler: Vec<u32>,
        malzeme: MalzemeVerisi,
    ) -> OyunSonucu<Self> {
        if konumlar.is_empty() {
            return Err(OyunHatasi::yeni("Mesh en az bir tepe içermeli."));
        }
        if konumlar.len() != normaller.len() || konumlar.len() != uvler.len() {
            return Err(OyunHatasi::yeni(
                "Mesh konum, normal ve UV sayıları birbiriyle eşleşmiyor.",
            ));
        }
        if indeksler.is_empty() || !indeksler.len().is_multiple_of(3) {
            return Err(OyunHatasi::yeni(
                "Mesh indeks sayısı üçgenler için sıfırdan büyük ve üçün katı olmalı.",
            ));
        }

        let tepe_sayisi = u32::try_from(konumlar.len())
            .map_err(|_| OyunHatasi::yeni("Mesh desteklenenden fazla tepe içeriyor."))?;
        if indeksler.iter().any(|indeks| *indeks >= tepe_sayisi) {
            return Err(OyunHatasi::yeni("Mesh geçersiz bir tepe indeksi içeriyor."));
        }

        Ok(Self {
            konumlar,
            normaller,
            uvler,
            indeksler,
            malzeme,
        })
    }

    /// Tepe konumlarını döndürür.
    #[must_use]
    pub fn konumlar(&self) -> &[Vektor3] {
        &self.konumlar
    }

    /// Tepe normallerini döndürür.
    #[must_use]
    pub fn normaller(&self) -> &[Vektor3] {
        &self.normaller
    }

    /// Tepe UV koordinatlarını döndürür.
    #[must_use]
    pub fn uvler(&self) -> &[Vektor2] {
        &self.uvler
    }

    /// Üçgen indekslerini döndürür.
    #[must_use]
    pub fn indeksler(&self) -> &[u32] {
        &self.indeksler
    }

    /// Mesh'in içe aktarılmış varsayılan malzemesini döndürür.
    #[must_use]
    pub const fn malzeme(&self) -> &MalzemeVerisi {
        &self.malzeme
    }

    /// Mesh'i geometri ve malzeme parçalarına ayırır.
    #[must_use]
    pub fn geometri_ve_malzemeye_ayir(mut self) -> (Self, MalzemeVerisi) {
        let malzeme = std::mem::take(&mut self.malzeme);
        (self, malzeme)
    }
}

/// Bir glTF/GLB dosyasından alınmış bir veya daha fazla üçgen mesh'i taşır.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModelVerisi {
    meshler: Vec<MeshVerisi>,
}

impl ModelVerisi {
    /// Dosya sistemindeki glTF veya GLB modelini yükler.
    ///
    /// Üçgen olmayan primitive'ler atlanır. Normal verisi bulunmayan mesh'ler için
    /// indeksli üçgenlerden yumuşatılmış tepe normalleri hesaplanır. `TEXCOORD_0`,
    /// PBR taban renk çarpanı, taban renk dokusu ve sampler ayarları içe aktarılır.
    ///
    /// # Errors
    ///
    /// Dosya okunamazsa, glTF çözümlenemezse, konum verisi yoksa, desteklenmeyen
    /// doku biçimi kullanılırsa veya mesh verisi geçersizse [`OyunHatasi`] döndürür.
    pub fn gltf_yukle(yol: impl AsRef<Path>) -> OyunSonucu<Self> {
        let yol = yol.as_ref();
        let (belge, tamponlar, resimler) = gltf::import(yol).map_err(|hata| {
            OyunHatasi::yeni(format!(
                "glTF modeli '{}' yüklenemedi: {hata}",
                yol.display()
            ))
        })?;
        let mut meshler = Vec::new();

        for mesh in belge.meshes() {
            for primitive in mesh.primitives() {
                if primitive.mode() != Mode::Triangles {
                    continue;
                }
                let okuyucu = primitive.reader(|tampon| Some(&tamponlar[tampon.index()].0));
                let konumlar = okuyucu
                    .read_positions()
                    .ok_or_else(|| OyunHatasi::yeni("glTF mesh'i tepe konumu içermiyor."))?
                    .map(|konum| Vektor3::yeni(konum[0], konum[1], konum[2]))
                    .collect::<Vec<_>>();
                let indeksler = if let Some(indeksler) = okuyucu.read_indices() {
                    indeksler.into_u32().collect::<Vec<_>>()
                } else {
                    (0..konumlar.len())
                        .map(|indeks| {
                            u32::try_from(indeks).map_err(|_| {
                                OyunHatasi::yeni("glTF mesh'i desteklenenden fazla tepe içeriyor.")
                            })
                        })
                        .collect::<OyunSonucu<Vec<_>>>()?
                };
                let normaller = okuyucu
                    .read_normals()
                    .map(|normaller| {
                        normaller
                            .map(|normal| Vektor3::yeni(normal[0], normal[1], normal[2]).birim())
                            .collect::<Vec<_>>()
                    })
                    .map_or_else(|| normalleri_hesapla(&konumlar, &indeksler), Ok)?;
                let uvler = okuyucu
                    .read_tex_coords(0)
                    .map(|uvler| {
                        uvler
                            .into_f32()
                            .map(|uv| Vektor2::yeni(uv[0], uv[1]))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_else(|| vec![Vektor2::SIFIR; konumlar.len()]);
                let malzeme = gltf_malzemesini_cevir(&primitive.material(), &resimler)?;

                meshler.push(MeshVerisi::yeni_malzemeli(
                    konumlar, normaller, uvler, indeksler, malzeme,
                )?);
            }
        }

        if meshler.is_empty() {
            return Err(OyunHatasi::yeni(
                "glTF modeli çizilebilir üçgen mesh içermiyor.",
            ));
        }

        Ok(Self { meshler })
    }

    /// Modeldeki mesh'leri döndürür.
    #[must_use]
    pub fn meshler(&self) -> &[MeshVerisi] {
        &self.meshler
    }

    /// Modelin sahip olduğu mesh'leri tüketerek döndürür.
    #[must_use]
    pub fn meshlere_ayir(self) -> Vec<MeshVerisi> {
        self.meshler
    }
}

fn gltf_malzemesini_cevir(
    malzeme: &gltf::Material<'_>,
    resimler: &[GltfResmi],
) -> OyunSonucu<MalzemeVerisi> {
    let pbr = malzeme.pbr_metallic_roughness();
    let faktor = pbr.base_color_factor();
    let mut sonuc = MalzemeVerisi::yeni(Renk::yeni(
        faktor[0], faktor[1], faktor[2], faktor[3],
    ));

    if let Some(bilgi) = pbr.base_color_texture() {
        if bilgi.tex_coord() != 0 {
            return Err(OyunHatasi::yeni(
                "glTF taban renk dokusu için yalnızca TEXCOORD_0 destekleniyor.",
            ));
        }
        let doku = bilgi.texture();
        let resim = resimler
            .get(doku.source().index())
            .ok_or_else(|| OyunHatasi::yeni("glTF malzemesinin doku resmi bulunamadı."))?;
        let ornekleyici = gltf_ornekleyicisini_cevir(doku.sampler());
        sonuc = sonuc.temel_doku(gltf_resmini_cevir(resim, ornekleyici)?);
    }

    Ok(sonuc)
}

fn gltf_ornekleyicisini_cevir(ornekleyici: gltf::texture::Sampler<'_>) -> OrnekleyiciVerisi {
    OrnekleyiciVerisi {
        buyutme: match ornekleyici.mag_filter() {
            Some(MagFilter::Nearest) => DokuFiltresi::EnYakin,
            Some(MagFilter::Linear) | None => DokuFiltresi::Dogrusal,
        },
        kucultme: match ornekleyici.min_filter() {
            Some(
                MinFilter::Nearest
                | MinFilter::NearestMipmapNearest
                | MinFilter::NearestMipmapLinear,
            ) => DokuFiltresi::EnYakin,
            Some(
                MinFilter::Linear
                | MinFilter::LinearMipmapNearest
                | MinFilter::LinearMipmapLinear,
            )
            | None => DokuFiltresi::Dogrusal,
        },
        sarma_u: gltf_sarmasini_cevir(ornekleyici.wrap_s()),
        sarma_v: gltf_sarmasini_cevir(ornekleyici.wrap_t()),
    }
}

const fn gltf_sarmasini_cevir(sarma: WrappingMode) -> DokuSarmasi {
    match sarma {
        WrappingMode::ClampToEdge => DokuSarmasi::KenaraSabitle,
        WrappingMode::MirroredRepeat => DokuSarmasi::AynalayarakTekrarla,
        WrappingMode::Repeat => DokuSarmasi::Tekrarla,
    }
}

fn gltf_resmini_cevir(
    resim: &GltfResmi,
    ornekleyici: OrnekleyiciVerisi,
) -> OyunSonucu<DokuVerisi> {
    let piksel_sayisi = resim
        .width
        .checked_mul(resim.height)
        .ok_or_else(|| OyunHatasi::yeni("glTF doku boyutu desteklenen sınırı aştı."))?;
    let kapasite = usize::try_from(
        piksel_sayisi
            .checked_mul(4)
            .ok_or_else(|| OyunHatasi::yeni("glTF RGBA doku boyutu desteklenen sınırı aştı."))?,
    )
    .map_err(|_| OyunHatasi::yeni("glTF doku boyutu bu platformda temsil edilemiyor."))?;
    let mut rgba8 = Vec::with_capacity(kapasite);

    match resim.format {
        GltfResimBicimi::R8 => {
            for &gri in &resim.pixels {
                rgba8.extend_from_slice(&[gri, gri, gri, u8::MAX]);
            }
        }
        GltfResimBicimi::R8G8 => {
            for piksel in resim.pixels.chunks_exact(2) {
                rgba8.extend_from_slice(&[piksel[0], piksel[0], piksel[0], piksel[1]]);
            }
        }
        GltfResimBicimi::R8G8B8 => {
            for piksel in resim.pixels.chunks_exact(3) {
                rgba8.extend_from_slice(&[piksel[0], piksel[1], piksel[2], u8::MAX]);
            }
        }
        GltfResimBicimi::R8G8B8A8 => rgba8.extend_from_slice(&resim.pixels),
        GltfResimBicimi::R16
        | GltfResimBicimi::R16G16
        | GltfResimBicimi::R16G16B16
        | GltfResimBicimi::R16G16B16A16
        | GltfResimBicimi::R32G32B32FLOAT
        | GltfResimBicimi::R32G32B32A32FLOAT => {
            return Err(OyunHatasi::yeni(
                "glTF dokularında şimdilik yalnızca 8 bit kanal biçimleri destekleniyor.",
            ));
        }
    }

    DokuVerisi::yeni_rgba8(resim.width, resim.height, rgba8, ornekleyici)
}

fn normalleri_hesapla(konumlar: &[Vektor3], indeksler: &[u32]) -> OyunSonucu<Vec<Vektor3>> {
    if indeksler.is_empty() || !indeksler.len().is_multiple_of(3) {
        return Err(OyunHatasi::yeni(
            "Mesh indeks sayısı üçgenler için sıfırdan büyük ve üçün katı olmalı.",
        ));
    }

    let mut normaller = vec![Vektor3::SIFIR; konumlar.len()];
    for ucgen in indeksler.chunks_exact(3) {
        let a = indeks_usize(ucgen[0], konumlar.len())?;
        let b = indeks_usize(ucgen[1], konumlar.len())?;
        let c = indeks_usize(ucgen[2], konumlar.len())?;
        let normal = (konumlar[b] - konumlar[a])
            .capraz(konumlar[c] - konumlar[a])
            .birim();
        normaller[a] += normal;
        normaller[b] += normal;
        normaller[c] += normal;
    }

    for normal in &mut normaller {
        *normal = normal.birim();
    }
    Ok(normaller)
}

fn indeks_usize(indeks: u32, tepe_sayisi: usize) -> OyunSonucu<usize> {
    let indeks = usize::try_from(indeks)
        .map_err(|_| OyunHatasi::yeni("Mesh indeksi bu platformda temsil edilemiyor."))?;
    if indeks >= tepe_sayisi {
        return Err(OyunHatasi::yeni("Mesh indeksi tepe sınırını aşıyor."));
    }
    Ok(indeks)
}

#[cfg(test)]
mod testler {
    use tgame_matematik::{Renk, Vektor2, Vektor3};

    use super::{
        DokuFiltresi, DokuSarmasi, DokuVerisi, MalzemeVerisi, MeshVerisi, OrnekleyiciVerisi,
    };

    #[test]
    fn gecersiz_mesh_reddedilir() {
        let sonuc = MeshVerisi::yeni(vec![Vektor3::SIFIR], vec![Vektor3::YUKARI], vec![0, 1, 2]);

        assert!(sonuc.is_err());
    }

    #[test]
    fn dokulu_ucgen_mesh_kabul_edilir() {
        let doku = DokuVerisi::yeni_rgba8(
            1,
            1,
            vec![255, 128, 64, 255],
            OrnekleyiciVerisi {
                buyutme: DokuFiltresi::EnYakin,
                kucultme: DokuFiltresi::EnYakin,
                sarma_u: DokuSarmasi::KenaraSabitle,
                sarma_v: DokuSarmasi::KenaraSabitle,
            },
        )
        .expect("Geçerli bir texel kabul edilmeli.");
        let sonuc = MeshVerisi::yeni_malzemeli(
            vec![Vektor3::SIFIR, Vektor3::SAG, Vektor3::YUKARI],
            vec![Vektor3::ILERI; 3],
            vec![Vektor2::SIFIR, Vektor2::SAG, Vektor2::YUKARI],
            vec![0, 1, 2],
            MalzemeVerisi::yeni(Renk::BEYAZ).temel_doku(doku),
        );

        assert!(sonuc.is_ok());
    }

    #[test]
    fn gecersiz_doku_bayt_sayisi_reddedilir() {
        let sonuc = DokuVerisi::yeni_rgba8(2, 2, vec![255; 15], OrnekleyiciVerisi::default());

        assert!(sonuc.is_err());
    }
}
