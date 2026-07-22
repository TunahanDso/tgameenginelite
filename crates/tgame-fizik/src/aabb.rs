use tgame_matematik::Vektor3;

/// Eksenlere hizalı üç boyutlu çarpışma kutusudur.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb3 {
    merkez: Vektor3,
    yari_boyut: Vektor3,
}

impl Aabb3 {
    /// Merkez ve pozitif yarı boyutlardan çarpışma kutusu oluşturur.
    #[must_use]
    pub fn yeni(merkez: Vektor3, yari_boyut: Vektor3) -> Self {
        Self {
            merkez,
            yari_boyut: Vektor3::yeni(yari_boyut.x.abs(), yari_boyut.y.abs(), yari_boyut.z.abs()),
        }
    }

    /// Kutunun merkezini döndürür.
    #[must_use]
    pub const fn merkez(self) -> Vektor3 {
        self.merkez
    }

    /// Kutunun yarı boyutunu döndürür.
    #[must_use]
    pub const fn yari_boyut(self) -> Vektor3 {
        self.yari_boyut
    }

    /// Kutunun en küçük köşesini döndürür.
    #[must_use]
    pub fn en_kucuk(self) -> Vektor3 {
        self.merkez - self.yari_boyut
    }

    /// Kutunun en büyük köşesini döndürür.
    #[must_use]
    pub fn en_buyuk(self) -> Vektor3 {
        self.merkez + self.yari_boyut
    }

    /// İki kutunun hacimsel olarak kesişip kesişmediğini döndürür.
    #[must_use]
    pub fn kesisiyor_mu(self, diger: Self) -> bool {
        let sol_en_kucuk = self.en_kucuk();
        let sol_en_buyuk = self.en_buyuk();
        let sag_en_kucuk = diger.en_kucuk();
        let sag_en_buyuk = diger.en_buyuk();

        sol_en_kucuk.x < sag_en_buyuk.x
            && sol_en_buyuk.x > sag_en_kucuk.x
            && sol_en_kucuk.y < sag_en_buyuk.y
            && sol_en_buyuk.y > sag_en_kucuk.y
            && sol_en_kucuk.z < sag_en_buyuk.z
            && sol_en_buyuk.z > sag_en_kucuk.z
    }
}

#[cfg(test)]
mod testler {
    use tgame_matematik::Vektor3;

    use super::Aabb3;

    #[test]
    fn ust_uste_binen_kutular_kesisir() {
        let sol = Aabb3::yeni(Vektor3::SIFIR, Vektor3::BIR);
        let sag = Aabb3::yeni(Vektor3::yeni(1.5, 0.0, 0.0), Vektor3::BIR);

        assert!(sol.kesisiyor_mu(sag));
    }

    #[test]
    fn yalnizca_yuzeyden_degen_kutular_kesismez() {
        let sol = Aabb3::yeni(Vektor3::SIFIR, Vektor3::BIR);
        let sag = Aabb3::yeni(Vektor3::yeni(2.0, 0.0, 0.0), Vektor3::BIR);

        assert!(!sol.kesisiyor_mu(sag));
    }
}
