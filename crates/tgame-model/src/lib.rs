//! Tgame Engine Lite genel mesh ve glTF/GLB model yükleme katmanı.

use std::path::Path;

use gltf::mesh::Mode;
use tgame_cekirdek::{OyunHatasi, OyunSonucu};
use tgame_matematik::Vektor3;

/// GPU'ya aktarılmaya hazır tek bir üçgen mesh'in ham verisidir.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeshVerisi {
    konumlar: Vec<Vektor3>,
    normaller: Vec<Vektor3>,
    indeksler: Vec<u32>,
}

impl MeshVerisi {
    /// Konum, normal ve üçgen indekslerinden mesh oluşturur.
    ///
    /// # Errors
    ///
    /// Mesh boşsa, konum ve normal sayıları eşleşmezse, indeks sayısı üçün katı
    /// değilse veya indekslerden biri tepe sınırını aşarsa [`OyunHatasi`] döndürür.
    pub fn yeni(
        konumlar: Vec<Vektor3>,
        normaller: Vec<Vektor3>,
        indeksler: Vec<u32>,
    ) -> OyunSonucu<Self> {
        if konumlar.is_empty() {
            return Err(OyunHatasi::yeni("Mesh en az bir tepe içermeli."));
        }
        if konumlar.len() != normaller.len() {
            return Err(OyunHatasi::yeni(
                "Mesh konum ve normal sayıları birbiriyle eşleşmiyor.",
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
            indeksler,
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

    /// Üçgen indekslerini döndürür.
    #[must_use]
    pub fn indeksler(&self) -> &[u32] {
        &self.indeksler
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
    /// indeksli üçgenlerden yumuşatılmış tepe normalleri hesaplanır.
    ///
    /// # Errors
    ///
    /// Dosya okunamazsa, glTF çözümlenemezse, konum verisi yoksa veya mesh verisi
    /// geçersizse [`OyunHatasi`] döndürür.
    pub fn gltf_yukle(yol: impl AsRef<Path>) -> OyunSonucu<Self> {
        let yol = yol.as_ref();
        let (belge, tamponlar, _resimler) = gltf::import(yol).map_err(|hata| {
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

                meshler.push(MeshVerisi::yeni(konumlar, normaller, indeksler)?);
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
    use tgame_matematik::Vektor3;

    use super::MeshVerisi;

    #[test]
    fn gecersiz_mesh_reddedilir() {
        let sonuc = MeshVerisi::yeni(vec![Vektor3::SIFIR], vec![Vektor3::YUKARI], vec![0, 1, 2]);

        assert!(sonuc.is_err());
    }

    #[test]
    fn ucgen_mesh_kabul_edilir() {
        let sonuc = MeshVerisi::yeni(
            vec![Vektor3::SIFIR, Vektor3::SAG, Vektor3::YUKARI],
            vec![Vektor3::ILERI; 3],
            vec![0, 1, 2],
        );

        assert!(sonuc.is_ok());
    }
}
