mod ekran_arayuzu;
mod macera_icerigi;
mod sahne;

use ekran_arayuzu::arayuzu_guncelle;
use macera_icerigi::{MaceraKimlikleri, macerayi_olustur};
use sahne::{SahneKurulumu, sahneyi_olustur};
use tgame::onsoz::{
    AjanKarari, BasitAjan, Dunya, FizikDunyasi, Girdi, GorevAsamasi, GorevIlerlemesi,
    KayitYoneticisi, Macera, Oyun, OyunAkisi, OyunOlayi, OyunSonucu, Sahne, SahneKimligi,
    SavasDunyasi, SavasOlayi, Savasci, SesKuyrugu, SesOlayi, Takim, Tus, VarlikKimligi, Vektor3,
    Zaman,
};

const OYUNCU_HIZI: f32 = 4.8;
const ZIPLAMA_HIZI: f32 = 6.2;
const FARE_HASSASIYETI: f32 = 0.0025;
const KLAVYE_KAMERA_HIZI: f32 = 1.5;
const KAMERA_UZAKLIGI: f32 = 7.5;
const KAYIT_KLASORU: &str = "kayitlar/kayip-muhur";

struct DemoDurumu {
    fizik: FizikDunyasi,
    oyuncu: VarlikKimligi,
    merkez: VarlikKimligi,
    muhafiz: VarlikKimligi,
    piramitler: Vec<VarlikKimligi>,
    kamera_yatay: f32,
    kamera_dikey: f32,
    kayitlar: KayitYoneticisi,
    kimlikler: MaceraKimlikleri,
    son_gorev_asamasi: GorevAsamasi,
    savas: SavasDunyasi,
    muhafiz_ajani: BasitAjan,
    sesler: SesKuyrugu,
    gunluk_acik: bool,
    bildirim: String,
}

impl DemoDurumu {
    fn yeni(kurulum: SahneKurulumu, kimlikler: MaceraKimlikleri) -> (Dunya, Self) {
        let SahneKurulumu {
            dunya,
            fizik,
            oyuncu,
            merkez,
            muhafiz,
            piramitler,
        } = kurulum;
        let mut savas = SavasDunyasi::yeni();
        savas.savasci_ekle(
            oyuncu,
            Savasci::yeni(
                Takim::Oyuncu,
                100.0,
                24.0,
                2.1,
                std::time::Duration::from_millis(520),
            ),
        );
        savas.savasci_ekle(
            muhafiz,
            Savasci::yeni(
                Takim::Dusman,
                120.0,
                11.0,
                1.8,
                std::time::Duration::from_millis(900),
            ),
        );
        let durum = Self {
            fizik,
            oyuncu,
            merkez,
            muhafiz,
            piramitler,
            kamera_yatay: 0.0,
            kamera_dikey: 0.35,
            kayitlar: KayitYoneticisi::yeni(KAYIT_KLASORU, 3),
            kimlikler,
            son_gorev_asamasi: GorevAsamasi::Kilitli,
            savas,
            muhafiz_ajani: BasitAjan::yeni(),
            sesler: SesKuyrugu::yeni(),
            gunluk_acik: false,
            bildirim: "Gözcü Aras'ı bul.".to_owned(),
        };
        (dunya, durum)
    }

