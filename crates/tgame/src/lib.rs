//! Tgame Engine Lite kullanıcı API'si.

use std::fmt;

use tgame_cekirdek::{Cozunurluk, OyunAyarlari, OyunSonucu};
use tgame_girdi::Girdi;
use tgame_mod::ModYoneticisi;
use tgame_pencere::{KareGorevi, OyunAkisi, PencereAyarlari, calistir as pencereyi_calistir};
use tgame_sahne::Sahne;
use tgame_varlik::Dunya;
use tgame_zaman::Zaman;

/// Oyun geliştiricisinin doğrudan kullandığı ana motor yapısı.
pub struct Oyun {
    ayarlar: OyunAyarlari,
    sahneler: Vec<Sahne>,
    mod_yoneticisi: ModYoneticisi,
    dunya: Dunya,
    kare_gorevi: Option<KareGorevi>,
}

impl Oyun {
    /// Varsayılan 800×600 çözünürlükte yeni bir oyun oluşturur.
    #[must_use]
    pub fn yeni(baslik: impl Into<String>) -> Self {
        Self {
            ayarlar: OyunAyarlari::yeni(baslik),
            sahneler: Vec::new(),
            mod_yoneticisi: ModYoneticisi::yeni(),
            dunya: Dunya::yeni(),
            kare_gorevi: None,
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

    /// Başlangıç oyun dünyasını değiştirir.
    #[must_use]
    pub fn dunya(mut self, dunya: Dunya) -> Self {
        self.dunya = dunya;
        self
    }

    /// Her karede çalışacak oyun görevini belirler.
    ///
    /// Görev güncel klavye durumunu, kare zamanını ve değiştirilebilir oyun
    /// dünyasını alır. Döndürdüğü [`OyunAkisi`] oyunun devamını belirler.
    #[must_use]
    pub fn her_kare<F>(mut self, gorev: F) -> Self
    where
        F: FnMut(&Girdi, &Zaman, &mut Dunya) -> OyunAkisi + 'static,
    {
        self.kare_gorevi = Some(Box::new(gorev));
        self
    }

    /// Motoru doğrular, oyun penceresini oluşturur ve olay döngüsünü başlatır.
    ///
    /// # Errors
    ///
    /// Oyun ayarları geçersizse, olay döngüsü, pencere veya GPU grafik sistemi
    /// oluşturulamazsa [`tgame_cekirdek::OyunHatasi`] döndürür.
    pub fn calistir(self) -> OyunSonucu {
        let Self {
            ayarlar,
            sahneler,
            mod_yoneticisi,
            dunya,
            kare_gorevi,
        } = self;
        let cozunurluk = ayarlar.cozunurluk.dogrula()?;
        let kare_gorevi = kare_gorevi.unwrap_or_else(|| Box::new(|_, _, _| OyunAkisi::DevamEt));

        println!(
            "{} başlatılıyor — {}×{} — {} sahne — {} varlık — {} yüklü mod — mod klasörü: {}",
            ayarlar.baslik,
            cozunurluk.genislik,
            cozunurluk.yukseklik,
            sahneler.len(),
            dunya.varliklar().len(),
            mod_yoneticisi.yuklu_modlar().len(),
            ayarlar.mod_klasoru,
        );

        pencereyi_calistir(
            PencereAyarlari::yeni(ayarlar.baslik, cozunurluk),
            dunya,
            kare_gorevi,
        )
    }
}

impl fmt::Debug for Oyun {
    fn fmt(&self, bicimlendirici: &mut fmt::Formatter<'_>) -> fmt::Result {
        bicimlendirici
            .debug_struct("Oyun")
            .field("ayarlar", &self.ayarlar)
            .field("sahneler", &self.sahneler)
            .field("mod_yoneticisi", &self.mod_yoneticisi)
            .field("dunya", &self.dunya)
            .field("kare_gorevi_tanimli", &self.kare_gorevi.is_some())
            .finish()
    }
}

/// Oyun geliştiricilerinin tek satırda içe aktaracağı önsöz modülü.
pub mod onsoz {
    pub use crate::Oyun;
    pub use tgame_cekirdek::{Cozunurluk, OyunHatasi, OyunSonucu};
    pub use tgame_girdi::{Girdi, Tus};
    pub use tgame_matematik::{Renk, Vektor2};
    pub use tgame_mod::{ModBilgisi, ModYoneticisi};
    pub use tgame_pencere::OyunAkisi;
    pub use tgame_sahne::Sahne;
    pub use tgame_varlik::{Donusum2B, Dunya, Gorunum2B, Kamera2B, Varlik, VarlikKimligi};
    pub use tgame_zaman::Zaman;
}

#[cfg(test)]
mod testler {
    use super::Oyun;
    use tgame_cekirdek::Cozunurluk;
    use tgame_pencere::OyunAkisi;
    use tgame_sahne::Sahne;
    use tgame_varlik::Dunya;

    #[test]
    fn oyun_varsayilan_ayarlarla_olusturulur() {
        let oyun = Oyun::yeni("Deneme");

        assert_eq!(oyun.ayarlar.baslik, "Deneme");
        assert_eq!(oyun.ayarlar.cozunurluk, Cozunurluk::BASLANGIC);
        assert_eq!(oyun.ayarlar.mod_klasoru, "modlar");
        assert!(oyun.sahneler.is_empty());
        assert!(oyun.mod_yoneticisi.yuklu_modlar().is_empty());
        assert!(oyun.dunya.varliklar().is_empty());
        assert!(oyun.kare_gorevi.is_none());
    }

    #[test]
    fn kurucu_yontemler_oyunu_yapilandirir() {
        let oyun = Oyun::yeni("Deneme")
            .cozunurluk(1024, 768)
            .mod_klasoru("eklentiler")
            .sahne_ekle(Sahne::yeni("Baslangic"))
            .dunya(Dunya::yeni())
            .her_kare(|_, _, _| OyunAkisi::DevamEt);

        assert_eq!(oyun.ayarlar.cozunurluk, Cozunurluk::yeni(1024, 768));
        assert_eq!(oyun.ayarlar.mod_klasoru, "eklentiler");
        assert_eq!(oyun.sahneler.len(), 1);
        assert!(oyun.kare_gorevi.is_some());
    }
}
