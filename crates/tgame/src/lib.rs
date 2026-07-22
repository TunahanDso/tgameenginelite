//! Tgame Engine Lite kullanıcı API'si.

use std::fmt;

use tgame_cekirdek::{Cozunurluk, OyunAyarlari, OyunSonucu};
use tgame_girdi::Girdi;
use tgame_macera::Macera;
use tgame_mod::ModYoneticisi;
use tgame_pencere::{KareGorevi, OyunAkisi, PencereAyarlari, calistir as pencereyi_calistir};
use tgame_sahne::Sahne;
use tgame_varlik::Dunya;
use tgame_zaman::Zaman;

type MaceraKareGorevi = Box<dyn FnMut(&Girdi, &Zaman, &mut Dunya, &mut Macera) -> OyunAkisi>;

/// Oyun geliştiricisinin doğrudan kullandığı ana motor yapısı.
pub struct Oyun {
    ayarlar: OyunAyarlari,
    sahneler: Vec<Sahne>,
    mod_yoneticisi: ModYoneticisi,
    dunya: Dunya,
    macera: Macera,
    kare_gorevi: Option<KareGorevi>,
    macera_kare_gorevi: Option<MaceraKareGorevi>,
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
            macera: Macera::yeni(),
            kare_gorevi: None,
            macera_kare_gorevi: None,
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

    /// Oyunun hikâye, görev, diyalog ve kayıt çalışma zamanını değiştirir.
    #[must_use]
    pub fn macera(mut self, macera: Macera) -> Self {
        self.macera = macera;
        self
    }

    /// Her karede macera çalışma zamanına erişen oyun görevini belirler.
    #[must_use]
    pub fn her_kare_macera<F>(mut self, gorev: F) -> Self
    where
        F: FnMut(&Girdi, &Zaman, &mut Dunya, &mut Macera) -> OyunAkisi + 'static,
    {
        self.kare_gorevi = None;
        self.macera_kare_gorevi = Some(Box::new(gorev));
        self
    }

    /// Her karede çalışacak oyun görevini belirler.
    ///
    /// Görev güncel klavye/fare durumunu, kare zamanını ve değiştirilebilir oyun
    /// dünyasını alır. Döndürdüğü [`OyunAkisi`] oyunun devamını belirler.
    #[must_use]
    pub fn her_kare<F>(mut self, gorev: F) -> Self
    where
        F: FnMut(&Girdi, &Zaman, &mut Dunya) -> OyunAkisi + 'static,
    {
        self.macera_kare_gorevi = None;
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
            macera,
            kare_gorevi,
            macera_kare_gorevi,
        } = self;
        let cozunurluk = ayarlar.cozunurluk.dogrula()?;
        let macera_raporu = macera.rapor();
        let kare_gorevi: KareGorevi = if let Some(mut gorev) = macera_kare_gorevi {
            let mut macera = macera;
            Box::new(move |girdi, zaman, dunya| {
                macera.sure_ekle(zaman.kare_suresi());
                gorev(girdi, zaman, dunya, &mut macera)
            })
        } else {
            kare_gorevi.unwrap_or_else(|| Box::new(|_, _, _| OyunAkisi::DevamEt))
        };

        println!(
            "{} başlatılıyor — {}×{} — {:?} — {} sahne — {} varlık — {} mesh — {} malzeme — {} doku — macera: {} eşya / {} görev / {} diyalog / {} bölüm / {} etkileşim / {} alan / {} kural — {} yüklü mod — mod klasörü: {}",
            ayarlar.baslik,
            cozunurluk.genislik,
            cozunurluk.yukseklik,
            dunya.boyut(),
            sahneler.len(),
            dunya.varliklar().len(),
            dunya.meshler().len(),
            dunya.malzemeler().len(),
            dunya.dokular().len(),
            macera_raporu.esya,
            macera_raporu.gorev,
            macera_raporu.diyalog,
            macera_raporu.sahne,
            macera_raporu.etkilesim,
            macera_raporu.alan,
            macera_raporu.kural,
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
            .field("macera", &self.macera)
            .field("kare_gorevi_tanimli", &self.kare_gorevi.is_some())
            .field(
                "macera_kare_gorevi_tanimli",
                &self.macera_kare_gorevi.is_some(),
            )
            .finish()
    }
}