    fn guncelle(
        &mut self,
        girdi: &Girdi,
        zaman: &Zaman,
        dunya: &mut Dunya,
        macera: &mut Macera,
    ) -> OyunAkisi {
        let kare_saniyesi = zaman.kare_saniyesi().min(0.05);
        self.kamera_acisini_guncelle(girdi, kare_saniyesi);
        let yon = if diyalog_acik_mi(macera) {
            Vektor3::SIFIR
        } else {
            self.hareket_yonu(girdi)
        };
        let oyuncu_konumu = self.oyuncuyu_guncelle(girdi, zaman, dunya, yon);

        if let Err(hata) = macera.alanlari_guncelle(oyuncu_konumu) {
            eprintln!("Alan sistemi hatası: {hata}");
        }
        self.macera_girdisini_isle(girdi, dunya, macera, oyuncu_konumu);
        self.savasi_guncelle(girdi, zaman, dunya, macera, oyuncu_konumu);
        self.sahne_gecisini_uygula(dunya, macera);
        self.gorev_degisimini_yazdir(macera);
        self.sahneyi_canlandir(dunya, kare_saniyesi);
        self.kamerayi_yerlestir(dunya);
        let oyuncu_can = self
            .savas
            .savasci(self.oyuncu)
            .map_or(0.0, |s| s.can.oran());
        let muhafiz_can = self
            .savas
            .savasci(self.muhafiz)
            .map_or(0.0, |s| s.can.oran());
        arayuzu_guncelle(
            dunya,
            macera,
            &self.kimlikler,
            oyuncu_konumu,
            self.gunluk_acik,
            oyuncu_can,
            muhafiz_can,
            &self.bildirim,
        );

        if girdi.bu_kare_basildi_mi(Tus::Kacis) {
            OyunAkisi::Kapat
        } else {
            OyunAkisi::DevamEt
        }
    }

    fn savasi_guncelle(
        &mut self,
        girdi: &Girdi,
        zaman: &Zaman,
        dunya: &mut Dunya,
        macera: &mut Macera,
        oyuncu_konumu: Vektor3,
    ) {
        self.savas.guncelle(zaman.kare_suresi());
        let muhafiz_canli = self
            .savas
            .savasci(self.muhafiz)
            .is_some_and(|s| s.can.canli_mi());
        let muhafiz_konumu = dunya
            .varlik(self.muhafiz)
            .map_or(Vektor3::SIFIR, |v| v.donusumu3b().konum);
        let mesafe = (muhafiz_konumu - oyuncu_konumu).uzunluk();
        if girdi.bu_kare_basildi_mi(Tus::F) && muhafiz_canli {
            if self.savas.saldir(self.oyuncu, self.muhafiz, mesafe) {
                self.bildirim = "Taş Muhafız'a saldırdın.".to_owned();
                self.sesler
                    .ekle(SesOlayi::dunya("kilic-vurus", muhafiz_konumu, 0.9));
            } else {
                self.bildirim = "Saldırı için hedefe yaklaş.".to_owned();
            }
        }
        match self
            .muhafiz_ajani
            .karar(muhafiz_konumu, oyuncu_konumu, muhafiz_canli)
        {
            AjanKarari::Bekle => {}
            AjanKarari::TakipEt { yon, hiz } => {
                if let Some(v) = dunya.varlik_mut(self.muhafiz) {
                    v.donusumu3b_mut().tasi(yon * hiz * zaman.kare_saniyesi());
                }
            }
            AjanKarari::Saldir => {
                if self.savas.saldir(self.muhafiz, self.oyuncu, mesafe) {
                    self.bildirim = "Taş Muhafız sana vurdu!".to_owned();
                    self.sesler
                        .ekle(SesOlayi::dunya("tas-vurus", oyuncu_konumu, 0.8));
                }
            }
        }
        for olay in self.savas.olaylari_al() {
            if let SavasOlayi::Yenildi { varlik, .. } = olay {
                if varlik == self.muhafiz {
                    if let Some(v) = dunya.varlik_mut(self.muhafiz) {
                        v.etkinlestir(false);
                    }
                    let _ = macera.olay_yayinla(OyunOlayi::DusmanYenildi {
                        dusman: "Taş Muhafız".to_owned(),
                    });
                    self.bildirim = "Taş Muhafız yenildi. Kadim mühür çözüldü!".to_owned();
                    self.sesler.ekle(SesOlayi::ekran("muhafiz-yenildi", 1.0));
                } else if varlik == self.oyuncu {
                    self.bildirim = "Yenildin. R ile kontrol noktasına dön.".to_owned();
                }
            }
        }
        for ses in self.sesler.olaylari_al() {
            println!("Ses olayı: {} / şiddet {:.2}", ses.ses, ses.siddet);
        }
    }

    fn kamera_acisini_guncelle(&mut self, girdi: &Girdi, kare_saniyesi: f32) {
        let fare = girdi.fare_hareketi();
        self.kamera_yatay -= fare.x * FARE_HASSASIYETI;
        self.kamera_dikey -= fare.y * FARE_HASSASIYETI;
        self.klavye_kamerasini_guncelle(girdi, kare_saniyesi);
        self.kamera_dikey = self.kamera_dikey.clamp(-0.65, 1.05);
    }

