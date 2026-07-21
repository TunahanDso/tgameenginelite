use tgame_matematik::Vektor3;
use tgame_varlik::VarlikKimligi;

/// Bir fizik gövdesinin simülasyondaki davranışını belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovdeTuru {
    /// Hareket etmeyen fakat dinamik gövdeleri engelleyen gövde.
    Statik,
    /// Hız, yerçekimi ve çarpışma çözümlemesine katılan gövde.
    Dinamik,
}

/// Bir varlığı sabit adımlı fizik simülasyonuna bağlar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FizikGovdesi {
    varlik: VarlikKimligi,
    tur: GovdeTuru,
    yari_boyut: Vektor3,
    hiz: Vektor3,
    yercekimi_etkin: bool,
    zeminde: bool,
}

impl FizikGovdesi {
    /// Statik küp gövdesi oluşturur.
    #[must_use]
    pub fn statik_kup(varlik: VarlikKimligi, olcek: Vektor3) -> Self {
        Self::yeni(varlik, GovdeTuru::Statik, yari_boyut(olcek))
    }

    /// Dinamik küp gövdesi oluşturur.
    #[must_use]
    pub fn dinamik_kup(varlik: VarlikKimligi, olcek: Vektor3) -> Self {
        Self::yeni(varlik, GovdeTuru::Dinamik, yari_boyut(olcek))
    }

    /// Kimlik, tür ve yarı boyutlarla fizik gövdesi oluşturur.
    #[must_use]
    pub fn yeni(varlik: VarlikKimligi, tur: GovdeTuru, yari_boyut: Vektor3) -> Self {
        Self {
            varlik,
            tur,
            yari_boyut: Vektor3::yeni(
                yari_boyut.x.abs(),
                yari_boyut.y.abs(),
                yari_boyut.z.abs(),
            ),
            hiz: Vektor3::SIFIR,
            yercekimi_etkin: tur == GovdeTuru::Dinamik,
            zeminde: false,
        }
    }

    /// Bağlı varlığın kimliğini döndürür.
    #[must_use]
    pub const fn varlik(self) -> VarlikKimligi {
        self.varlik
    }

    /// Gövde türünü döndürür.
    #[must_use]
    pub const fn tur(self) -> GovdeTuru {
        self.tur
    }

    /// Çarpışma kutusunun yarı boyutunu döndürür.
    #[must_use]
    pub const fn yari_boyut(self) -> Vektor3 {
        self.yari_boyut
    }

    /// Güncel dünya hızını döndürür.
    #[must_use]
    pub const fn hiz(self) -> Vektor3 {
        self.hiz
    }

    /// Dünya hızını değiştirir.
    pub const fn hizi_ayarla(&mut self, hiz: Vektor3) {
        self.hiz = hiz;
    }

    /// Yatay X-Z hızını değiştirirken dikey hızı korur.
    pub const fn yatay_hizi_ayarla(&mut self, hiz: Vektor3) {
        self.hiz.x = hiz.x;
        self.hiz.z = hiz.z;
    }

    /// Yerçekimi etkisini açar veya kapatır.
    pub const fn yercekimi_etkinlestir(&mut self, etkin: bool) {
        self.yercekimi_etkin = etkin;
    }

    /// Gövdenin destekleyen bir yüzey üzerinde olup olmadığını döndürür.
    #[must_use]
    pub const fn zeminde_mi(self) -> bool {
        self.zeminde
    }

    /// Gövde zemindeyse yukarı doğru zıplama hızı uygular.
    pub fn ziplat(&mut self, hiz: f32) -> bool {
        if self.tur != GovdeTuru::Dinamik || !self.zeminde || !hiz.is_finite() || hiz <= 0.0 {
            return false;
        }

        self.hiz.y = hiz;
        self.zeminde = false;
        true
    }

    pub(super) const fn yercekimi_etkin(self) -> bool {
        self.yercekimi_etkin
    }

    pub(super) const fn zemini_ayarla(&mut self, zeminde: bool) {
        self.zeminde = zeminde;
    }

    pub(super) const fn eksen_hizini_sifirla(&mut self, eksen: usize) {
        match eksen {
            0 => self.hiz.x = 0.0,
            1 => self.hiz.y = 0.0,
            _ => self.hiz.z = 0.0,
        }
    }
}

fn yari_boyut(olcek: Vektor3) -> Vektor3 {
    Vektor3::yeni(olcek.x.abs() * 0.5, olcek.y.abs() * 0.5, olcek.z.abs() * 0.5)
}

#[cfg(test)]
mod testler {
    use tgame_matematik::Vektor3;
    use tgame_varlik::{Dunya, Varlik};

    use super::FizikGovdesi;

    #[test]
    fn dinamik_kup_olcekten_yari_boyut_uretir() {
        let mut dunya = Dunya::yeni_3b();
        let kimlik = dunya.varlik_ekle(Varlik::yeni("Deneme"));
        let govde = FizikGovdesi::dinamik_kup(kimlik, Vektor3::yeni(2.0, 4.0, 6.0));

        assert_eq!(govde.yari_boyut(), Vektor3::yeni(1.0, 2.0, 3.0));
    }
}