/// Oyun geliştiricilerinin tek satırda içe aktaracağı önsöz modülü.
pub mod onsoz {
    pub use crate::Oyun;
    pub use tgame_arayuz::{Arayuz, ArayuzPaneli, EkranDikdortgeni, EkranMetni, EkranRengi};
    pub use tgame_cekirdek::{Cozunurluk, OyunHatasi, OyunSonucu};
    pub use tgame_fizik::{Aabb3, FizikDunyasi, FizikGovdesi, FizikRaporu, GovdeTuru};
    pub use tgame_girdi::{FareHareketi, Girdi, Tus};
    pub use tgame_macera::{
        AlanKimligi, AlanTetikleyicisi, DiyalogDugumu, DiyalogGorunumu, DiyalogKimligi,
        DiyalogSecenegi, DiyalogSecenegiGorunumu, DiyalogTanimi, Envanter, EsyaKimligi, EsyaTanimi,
        EtkilesimGorunumu, EtkilesimKimligi, EtkilesimNoktasi, Eylem, GorevAdimi, GorevAsamasi,
        GorevHedefi, GorevIlerlemesi, GorevKimligi, GorevTanimi, KayitYoneticisi, KontrolNoktasi,
        KontrolNoktasiKimligi, Kosul, KuralKimligi, KutuAlan, Macera, MaceraRaporu, OlayFiltresi,
        OlayKurali, OyunDurumu, OyunOlayi, SahneGecisi, SahneKimligi, SahneTanimi, Tekrarlama,
    };
    pub use tgame_matematik::{Matris4, Renk, Vektor2, Vektor3};
    pub use tgame_mod::{ModBilgisi, ModYoneticisi};
    pub use tgame_model::uretim::{AraziUreteci, koni, kure, silindir};
    pub use tgame_model::{
        DokuFiltresi, DokuSarmasi, DokuVerisi, MalzemeVerisi, MeshVerisi, ModelOrnegi, ModelVerisi,
        OrnekleyiciVerisi, SinirKuresi,
    };
    pub use tgame_oynanis::{
        AjanKarari, BasitAjan, Can, SavasDunyasi, SavasOlayi, Savasci, SesKuyrugu, SesOlayi, Takim,
    };
    pub use tgame_pencere::OyunAkisi;
    pub use tgame_sahne::Sahne;
    pub use tgame_varlik::{
        DokuKimligi, Donusum2B, Donusum3B, Dunya, DunyaBoyutu, Gorunum2B, Gorunum3B, Kamera2B,
        Kamera3B, MalzemeKaydi, MalzemeKimligi, MeshKimligi, Varlik, VarlikKimligi,
    };
    pub use tgame_zaman::Zaman;
}

#[cfg(test)]
mod testler {
    use super::Oyun;
    use tgame_cekirdek::Cozunurluk;
    use tgame_macera::Macera;
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
        assert!(oyun.dunya.meshler().is_empty());
        assert!(oyun.dunya.malzemeler().is_empty());
        assert!(oyun.dunya.dokular().is_empty());
        assert_eq!(oyun.macera.rapor().gorev, 0);
        assert!(oyun.kare_gorevi.is_none());
        assert!(oyun.macera_kare_gorevi.is_none());
    }

    #[test]
    fn kurucu_yontemler_oyunu_yapilandirir() {
        let oyun = Oyun::yeni("Deneme")
            .cozunurluk(1024, 768)
            .mod_klasoru("eklentiler")
            .sahne_ekle(Sahne::yeni("Baslangic"))
            .dunya(Dunya::yeni_3b())
            .her_kare(|_, _, _| OyunAkisi::DevamEt);

        assert_eq!(oyun.ayarlar.cozunurluk, Cozunurluk::yeni(1024, 768));
        assert_eq!(oyun.ayarlar.mod_klasoru, "eklentiler");
        assert_eq!(oyun.sahneler.len(), 1);
        assert!(oyun.kare_gorevi.is_some());
    }

    #[test]
    fn macera_kare_gorevi_oyunu_yapilandirir() {
        let oyun = Oyun::yeni("Macera")
            .macera(Macera::yeni())
            .her_kare_macera(|_, _, _, _| OyunAkisi::DevamEt);

        assert!(oyun.kare_gorevi.is_none());
        assert!(oyun.macera_kare_gorevi.is_some());
    }
}