    fn klavye_kamerasini_guncelle(&mut self, girdi: &Girdi, kare_saniyesi: f32) {
        let hareket = KLAVYE_KAMERA_HIZI * kare_saniyesi;
        if girdi.basili_mi(Tus::Sol) {
            self.kamera_yatay -= hareket;
        }
        if girdi.basili_mi(Tus::Sag) {
            self.kamera_yatay += hareket;
        }
        if girdi.basili_mi(Tus::Yukari) {
            self.kamera_dikey += hareket;
        }
        if girdi.basili_mi(Tus::Asagi) {
            self.kamera_dikey -= hareket;
        }
    }

    fn hareket_yonu(&self, girdi: &Girdi) -> Vektor3 {
        let ileri = Vektor3::yeni(-self.kamera_yatay.sin(), 0.0, -self.kamera_yatay.cos());
        let sag = Vektor3::yeni(self.kamera_yatay.cos(), 0.0, -self.kamera_yatay.sin());
        let mut yon = Vektor3::SIFIR;
        if girdi.basili_mi(Tus::W) {
            yon += ileri;
        }
        if girdi.basili_mi(Tus::S) {
            yon -= ileri;
        }
        if girdi.basili_mi(Tus::A) {
            yon -= sag;
        }
        if girdi.basili_mi(Tus::D) {
            yon += sag;
        }
        yon.birim()
    }

    fn oyuncuyu_guncelle(
        &mut self,
        girdi: &Girdi,
        zaman: &Zaman,
        dunya: &mut Dunya,
        yon: Vektor3,
    ) -> Vektor3 {
        {
            let govde = self
                .fizik
                .govde_mut(self.oyuncu)
                .expect("Oyuncu fizik gövdesi oyun boyunca kalmalı.");
            govde.yatay_hizi_ayarla(yon * OYUNCU_HIZI);
            if girdi.bu_kare_basildi_mi(Tus::Bosluk) {
                govde.ziplat(ZIPLAMA_HIZI);
            }
        }
        self.fizik.guncelle(dunya, zaman.kare_suresi());
        let oyuncu = dunya
            .varlik_mut(self.oyuncu)
            .expect("Oyuncu varlığı oyun boyunca kalmalı.");
        oyuncu.donusumu3b_mut().donus_radyan.y = self.kamera_yatay;
        oyuncu.donusumu3b().konum
    }

    fn macera_girdisini_isle(
        &mut self,
        girdi: &Girdi,
        dunya: &mut Dunya,
        macera: &mut Macera,
        oyuncu_konumu: Vektor3,
    ) {
        if diyalog_girdisini_isle(girdi, macera) {
            return;
        }
        if girdi.bu_kare_basildi_mi(Tus::E) {
            self.etkilesimi_isle(macera, oyuncu_konumu);
        }
        self.kayit_girdisini_isle(girdi, dunya, macera);
        Self::kontrol_girdisini_isle(girdi, macera);
        if girdi.bu_kare_basildi_mi(Tus::Sekme) {
            self.gunluk_acik = !self.gunluk_acik;
            self.durum_yazdir(macera);
        }
    }

    fn kayit_girdisini_isle(&mut self, girdi: &Girdi, dunya: &mut Dunya, macera: &mut Macera) {
        if girdi.bu_kare_basildi_mi(Tus::F5) {
            match self.kayitlar.kaydet(0, macera) {
                Ok(yol) => println!("Oyun kaydedildi: {}", yol.display()),
                Err(hata) => eprintln!("Kayıt başarısız: {hata}"),
            }
        }
        if girdi.bu_kare_basildi_mi(Tus::F9) {
            match self.kayitlar.yukle(0, macera) {
                Ok(yol) => {
                    println!("Oyun yüklendi: {}", yol.display());
                    self.yuklenen_konuma_tasi(dunya, macera);
                    self.durum_yazdir(macera);
                }
                Err(hata) => eprintln!("Yükleme başarısız: {hata}"),
            }
        }
    }

