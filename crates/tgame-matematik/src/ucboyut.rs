use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

/// Üç boyutlu konum, yön, dönüş ekseni ve ölçek değerlerini taşır.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vektor3 {
    /// Yatay bileşen.
    pub x: f32,
    /// Dikey bileşen.
    pub y: f32,
    /// Derinlik bileşeni.
    pub z: f32,
}

impl Vektor3 {
    /// Sıfır vektörü.
    pub const SIFIR: Self = Self::yeni(0.0, 0.0, 0.0);
    /// Her bileşeni bir olan vektör.
    pub const BIR: Self = Self::yeni(1.0, 1.0, 1.0);
    /// Pozitif Y yönü.
    pub const YUKARI: Self = Self::yeni(0.0, 1.0, 0.0);
    /// Negatif Y yönü.
    pub const ASAGI: Self = Self::yeni(0.0, -1.0, 0.0);
    /// Negatif X yönü.
    pub const SOL: Self = Self::yeni(-1.0, 0.0, 0.0);
    /// Pozitif X yönü.
    pub const SAG: Self = Self::yeni(1.0, 0.0, 0.0);
    /// Sağ elli koordinat sisteminde ileri yön.
    pub const ILERI: Self = Self::yeni(0.0, 0.0, -1.0);
    /// Sağ elli koordinat sisteminde geri yön.
    pub const GERI: Self = Self::yeni(0.0, 0.0, 1.0);

    /// Yeni bir üç boyutlu vektör oluşturur.
    #[must_use]
    pub const fn yeni(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Vektör uzunluğunun karesini döndürür.
    #[must_use]
    pub const fn uzunluk_karesi(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Vektör uzunluğunu döndürür.
    #[must_use]
    pub fn uzunluk(self) -> f32 {
        self.uzunluk_karesi().sqrt()
    }

    /// Sıfır değilse birim uzunluğa getirir; sıfırsa sıfır döndürür.
    #[must_use]
    pub fn birim(self) -> Self {
        let uzunluk = self.uzunluk();
        if uzunluk > f32::EPSILON {
            self / uzunluk
        } else {
            Self::SIFIR
        }
    }

    /// İki vektörün nokta çarpımını döndürür.
    #[must_use]
    pub const fn nokta(self, diger: Self) -> f32 {
        self.x * diger.x + self.y * diger.y + self.z * diger.z
    }

    /// İki vektörün çapraz çarpımını döndürür.
    #[must_use]
    pub const fn capraz(self, diger: Self) -> Self {
        Self::yeni(
            self.y * diger.z - self.z * diger.y,
            self.z * diger.x - self.x * diger.z,
            self.x * diger.y - self.y * diger.x,
        )
    }
}

impl Add for Vektor3 {
    type Output = Self;

    fn add(self, sag: Self) -> Self::Output {
        Self::yeni(self.x + sag.x, self.y + sag.y, self.z + sag.z)
    }
}

impl AddAssign for Vektor3 {
    fn add_assign(&mut self, sag: Self) {
        *self = *self + sag;
    }
}

impl Sub for Vektor3 {
    type Output = Self;

    fn sub(self, sag: Self) -> Self::Output {
        Self::yeni(self.x - sag.x, self.y - sag.y, self.z - sag.z)
    }
}

impl SubAssign for Vektor3 {
    fn sub_assign(&mut self, sag: Self) {
        *self = *self - sag;
    }
}

impl Mul<f32> for Vektor3 {
    type Output = Self;

    fn mul(self, sag: f32) -> Self::Output {
        Self::yeni(self.x * sag, self.y * sag, self.z * sag)
    }
}

impl MulAssign<f32> for Vektor3 {
    fn mul_assign(&mut self, sag: f32) {
        *self = *self * sag;
    }
}

impl Div<f32> for Vektor3 {
    type Output = Self;

    fn div(self, sag: f32) -> Self::Output {
        Self::yeni(self.x / sag, self.y / sag, self.z / sag)
    }
}

/// GPU ile uyumlu sütun öncelikli 4×4 dönüşüm matrisidir.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matris4 {
    degerler: [f32; 16],
}

impl Matris4 {
    /// Birim matris.
    pub const BIRIM: Self = Self::yeni([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]);

    /// Sütun öncelikli ham değerlerden matris oluşturur.
    #[must_use]
    pub const fn yeni(degerler: [f32; 16]) -> Self {
        Self { degerler }
    }

    /// GPU tamponuna yazılabilecek sütun öncelikli değerleri döndürür.
    #[must_use]
    pub const fn degerler(self) -> [f32; 16] {
        self.degerler
    }

    /// Öteleme matrisi oluşturur.
    #[must_use]
    pub const fn oteleme(konum: Vektor3) -> Self {
        Self::yeni([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, konum.x, konum.y, konum.z,
            1.0,
        ])
    }

    /// Ölçek matrisi oluşturur.
    #[must_use]
    pub const fn olcekleme(olcek: Vektor3) -> Self {
        Self::yeni([
            olcek.x, 0.0, 0.0, 0.0, 0.0, olcek.y, 0.0, 0.0, 0.0, 0.0, olcek.z, 0.0, 0.0, 0.0, 0.0,
            1.0,
        ])
    }

