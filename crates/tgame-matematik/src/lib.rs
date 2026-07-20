//! Tgame Engine Lite Türkçe matematik katmanı.

mod ucboyut;

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

pub use ucboyut::{Matris4, Vektor3};

/// İki boyutlu konum, yön ve ölçek değerlerini taşır.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vektor2 {
    /// Yatay bileşen.
    pub x: f32,
    /// Dikey bileşen.
    pub y: f32,
}

impl Vektor2 {
    /// Sıfır vektörü.
    pub const SIFIR: Self = Self::yeni(0.0, 0.0);
    /// Her iki bileşeni bir olan vektör.
    pub const BIR: Self = Self::yeni(1.0, 1.0);
    /// Yukarı yönü.
    pub const YUKARI: Self = Self::yeni(0.0, 1.0);
    /// Aşağı yönü.
    pub const ASAGI: Self = Self::yeni(0.0, -1.0);
    /// Sol yönü.
    pub const SOL: Self = Self::yeni(-1.0, 0.0);
    /// Sağ yönü.
    pub const SAG: Self = Self::yeni(1.0, 0.0);

    /// Yeni bir iki boyutlu vektör oluşturur.
    #[must_use]
    pub const fn yeni(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Vektör uzunluğunun karesini döndürür.
    #[must_use]
    pub const fn uzunluk_karesi(self) -> f32 {
        self.x * self.x + self.y * self.y
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
}

impl Add for Vektor2 {
    type Output = Self;

    fn add(self, sag: Self) -> Self::Output {
        Self::yeni(self.x + sag.x, self.y + sag.y)
    }
}

impl AddAssign for Vektor2 {
    fn add_assign(&mut self, sag: Self) {
        *self = *self + sag;
    }
}

impl Sub for Vektor2 {
    type Output = Self;

    fn sub(self, sag: Self) -> Self::Output {
        Self::yeni(self.x - sag.x, self.y - sag.y)
    }
}

impl SubAssign for Vektor2 {
    fn sub_assign(&mut self, sag: Self) {
        *self = *self - sag;
    }
}

impl Mul<f32> for Vektor2 {
    type Output = Self;

    fn mul(self, sag: f32) -> Self::Output {
        Self::yeni(self.x * sag, self.y * sag)
    }
}

impl MulAssign<f32> for Vektor2 {
    fn mul_assign(&mut self, sag: f32) {
        *self = *self * sag;
    }
}

impl Div<f32> for Vektor2 {
    type Output = Self;

    fn div(self, sag: f32) -> Self::Output {
        Self::yeni(self.x / sag, self.y / sag)
    }
}

/// Doğrusal RGBA renk değerini taşır.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Renk {
    /// Kırmızı bileşen.
    pub kirmizi: f32,
    /// Yeşil bileşen.
    pub yesil: f32,
    /// Mavi bileşen.
    pub mavi: f32,
    /// Saydamlık bileşeni.
    pub alfa: f32,
}

impl Renk {
    /// Beyaz renk.
    pub const BEYAZ: Self = Self::yeni(1.0, 1.0, 1.0, 1.0);
    /// Kırmızı renk.
    pub const KIRMIZI: Self = Self::yeni(0.95, 0.20, 0.18, 1.0);
    /// Yeşil renk.
    pub const YESIL: Self = Self::yeni(0.20, 0.85, 0.42, 1.0);
    /// Mavi renk.
    pub const MAVI: Self = Self::yeni(0.20, 0.45, 0.95, 1.0);
    /// Sarı renk.
    pub const SARI: Self = Self::yeni(0.95, 0.80, 0.20, 1.0);

    /// Yeni bir doğrusal RGBA rengi oluşturur.
    #[must_use]
    pub const fn yeni(kirmizi: f32, yesil: f32, mavi: f32, alfa: f32) -> Self {
        Self {
            kirmizi,
            yesil,
            mavi,
            alfa,
        }
    }
}

impl Default for Renk {
    fn default() -> Self {
        Self::BEYAZ
    }
}

#[cfg(test)]
mod testler {
    use super::{Renk, Vektor2};

    fn yakin(sol: f32, sag: f32) -> bool {
        (sol - sag).abs() < 0.000_01
    }

    #[test]
    fn vektor_islemleri_dogru_sonuc_verir() {
        let mut konum = Vektor2::yeni(2.0, -1.0);
        konum += Vektor2::YUKARI * 3.0;

        assert!(yakin(konum.x, 2.0));
        assert!(yakin(konum.y, 2.0));
    }

    #[test]
    fn birim_vektor_uzunlugu_birdir() {
        let yon = Vektor2::yeni(3.0, 4.0).birim();

        assert!(yakin(yon.uzunluk(), 1.0));
    }

    #[test]
    fn varsayilan_renk_beyazdir() {
        assert_eq!(Renk::default(), Renk::BEYAZ);
    }
}