    fn kontrol_girdisini_isle(girdi: &Girdi, macera: &mut Macera) {
        if !girdi.bu_kare_basildi_mi(Tus::R) {
            return;
        }
        match macera.kontrol_noktasina_don() {
            Ok(()) => println!("Son kontrol noktasına dönüş istendi."),
            Err(hata) => eprintln!("Kontrol noktasına dönülemedi: {hata}"),
        }
    }

    fn etkilesimi_isle(&self, macera: &mut Macera, oyuncu_konumu: Vektor3) {
        let Some(yakin) = macera.yakin_etkilesim(oyuncu_konumu) else {
            println!("Yakında kullanılabilir bir etkileşim yok.");
            return;
        };
        println!("Etkileşim: {}", yakin.ileti);
        match macera.etkiles(&yakin.kimlik, oyuncu_konumu) {
            Ok(()) => {
                diyalogu_yazdir(macera);
                println!(
                    "Mühür parçaları: {}/3",
                    macera.envanter().miktar(&self.kimlikler.muhur)
                );
            }
            Err(hata) => eprintln!("Etkileşim uygulanamadı: {hata}"),
        }
    }

    fn sahne_gecisini_uygula(&mut self, dunya: &mut Dunya, macera: &mut Macera) {
        let gecis = match macera.sahne_gecisini_al() {
            Ok(gecis) => gecis,
            Err(hata) => {
                eprintln!("Sahne geçişi uygulanamadı: {hata}");
                return;
            }
        };
        let Some(gecis) = gecis else {
            return;
        };
        let konum = gecis
            .giris_noktasi
            .as_deref()
            .and_then(|giris| macera.etkin_giris_konumu(giris))
            .unwrap_or(Vektor3::yeni(0.0, 2.5, 3.0));
        self.oyuncuyu_tasi(dunya, konum);
        println!(
            "Sahne geçişi: {:?} → {} / giriş: {}",
            gecis.onceki.as_ref().map(SahneKimligi::deger),
            gecis.hedef.deger(),
            gecis.giris_noktasi.as_deref().unwrap_or("varsayılan")
        );
    }

    fn yuklenen_konuma_tasi(&mut self, dunya: &mut Dunya, macera: &Macera) {
        let kontrol = macera.kontrol_noktasi().cloned();
        let giris = kontrol
            .as_ref()
            .and_then(|kontrol| kontrol.giris_noktasi.as_deref())
            .unwrap_or_else(|| self.varsayilan_giris(macera));
        let konum = macera
            .etkin_giris_konumu(giris)
            .unwrap_or(Vektor3::yeni(0.0, 2.5, 3.0));
        self.oyuncuyu_tasi(dunya, konum);
    }