    /// X ekseni çevresinde dönüş matrisi oluşturur.
    #[must_use]
    pub fn donus_x(radyan: f32) -> Self {
        let kosinus = radyan.cos();
        let sinus = radyan.sin();
        Self::yeni([
            1.0, 0.0, 0.0, 0.0, 0.0, kosinus, sinus, 0.0, 0.0, -sinus, kosinus, 0.0, 0.0, 0.0, 0.0,
            1.0,
        ])
    }

    /// Y ekseni çevresinde dönüş matrisi oluşturur.
    #[must_use]
    pub fn donus_y(radyan: f32) -> Self {
        let kosinus = radyan.cos();
        let sinus = radyan.sin();
        Self::yeni([
            kosinus, 0.0, -sinus, 0.0, 0.0, 1.0, 0.0, 0.0, sinus, 0.0, kosinus, 0.0, 0.0, 0.0, 0.0,
            1.0,
        ])
    }

    /// Z ekseni çevresinde dönüş matrisi oluşturur.
    #[must_use]
    pub fn donus_z(radyan: f32) -> Self {
        let kosinus = radyan.cos();
        let sinus = radyan.sin();
        Self::yeni([
            kosinus, sinus, 0.0, 0.0, -sinus, kosinus, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            1.0,
        ])
    }

    /// Konum, Euler dönüşü ve ölçekten model matrisi oluşturur.
    #[must_use]
    pub fn model(konum: Vektor3, donus_radyan: Vektor3, olcek: Vektor3) -> Self {
        Self::oteleme(konum)
            * Self::donus_z(donus_radyan.z)
            * Self::donus_y(donus_radyan.y)
            * Self::donus_x(donus_radyan.x)
            * Self::olcekleme(olcek)
    }

    /// Sağ elli kamera görünüm matrisi oluşturur.
    #[must_use]
    pub fn bakis_sag_el(goz: Vektor3, hedef: Vektor3, yukari: Vektor3) -> Self {
        let ileri = (hedef - goz).birim();
        let sag = ileri.capraz(yukari).birim();
        let gercek_yukari = sag.capraz(ileri);

        Self::yeni([
            sag.x,
            gercek_yukari.x,
            -ileri.x,
            0.0,
            sag.y,
            gercek_yukari.y,
            -ileri.y,
            0.0,
            sag.z,
            gercek_yukari.z,
            -ileri.z,
            0.0,
            -sag.nokta(goz),
            -gercek_yukari.nokta(goz),
            ileri.nokta(goz),
            1.0,
        ])
    }

    /// WGPU'nun 0–1 derinlik aralığına uygun perspektif matrisi oluşturur.
    #[must_use]
    pub fn perspektif_sag_el(
        dikey_gorus_radyan: f32,
        en_boy_orani: f32,
        yakin: f32,
        uzak: f32,
    ) -> Self {
        let odak = 1.0 / (dikey_gorus_radyan * 0.5).tan();
        let derinlik = uzak / (yakin - uzak);
        let oteleme = yakin * uzak / (yakin - uzak);

        Self::yeni([
            odak / en_boy_orani,
            0.0,
            0.0,
            0.0,
            0.0,
            odak,
            0.0,
            0.0,
            0.0,
            0.0,
            derinlik,
            -1.0,
            0.0,
            0.0,
            oteleme,
            0.0,
        ])
    }
}

impl Default for Matris4 {
    fn default() -> Self {
        Self::BIRIM
    }
}

impl Mul for Matris4 {
    type Output = Self;

    fn mul(self, sag: Self) -> Self::Output {
        let mut sonuc = [0.0; 16];
        for (sutun, sutun_degerleri) in sonuc.chunks_exact_mut(4).enumerate() {
            for (satir, hedef) in sutun_degerleri.iter_mut().enumerate() {
                *hedef = self.degerler[satir] * sag.degerler[sutun * 4]
                    + self.degerler[4 + satir] * sag.degerler[sutun * 4 + 1]
                    + self.degerler[8 + satir] * sag.degerler[sutun * 4 + 2]
                    + self.degerler[12 + satir] * sag.degerler[sutun * 4 + 3];
            }
        }
        Self::yeni(sonuc)
    }
}

#[cfg(test)]
mod testler {
    use super::{Matris4, Vektor3};

    fn yakin(sol: f32, sag: f32) -> bool {
        (sol - sag).abs() < 0.000_01
    }

    #[test]
    fn capraz_carpim_dik_vektor_uretir() {
        let sonuc = Vektor3::SAG.capraz(Vektor3::YUKARI);
        assert_eq!(sonuc, Vektor3::GERI);
    }

    #[test]
    fn uc_boyutlu_birim_vektorun_uzunlugu_birdir() {
        let yon = Vektor3::yeni(2.0, 3.0, 6.0).birim();
        assert!(yakin(yon.uzunluk(), 1.0));
    }

    #[test]
    fn model_matrisi_otelemeyi_son_sutunda_tasir() {
        let matris =
            Matris4::model(Vektor3::yeni(2.0, 3.0, 4.0), Vektor3::SIFIR, Vektor3::BIR).degerler();
        assert!(yakin(matris[12], 2.0));
        assert!(yakin(matris[13], 3.0));
        assert!(yakin(matris[14], 4.0));
    }

    #[test]
    fn birim_matris_carpimi_degeri_degistirmez() {
        let oteleme = Matris4::oteleme(Vektor3::yeni(1.0, 2.0, 3.0));
        assert_eq!(Matris4::BIRIM * oteleme, oteleme);
    }
}
