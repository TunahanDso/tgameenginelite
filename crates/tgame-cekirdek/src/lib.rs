//! Tgame Engine Lite çekirdek türleri.

use std::fmt;

/// Motor genelinde kullanılan sonuç türü.
pub type OyunSonucu<T = ()> = Result<T, OyunHatasi>;

/// Motorun dışarıya sunduğu temel hata türü.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OyunHatasi {
    ileti: String,
}

impl OyunHatasi {
    #[must_use]
    pub fn yeni(ileti: impl Into<String>) -> Self {
        Self {
            ileti: ileti.into(),
        }
    }

    #[must_use]
    pub fn ileti(&self) -> &str {
        &self.ileti
    }
}

impl fmt::Display for OyunHatasi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ileti)
    }
}

impl std::error::Error for OyunHatasi {}

/// Oyunun iç çözünürlüğünü tanımlar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cozunurluk {
    pub genislik: u32,
    pub yukseklik: u32,
}

impl Cozunurluk {
    pub const BASLANGIC: Self = Self::yeni(800, 600);

    #[must_use]
    pub const fn yeni(genislik: u32, yukseklik: u32) -> Self {
        Self {
            genislik,
            yukseklik,
        }
    }

    /// Çözünürlüğün kullanılabilir olup olmadığını doğrular.
    ///
    /// # Errors
    ///
    /// Genişlik veya yükseklik sıfır olduğunda [`OyunHatasi`] döndürür.
    pub fn dogrula(self) -> OyunSonucu<Self> {
        if self.genislik == 0 || self.yukseklik == 0 {
            return Err(OyunHatasi::yeni("Çözünürlük sıfır olamaz."));
        }

        Ok(self)
    }
}

/// Motor başlatılırken kullanılan değişmez ayarlar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OyunAyarlari {
    pub baslik: String,
    pub cozunurluk: Cozunurluk,
    pub mod_klasoru: String,
}

impl OyunAyarlari {
    #[must_use]
    pub fn yeni(baslik: impl Into<String>) -> Self {
        Self {
            baslik: baslik.into(),
            cozunurluk: Cozunurluk::BASLANGIC,
            mod_klasoru: "modlar".to_owned(),
        }
    }
}

#[cfg(test)]
mod testler {
    use super::{Cozunurluk, OyunAyarlari, OyunHatasi};

    #[test]
    fn baslangic_cozunurlugu_800x600_dur() {
        assert_eq!(Cozunurluk::BASLANGIC, Cozunurluk::yeni(800, 600));
    }

    #[test]
    fn sifir_boyutlu_cozunurluk_reddedilir() {
        let hata = Cozunurluk::yeni(0, 600)
            .dogrula()
            .expect_err("Sıfır genişlik geçersiz olmalıdır.");

        assert_eq!(hata.ileti(), "Çözünürlük sıfır olamaz.");
    }

    #[test]
    fn oyun_ayarlari_guvenli_varsayilanlarla_olusturulur() {
        let ayarlar = OyunAyarlari::yeni("Deneme");

        assert_eq!(ayarlar.baslik, "Deneme");
        assert_eq!(ayarlar.cozunurluk, Cozunurluk::BASLANGIC);
        assert_eq!(ayarlar.mod_klasoru, "modlar");
    }

    #[test]
    fn oyun_hatasi_iletiyi_gosterir() {
        let hata = OyunHatasi::yeni("örnek hata");

        assert_eq!(hata.to_string(), "örnek hata");
    }
}
