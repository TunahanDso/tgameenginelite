//! Prosedürel üç boyutlu çevre mesh üreticileri.

use num_traits::ToPrimitive;
use tgame_cekirdek::{OyunHatasi, OyunSonucu};
use tgame_matematik::{Vektor2, Vektor3};

use crate::{MalzemeVerisi, MeshVerisi};

/// Düzenli bir yükseklik ızgarasından arazi mesh'i üretir.
#[derive(Debug, Clone, PartialEq)]
pub struct AraziUreteci {
    sutun: usize,
    satir: usize,
    hucre_boyutu: f32,
    yukseklikler: Vec<f32>,
}

impl AraziUreteci {
    /// Yükseklik verisinden arazi üreticisi oluşturur.
    ///
    /// # Errors
    ///
    /// Izgara boyutlarından biri ikiden küçükse, hücre boyutu geçersizse,
    /// yükseklik sayısı boyutlarla eşleşmezse veya sonlu olmayan değer varsa hata döndürür.
    pub fn yeni(
        sutun: usize,
        satir: usize,
        hucre_boyutu: f32,
        yukseklikler: Vec<f32>,
    ) -> OyunSonucu<Self> {
        if sutun < 2 || satir < 2 {
            return Err(OyunHatasi::yeni("Arazi ızgarası en az 2×2 tepe içermeli."));
        }
        if !hucre_boyutu.is_finite() || hucre_boyutu <= f32::EPSILON {
            return Err(OyunHatasi::yeni(
                "Arazi hücre boyutu pozitif ve sonlu olmalı.",
            ));
        }
        let beklenen = sutun
            .checked_mul(satir)
            .ok_or_else(|| OyunHatasi::yeni("Arazi boyutu desteklenen sınırı aştı."))?;
        if yukseklikler.len() != beklenen {
            return Err(OyunHatasi::yeni(
                "Arazi yükseklik sayısı ızgara boyutlarıyla eşleşmiyor.",
            ));
        }
        if yukseklikler.iter().any(|yukseklik| !yukseklik.is_finite()) {
            return Err(OyunHatasi::yeni(
                "Arazi sonlu olmayan yükseklik değeri içeriyor.",
            ));
        }
        Ok(Self {
            sutun,
            satir,
            hucre_boyutu,
            yukseklikler,
        })
    }

    /// Bir fonksiyonu örnekleyerek arazi üreticisi oluşturur.
    ///
    /// # Errors
    ///
    /// Boyutlar veya örneklenen yükseklikler geçersizse hata döndürür.
    pub fn fonksiyondan<F>(
        sutun: usize,
        satir: usize,
        hucre_boyutu: f32,
        mut yukseklik: F,
    ) -> OyunSonucu<Self>
    where
        F: FnMut(f32, f32) -> f32,
    {
        let yarim_x = indeks_f32(sutun.saturating_sub(1))? * hucre_boyutu * 0.5;
        let yarim_z = indeks_f32(satir.saturating_sub(1))? * hucre_boyutu * 0.5;
        let mut yukseklikler = Vec::with_capacity(sutun.saturating_mul(satir));
        for z in 0..satir {
            let dunya_z = indeks_f32(z)? * hucre_boyutu - yarim_z;
            for x in 0..sutun {
                let dunya_x = indeks_f32(x)? * hucre_boyutu - yarim_x;
                yukseklikler.push(yukseklik(dunya_x, dunya_z));
            }
        }
        Self::yeni(sutun, satir, hucre_boyutu, yukseklikler)
    }

