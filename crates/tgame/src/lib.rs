//! Tgame Engine Lite kullanıcı API'si.

use tgame_cekirdek::{Cozunurluk, OyunAyarlari, OyunSonucu};
use tgame_mod::ModYoneticisi;
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

    /// Motoru doğrular ve oyun döngüsünü başlatır.
    ///
    /// İlk iskelette gerçek pencere ve grafik döngüsü henüz eklenmemiştir.
    pub fn calistir(self) -> OyunSonucu {
        let cozunurluk = self.ayarlar.cozunurluk.dogrula()?;

        println!(
            "{} başlatılıyor — {}×{} — {} sahne — mod klasörü: {}",
            self.ayarlar.baslik,
            cozunurluk.genislik,
            cozunurluk.yukseklik,
            self.sahneler.len(),
            self.ayarlar.mod_klasoru,
        );

        let _ = self.mod_yoneticisi;
        Ok(())
    }
}

/// Oyun geliştiricilerinin tek satırda içe aktaracağı önsöz modülü.
pub mod onsoz {
    pub use crate::Oyun;
    pub use tgame_cekirdek::{Cozunurluk, OyunHatasi, OyunSonucu};
    pub use tgame_mod::{ModBilgisi, ModYoneticisi};
    pub use tgame_sahne::Sahne;
}