    fn varsayilan_giris(&self, macera: &Macera) -> &'static str {
        if macera.etkin_sahne() == Some(&self.kimlikler.mahzen) {
            "sunak"
        } else {
            "baslangic"
        }
    }

    fn oyuncuyu_tasi(&mut self, dunya: &mut Dunya, konum: Vektor3) {
        dunya
            .varlik_mut(self.oyuncu)
            .expect("Oyuncu varlığı oyun boyunca kalmalı.")
            .donusumu3b_mut()
            .konum = konum;
        self.fizik
            .govde_mut(self.oyuncu)
            .expect("Oyuncu fizik gövdesi oyun boyunca kalmalı.")
            .hizi_ayarla(Vektor3::SIFIR);
    }

    fn gorev_degisimini_yazdir(&mut self, macera: &Macera) {
        let asama = macera.gorev_asamasi(&self.kimlikler.gorev);
        if asama == self.son_gorev_asamasi {
            return;
        }
        self.son_gorev_asamasi = asama;
        println!("Görev aşaması değişti: {asama:?}");
        self.durum_yazdir(macera);
    }

    fn durum_yazdir(&self, macera: &Macera) {
        let ilerleme = macera.gorev_ilerlemesi(&self.kimlikler.gorev);
        let adim = ilerleme.map_or(0, GorevIlerlemesi::adim);
        let hedefler = ilerleme.map_or(&[][..], GorevIlerlemesi::hedefler);
        let oynama = macera.oynama_suresi_milisaniye();
        println!("\n--- KAYIP MÜHÜR GÜNLÜĞÜ ---");
        println!(
            "Sahne: {}",
            macera.etkin_sahne().map_or("yok", SahneKimligi::deger)
        );
        println!(
            "Görev: {:?} / adım {} / hedefler {:?}",
            macera.gorev_asamasi(&self.kimlikler.gorev),
            adim + 1,
            hedefler
        );
        println!(
            "Pusula: {} — mühür parçası: {}/3",
            macera.envanter().miktar(&self.kimlikler.pusula),
            macera.envanter().miktar(&self.kimlikler.muhur)
        );
        println!(
            "Unvan: {} — bölüm tamamlandı: {} — oynama: {}.{:03} sn",
            macera.durum().metin("oyuncu_unvani").unwrap_or("Yolcu"),
            macera.durum().bayrak("bolum_tamamlandi"),
            oynama / 1_000,
            oynama % 1_000,
        );
        println!("--------------------------\n");
    }

    fn sahneyi_canlandir(&self, dunya: &mut Dunya, kare_saniyesi: f32) {
        dunya
            .varlik_mut(self.merkez)
            .expect("Merkez küp oyun boyunca kalmalı.")
            .donusumu3b_mut()
            .dondur(Vektor3::yeni(
                0.25 * kare_saniyesi,
                0.7 * kare_saniyesi,
                0.15 * kare_saniyesi,
            ));
        for (sira, kimlik) in self.piramitler.iter().copied().enumerate() {
            let yon = if sira & 1 == 0 { 1.0 } else { -1.0 };
            dunya
                .varlik_mut(kimlik)
                .expect("Piramit varlığı oyun boyunca kalmalı.")
                .donusumu3b_mut()
                .dondur(Vektor3::YUKARI * (yon * 0.45 * kare_saniyesi));
        }
    }

    fn kamerayi_yerlestir(&self, dunya: &mut Dunya) {
        let oyuncu_konumu = dunya
            .varlik(self.oyuncu)
            .expect("Oyuncu varlığı oyun boyunca kalmalı.")
            .donusumu3b()
            .konum;
        let yatay_uzaklik = self.kamera_dikey.cos() * KAMERA_UZAKLIGI;
        let kamera_konumu = oyuncu_konumu
            + Vektor3::yeni(
                self.kamera_yatay.sin() * yatay_uzaklik,
                1.2 + self.kamera_dikey.sin() * KAMERA_UZAKLIGI,
                self.kamera_yatay.cos() * yatay_uzaklik,
            );
        let kamera = dunya.kamera3b_mut();
        kamera.konum = kamera_konumu;
        kamera.hedef = oyuncu_konumu + Vektor3::YUKARI * 0.3;
    }
}

fn diyalog_acik_mi(macera: &Macera) -> bool {
    macera.diyalog_gorunumu().ok().flatten().is_some()
}

fn diyalog_girdisini_isle(girdi: &Girdi, macera: &mut Macera) -> bool {
    let gorunum = match macera.diyalog_gorunumu() {
        Ok(Some(gorunum)) => gorunum,
        Ok(None) => return false,
        Err(hata) => {
            eprintln!("Diyalog okunamadı: {hata}");
            return true;
        }
    };
    let secim = [
        (Tus::Sayi1, 0_usize),
        (Tus::Sayi2, 1_usize),
        (Tus::Sayi3, 2_usize),
    ]
    .into_iter()
    .find(|(tus, _)| girdi.bu_kare_basildi_mi(*tus));
    if let Some((_, sira)) = secim {
        diyalog_secenegini_uygula(macera, &gorunum.secenekler, sira);
    } else if gorunum.ilerletilebilir && girdi.bu_kare_basildi_mi(Tus::Enter) {
        match macera.diyalog_ilerlet() {
            Ok(()) => diyalogu_yazdir(macera),
            Err(hata) => eprintln!("Diyalog ilerletilemedi: {hata}"),
        }
    }
    true
}