    /// Arazi mesh'ini verilen malzemeyle üretir.
    ///
    /// # Errors
    ///
    /// Tepe veya indeks sayısı desteklenen sınırı aşarsa hata döndürür.
    pub fn mesh_uret(&self, malzeme: MalzemeVerisi) -> OyunSonucu<MeshVerisi> {
        let tepe_sayisi = self
            .sutun
            .checked_mul(self.satir)
            .ok_or_else(|| OyunHatasi::yeni("Arazi tepe sayısı desteklenen sınırı aştı."))?;
        let mut konumlar = Vec::with_capacity(tepe_sayisi);
        let mut normaller = Vec::with_capacity(tepe_sayisi);
        let mut uvler = Vec::with_capacity(tepe_sayisi);
        let yarim_x = indeks_f32(self.sutun - 1)? * self.hucre_boyutu * 0.5;
        let yarim_z = indeks_f32(self.satir - 1)? * self.hucre_boyutu * 0.5;
        let uv_bolen_x = indeks_f32(self.sutun - 1)?;
        let uv_bolen_z = indeks_f32(self.satir - 1)?;

        for z in 0..self.satir {
            for x in 0..self.sutun {
                let merkez = self.yukseklik(x, z);
                let sol = self.yukseklik(x.saturating_sub(1), z);
                let sag = self.yukseklik((x + 1).min(self.sutun - 1), z);
                let geri = self.yukseklik(x, z.saturating_sub(1));
                let ileri = self.yukseklik(x, (z + 1).min(self.satir - 1));
                let dunya_x = indeks_f32(x)? * self.hucre_boyutu - yarim_x;
                let dunya_z = indeks_f32(z)? * self.hucre_boyutu - yarim_z;
                konumlar.push(Vektor3::yeni(dunya_x, merkez, dunya_z));
                normaller
                    .push(Vektor3::yeni(sol - sag, 2.0 * self.hucre_boyutu, geri - ileri).birim());
                uvler.push(Vektor2::yeni(
                    indeks_f32(x)? / uv_bolen_x,
                    indeks_f32(z)? / uv_bolen_z,
                ));
            }
        }

        let hucre_sayisi = (self.sutun - 1)
            .checked_mul(self.satir - 1)
            .ok_or_else(|| OyunHatasi::yeni("Arazi indeks sayısı desteklenen sınırı aştı."))?;
        let mut indeksler = Vec::with_capacity(hucre_sayisi.saturating_mul(6));
        for z in 0..self.satir - 1 {
            for x in 0..self.sutun - 1 {
                let a = indeks_u32(z * self.sutun + x)?;
                let b = indeks_u32(z * self.sutun + x + 1)?;
                let c = indeks_u32((z + 1) * self.sutun + x)?;
                let d = indeks_u32((z + 1) * self.sutun + x + 1)?;
                indeksler.extend_from_slice(&[a, c, b, b, c, d]);
            }
        }
        MeshVerisi::yeni_malzemeli(konumlar, normaller, uvler, indeksler, malzeme)
    }

    fn yukseklik(&self, x: usize, z: usize) -> f32 {
        self.yukseklikler[z * self.sutun + x]
    }
}

/// Dikey silindir mesh'i üretir.
///
/// # Errors
///
/// Dilim sayısı üçten küçükse veya ölçüler geçersizse hata döndürür.
pub fn silindir(
    dilim: usize,
    yaricap: f32,
    yukseklik: f32,
    malzeme: MalzemeVerisi,
) -> OyunSonucu<MeshVerisi> {
    olculeri_dogrula(dilim, yaricap, yukseklik)?;
    let mut konumlar = Vec::with_capacity(dilim.saturating_mul(4).saturating_add(2));
    let mut normaller = Vec::with_capacity(konumlar.capacity());
    let mut uvler = Vec::with_capacity(konumlar.capacity());
    let mut indeksler = Vec::with_capacity(dilim.saturating_mul(12));
    let yarim = yukseklik * 0.5;

    for i in 0..=dilim {
        let oran = indeks_f32(i)? / indeks_f32(dilim)?;
        let aci = oran * std::f32::consts::TAU;
        let x = aci.cos() * yaricap;
        let z = aci.sin() * yaricap;
        let normal = Vektor3::yeni(x, 0.0, z).birim();
        konumlar.push(Vektor3::yeni(x, -yarim, z));
        konumlar.push(Vektor3::yeni(x, yarim, z));
        normaller.extend_from_slice(&[normal, normal]);
        uvler.extend_from_slice(&[Vektor2::yeni(oran, 1.0), Vektor2::yeni(oran, 0.0)]);
    }
    for i in 0..dilim {
        let alt = indeks_u32(i * 2)?;
        let ust = alt + 1;
        let sonraki_alt = alt + 2;
        let sonraki_ust = alt + 3;
        indeksler.extend_from_slice(&[alt, ust, sonraki_alt, sonraki_alt, ust, sonraki_ust]);
    }

    kapak_ekle(
        &mut konumlar,
        &mut normaller,
        &mut uvler,
        &mut indeksler,
        dilim,
        yaricap,
        -yarim,
        false,
    )?;
    kapak_ekle(
        &mut konumlar,
        &mut normaller,
        &mut uvler,
        &mut indeksler,
        dilim,
        yaricap,
        yarim,
        true,
    )?;
    MeshVerisi::yeni_malzemeli(konumlar, normaller, uvler, indeksler, malzeme)
}

