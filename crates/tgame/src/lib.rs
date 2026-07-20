//! Tgame Engine Lite kullanıcı API'si.

use tgame_cekirdek::{Cozunurluk, OyunAyarlari, OyunSonucu};
use tgame_mod::ModYoneticisi;
use tgame_pencere::{calistir as pencereyi_calistir, PencereAyarlari};
use tgame_sahne::Sahne;

/// Oyun geliştiricisinin doğrudan kullandığı ana motor yapısı.
#[derive(Debug)]
pub struct Oyun {
    ayarlar: OyunAyarlari,
    sahneler: Vec<Sahne>,
    mod_yoneticisi: ModYoneticisi,
}

impl Oyun {
    /// Varsayılan 800×600 çözünürlükte yeni bir oyun oluşturur.
    #[must_use]
    pub fn yeni(baslik: impl Into<String>) -> Self {
        Self {
            ayarlar: OyunAyarlari::yeni(baslik),
            sahneler: Vec::new(),
            mod_yoneticisi: ModYoneticisi::yeni(),
        }
    }

    /// Oyunun iç çözünürlüğünü değiştirir.
    #[must_use]
    pub fn cozunurluk(mut self, genislik: u32, yukseklik: u32) -> Self {
        self.ayarlar.cozunurluk = Cozunurluk::yeni(genislik, yukseklik);
        self
    }

    /// Modların aranacağı klasörü belirler.
    #[must_use]
    pub fn mod_klasoru(mut self, klasor: impl Into<String>) -> Self {
        self.ayarlar.mod_klasoru = klasor.into();
        self
    }

    /// Oyuna bir sahne ekler.
    #[must_use]
    pub fn sahne_ekle(mut self, sahne: Sahne) -> Self {
        self.sahneler.push(sahne);
        self
    }

    /// Motoru doğrular, oyun penceresini oluşturur ve olay döngüsünü başlatır.
    ///
    /// # Errors
    ///
    /// Oyun ayarları geçersizse, olay döngüsü oluşturulamazsa veya işletim
    /// sistemi pencere oluşturmayı reddederse [`tgame_cekirdek::OyunHatasi`]
    /// döndürür.
    pub fn calistir(self) -> OyunSonucu {
        let Self {
            ayarlar,
            sahneler,
            mod_yoneticisi,
        } = self;
        let cozunurluk = ayarlar.cozunurluk.dogrula()?;

        println!(
            "{} başlatılıyor — {}×{} — {} sahne — {} yüklü mod — mod klasörü: {}",
            ayarlar.baslik,
            cozunurluk.genislik,
            cozunurluk.yukseklik,
            sahneler.len(),
            mod_yoneticisi.yuklu_modlar().len(),
            ayarlar.mod_klasoru,
        );

        pencereyi_calistir(PencereAyarlari::yeni(ayarlar.baslik, cozunurluk))
    }
}

/// Oyun geliştiricilerinin tek satırda içe aktaracağı önsöz modülü.
pub mod onsoz {
    pub use crate::Oyun;
    pub use tgame_cekirdek::{Cozunurluk, OyunHatasi, OyunSonucu};
    pub use tgame_mod::{ModBilgisi, ModYoneticisi};
    pub use tgame_sahne::Sahne;
}

#[cfg(test)]
mod testler {
    use super::Oyun;
    use tgame_cekirdek::Cozunurluk;
    use tgame_sahne::Sahne;

    #[test]
    fn oyun_varsayilan_ayarlarla_olusturulur() {
        let oyun = Oyun::yeni("Deneme");

        assert_eq!(oyun.ayarlar.baslik, "Deneme");
        assert_eq!(oyun.ayarlar.cozunurluk, Cozunurluk::BASLANGIC);
        assert_eq!(oyun.ayarlar.mod_klasoru, "modlar");
        assert!(oyun.sahneler.is_empty());
        assert!(oyun.mod_yoneticisi.yuklu_modlar().is_empty());
    }

    #[test]
    fn kurucu_yontemler_oyunu_yapilandirir() {
        let oyun = Oyun::yeni("Deneme")
            .cozunurluk(1024, 768)
            .mod_klasoru("eklentiler")
            .sahne_ekle(Sahne::yeni("Baslangic"));

        assert_eq!(oyun.ayarlar.cozunurluk, Cozunurluk::yeni(1024, 768));
        assert_eq!(oyun.ayarlar.mod_klasoru, "eklentiler");
        assert_eq!(oyun.sahneler.len(), 1);
    }
}