fn diyalog_secenegini_uygula(
    macera: &mut Macera,
    secenekler: &[tgame::onsoz::DiyalogSecenegiGorunumu],
    sira: usize,
) {
    let Some(secenek) = secenekler.get(sira) else {
        return;
    };
    match macera.diyalog_sec(&secenek.kimlik) {
        Ok(()) => diyalogu_yazdir(macera),
        Err(hata) => eprintln!("Diyalog seçimi uygulanamadı: {hata}"),
    }
}

fn diyalogu_yazdir(macera: &Macera) {
    match macera.diyalog_gorunumu() {
        Ok(Some(gorunum)) => {
            println!("\n{}: {}", gorunum.konusan, gorunum.metin);
            for (sira, secenek) in gorunum.secenekler.iter().enumerate() {
                println!("  {}. {}", sira + 1, secenek.metin);
            }
            if gorunum.ilerletilebilir {
                println!("  [Enter] devam et");
            }
        }
        Ok(None) => println!("Diyalog sona erdi."),
        Err(hata) => eprintln!("Diyalog gösterilemedi: {hata}"),
    }
}

fn oynanis_yardimi_yazdir() {
    println!(
        "\nTgame: Kayıp Mühür\n\
         WASD hareket | Fare/yön tuşları kamera | Boşluk zıpla\n\
         E etkileşim | 1-3 diyalog seçimi | Enter diyalog ilerlet\n\
         Tab görev günlüğü | F5 kaydet | F9 yükle | R kontrol noktası | Esc çık\n\
         İlk hedef: başlangıç noktasının yakınındaki mavi Gözcü Aras ile konuş.\n"
    );
}

fn main() -> OyunSonucu {
    let kimlikler = MaceraKimlikleri::yeni();
    let macera = macerayi_olustur(&kimlikler)?;
    let (dunya, mut durum) = DemoDurumu::yeni(sahneyi_olustur()?, kimlikler);
    oynanis_yardimi_yazdir();

    Oyun::yeni("Tgame: Kayıp Mühür Macerası")
        .cozunurluk(960, 640)
        .mod_klasoru("modlar")
        .sahne_ekle(Sahne::yeni("Köy Meydanı"))
        .sahne_ekle(Sahne::yeni("Kadim Mahzen"))
        .dunya(dunya)
        .macera(macera)
        .her_kare_macera(move |girdi, zaman, dunya, macera| {
            durum.guncelle(girdi, zaman, dunya, macera)
        })
        .calistir()
}

#[cfg(test)]
mod testler {
    use super::{MaceraKimlikleri, macerayi_olustur};
    use tgame::onsoz::{GorevAsamasi, Vektor3};

    #[test]
    fn ornek_macera_bastan_sona_tamamlanir() {
        let kimlikler = MaceraKimlikleri::yeni();
        let mut macera = macerayi_olustur(&kimlikler).expect("Macera içeriği kurulmalı.");
        macera
            .diyalog_baslat(&kimlikler.gozcu_diyalogu)
            .expect("Gözcü diyaloğu başlamalı.");
        macera.diyalog_sec("kabul").expect("Görev kabul edilmeli.");
        macera.diyalog_ilerlet().expect("Diyalog bitmeli.");
        assert_eq!(macera.gorev_asamasi(&kimlikler.gorev), GorevAsamasi::Etkin);

        for _ in 0..3 {
            macera
                .esya_ekle(&kimlikler.muhur, 1)
                .expect("Mühür parçası eklenmeli.");
        }
        macera
            .alanlari_guncelle(Vektor3::yeni(0.0, 0.5, -4.4))
            .expect("Tapınak alanı çalışmalı.");
        macera
            .olay_yayinla(tgame::onsoz::OyunOlayi::DusmanYenildi {
                dusman: "Taş Muhafız".to_owned(),
            })
            .expect("Muhafız olayı işlenmeli.");

        assert_eq!(
            macera.gorev_asamasi(&kimlikler.gorev),
            GorevAsamasi::Tamamlandi
        );
        assert!(macera.durum().bayrak("bolum_tamamlandi"));
        assert_eq!(macera.durum().metin("oyuncu_unvani"), Some("Mührün Varisi"));
    }
}