/// Dikey koni mesh'i üretir.
///
/// # Errors
///
/// Dilim sayısı üçten küçükse veya ölçüler geçersizse hata döndürür.
pub fn koni(
    dilim: usize,
    yaricap: f32,
    yukseklik: f32,
    malzeme: MalzemeVerisi,
) -> OyunSonucu<MeshVerisi> {
    olculeri_dogrula(dilim, yaricap, yukseklik)?;
    let yarim = yukseklik * 0.5;
    let mut konumlar = Vec::with_capacity(dilim.saturating_mul(2).saturating_add(2));
    let mut normaller = Vec::with_capacity(konumlar.capacity());
    let mut uvler = Vec::with_capacity(konumlar.capacity());
    let mut indeksler = Vec::with_capacity(dilim.saturating_mul(6));

    for i in 0..dilim {
        let oran = indeks_f32(i)? / indeks_f32(dilim)?;
        let aci = oran * std::f32::consts::TAU;
        let x = aci.cos() * yaricap;
        let z = aci.sin() * yaricap;
        konumlar.push(Vektor3::yeni(x, -yarim, z));
        let egim = yaricap / yukseklik;
        normaller.push(Vektor3::yeni(aci.cos(), egim, aci.sin()).birim());
        uvler.push(Vektor2::yeni(oran, 1.0));
    }
    let tepe = indeks_u32(konumlar.len())?;
    konumlar.push(Vektor3::yeni(0.0, yarim, 0.0));
    normaller.push(Vektor3::YUKARI);
    uvler.push(Vektor2::yeni(0.5, 0.0));
    for i in 0..dilim {
        let a = indeks_u32(i)?;
        let b = indeks_u32((i + 1) % dilim)?;
        indeksler.extend_from_slice(&[a, tepe, b]);
    }
    kapak_ekle(
        &mut konumlar,
        &mut normaller,
        &mut uvler,
        &mut indeksler,
        dilim,
        yaricap,
        -yarim,
        false,
    )?;
    MeshVerisi::yeni_malzemeli(konumlar, normaller, uvler, indeksler, malzeme)
}

/// UV küre veya düşük poligon kaya mesh'i üretir.
///
/// `duzensizlik` sıfırsa düzgün küre, pozitifse yüzeyi deterministik biçimde kırılmış kaya oluşur.
///
/// # Errors
///
/// Dilim/katman sayıları veya ölçüler geçersizse hata döndürür.
pub fn kure(
    dilim: usize,
    katman: usize,
    yaricap: f32,
    duzensizlik: f32,
    malzeme: MalzemeVerisi,
) -> OyunSonucu<MeshVerisi> {
    if dilim < 3 || katman < 2 {
        return Err(OyunHatasi::yeni(
            "Küre en az üç dilim ve iki katman içermeli.",
        ));
    }
    if !yaricap.is_finite() || yaricap <= f32::EPSILON || !duzensizlik.is_finite() {
        return Err(OyunHatasi::yeni("Küre ölçüleri pozitif ve sonlu olmalı."));
    }
    let duzensizlik = duzensizlik.clamp(0.0, 0.45);
    let tepe_sayisi = (dilim + 1)
        .checked_mul(katman + 1)
        .ok_or_else(|| OyunHatasi::yeni("Küre tepe sayısı desteklenen sınırı aştı."))?;
    let mut konumlar = Vec::with_capacity(tepe_sayisi);
    let mut normaller = Vec::with_capacity(tepe_sayisi);
    let mut uvler = Vec::with_capacity(tepe_sayisi);
    let mut indeksler = Vec::with_capacity(dilim.saturating_mul(katman).saturating_mul(6));

    for y in 0..=katman {
        let v = indeks_f32(y)? / indeks_f32(katman)?;
        let kutup = v * std::f32::consts::PI;
        for x in 0..=dilim {
            let u = indeks_f32(x)? / indeks_f32(dilim)?;
            let cevre = u * std::f32::consts::TAU;
            let yon = Vektor3::yeni(
                cevre.cos() * kutup.sin(),
                kutup.cos(),
                cevre.sin() * kutup.sin(),
            );
            let kirilma = 1.0
                + duzensizlik
                    * ((cevre * 3.1).sin() * (kutup * 4.3).cos() + (cevre * 7.0).cos())
                    * 0.5;
            konumlar.push(yon * (yaricap * kirilma));
            normaller.push(yon.birim());
            uvler.push(Vektor2::yeni(u, v));
        }
    }
    for y in 0..katman {
        for x in 0..dilim {
            let a = indeks_u32(y * (dilim + 1) + x)?;
            let b = a + 1;
            let c = indeks_u32((y + 1) * (dilim + 1) + x)?;
            let d = c + 1;
            indeksler.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
    MeshVerisi::yeni_malzemeli(konumlar, normaller, uvler, indeksler, malzeme)
}

fn kapak_ekle(
    konumlar: &mut Vec<Vektor3>,
    normaller: &mut Vec<Vektor3>,
    uvler: &mut Vec<Vektor2>,
    indeksler: &mut Vec<u32>,
    dilim: usize,
    yaricap: f32,
    y: f32,
    yukari: bool,
) -> OyunSonucu {
    let merkez = indeks_u32(konumlar.len())?;
    let normal = if yukari {
        Vektor3::YUKARI
    } else {
        Vektor3::ASAGI
    };
    konumlar.push(Vektor3::yeni(0.0, y, 0.0));
    normaller.push(normal);
    uvler.push(Vektor2::yeni(0.5, 0.5));
    let halka_baslangici = indeks_u32(konumlar.len())?;
    for i in 0..dilim {
        let oran = indeks_f32(i)? / indeks_f32(dilim)?;
        let aci = oran * std::f32::consts::TAU;
        let x = aci.cos() * yaricap;
        let z = aci.sin() * yaricap;
        konumlar.push(Vektor3::yeni(x, y, z));
        normaller.push(normal);
        uvler.push(Vektor2::yeni(
            x / yaricap * 0.5 + 0.5,
            z / yaricap * 0.5 + 0.5,
        ));
    }
    for i in 0..dilim {
        let a = halka_baslangici + indeks_u32(i)?;
        let b = halka_baslangici + indeks_u32((i + 1) % dilim)?;
        if yukari {
            indeksler.extend_from_slice(&[merkez, b, a]);
        } else {
            indeksler.extend_from_slice(&[merkez, a, b]);
        }
    }
    Ok(())
}

fn olculeri_dogrula(dilim: usize, yaricap: f32, yukseklik: f32) -> OyunSonucu {
    if dilim < 3 {
        return Err(OyunHatasi::yeni("Dairesel mesh en az üç dilim içermeli."));
    }
    if !yaricap.is_finite()
        || !yukseklik.is_finite()
        || yaricap <= f32::EPSILON
        || yukseklik <= f32::EPSILON
    {
        return Err(OyunHatasi::yeni("Mesh ölçüleri pozitif ve sonlu olmalı."));
    }
    Ok(())
}

fn indeks_u32(indeks: usize) -> OyunSonucu<u32> {
    u32::try_from(indeks)
        .map_err(|_| OyunHatasi::yeni("Mesh indeks sayısı desteklenen sınırı aştı."))
}

fn indeks_f32(indeks: usize) -> OyunSonucu<f32> {
    indeks
        .to_f32()
        .ok_or_else(|| OyunHatasi::yeni("Mesh sıra numarası f32 ile temsil edilemiyor."))
}

#[cfg(test)]
mod testler {
    use super::{AraziUreteci, koni, kure, silindir};
    use crate::MalzemeVerisi;

    #[test]
    fn arazi_izgarasi_beklenen_ucgen_sayisini_uretir() {
        let arazi = AraziUreteci::fonksiyondan(4, 3, 1.0, |x, z| (x + z) * 0.1)
            .expect("Arazi üreticisi kurulmalı.");
        let mesh = arazi
            .mesh_uret(MalzemeVerisi::default())
            .expect("Arazi mesh'i üretilmeli.");
        assert_eq!(mesh.konumlar().len(), 12);
        assert_eq!(mesh.indeksler().len(), 36);
    }

    #[test]
    fn dogal_sekiller_gecerli_mesh_uretir() {
        let malzeme = MalzemeVerisi::default();
        assert!(silindir(12, 0.5, 2.0, malzeme.clone()).is_ok());
        assert!(koni(12, 1.0, 2.0, malzeme.clone()).is_ok());
        assert!(kure(12, 6, 1.0, 0.2, malzeme).is_ok());
    }
}
