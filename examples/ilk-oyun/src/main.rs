use tgame::onsoz::{
    AlanKimligi, AlanTetikleyicisi, DiyalogDugumu, DiyalogKimligi, DiyalogSecenegi,
    DiyalogTanimi, Donusum3B, Dunya, EsyaKimligi, EsyaTanimi, EtkilesimKimligi,
    EtkilesimNoktasi, Eylem, FizikDunyasi, FizikGovdesi, Girdi, GorevAdimi, GorevAsamasi,
    GorevHedefi, GorevKimligi, GorevTanimi, Kamera3B, KayitYoneticisi,
    KontrolNoktasiKimligi, Kosul, KuralKimligi, KutuAlan, Macera, MalzemeKimligi,
    MalzemeVerisi, MeshKimligi, ModelVerisi, OlayFiltresi, OlayKurali, Oyun, OyunAkisi,
    OyunHatasi, OyunOlayi, OyunSonucu, Renk, Sahne, SahneKimligi, SahneTanimi,
    Tekrarlama, Tus, Varlik, VarlikKimligi, Vektor3, Zaman,
};

const OYUNCU_HIZI: f32 = 4.8;
const ZIPLAMA_HIZI: f32 = 6.2;
const FARE_HASSASIYETI: f32 = 0.0025;
const KLAVYE_KAMERA_HIZI: f32 = 1.5;
const KAMERA_UZAKLIGI: f32 = 7.5;
const OYUNCU_OLCEGI: Vektor3 = Vektor3::yeni(0.75, 0.75, 0.75);
const KAYIT_KLASORU: &str = "kayitlar/kayip-muhur";

#[derive(Clone)]
struct MaceraKimlikleri {
    pusula: EsyaKimligi,
    muhur: EsyaKimligi,
    gorev: GorevKimligi,
    gozcu_diyalogu: DiyalogKimligi,
    koy: SahneKimligi,
    mahzen: SahneKimligi,
    tapinak_alani: AlanKimligi,
    gozcu: EtkilesimKimligi,
    muhafiz: EtkilesimKimligi,
}

impl MaceraKimlikleri {
    fn yeni() -> Self {
        Self {
            pusula: EsyaKimligi::from("sis_pusulasi"),
            muhur: EsyaKimligi::from("muhur_parcasi"),
            gorev: GorevKimligi::from("kayip_muhur"),
            gozcu_diyalogu: DiyalogKimligi::from("gozcu_aras"),
            koy: SahneKimligi::from("koy_meydani"),
            mahzen: SahneKimligi::from("kadim_mahzen"),
            tapinak_alani: AlanKimligi::from("tapinak_esigi"),
            gozcu: EtkilesimKimligi::from("gozcu_ile_konus"),
            muhafiz: EtkilesimKimligi::from("tas_muhafiz"),
        }
    }
}

struct SahneKurulumu {
    dunya: Dunya,
    fizik: FizikDunyasi,
    oyuncu: VarlikKimligi,
    merkez: VarlikKimligi,
    piramitler: Vec<VarlikKimligi>,
}

struct DemoDurumu {
    fizik: FizikDunyasi,
    oyuncu: VarlikKimligi,
    merkez: VarlikKimligi,
    piramitler: Vec<VarlikKimligi>,
    kamera_yatay: f32,
    kamera_dikey: f32,
    kayitlar: KayitYoneticisi,
    kimlikler: MaceraKimlikleri,
    son_gorev_asamasi: GorevAsamasi,
}

impl DemoDurumu {
    fn yeni(kurulum: SahneKurulumu, kimlikler: MaceraKimlikleri) -> (Dunya, Self) {
        let SahneKurulumu {
            dunya,
            fizik,
            oyuncu,
            merkez,
            piramitler,
        } = kurulum;
        (
            dunya,
            Self {
                fizik,
                oyuncu,
                merkez,
                piramitler,
                kamera_yatay: 0.0,
                kamera_dikey: 0.35,
                kayitlar: KayitYoneticisi::yeni(KAYIT_KLASORU, 3),
                kimlikler,
                son_gorev_asamasi: GorevAsamasi::Kilitli,
            },
        )
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
        let diyalog_acik = macera
            .diyalog_gorunumu()
            .ok()
            .flatten()
            .is_some();
        let yon = if diyalog_acik {
            Vektor3::SIFIR
        } else {
            self.hareket_yonu(girdi)
        };
        let oyuncu_konumu = self.oyuncuyu_guncelle(girdi, zaman, dunya, yon);

        if let Err(hata) = macera.alanlari_guncelle(oyuncu_konumu) {
            eprintln!("Alan sistemi hatası: {hata}");
        }
        self.macera_girdisini_isle(girdi, dunya, macera, oyuncu_konumu);
        self.sahne_gecisini_uygula(dunya, macera);
        self.gorev_degisimini_yazdir(macera);
        self.sahneyi_canlandir(dunya, kare_saniyesi);
        self.kamerayi_yerlestir(dunya);

        if girdi.bu_kare_basildi_mi(Tus::Kacis) {
            OyunAkisi::Kapat
        } else {
            OyunAkisi::DevamEt
        }
    }

    fn kamera_acisini_guncelle(&mut self, girdi: &Girdi, kare_saniyesi: f32) {
        let fare = girdi.fare_hareketi();
        self.kamera_yatay -= fare.x * FARE_HASSASIYETI;
        self.kamera_dikey -= fare.y * FARE_HASSASIYETI;

        if girdi.basili_mi(Tus::Sol) {
            self.kamera_yatay -= KLAVYE_KAMERA_HIZI * kare_saniyesi;
        }
        if girdi.basili_mi(Tus::Sag) {
            self.kamera_yatay += KLAVYE_KAMERA_HIZI * kare_saniyesi;
        }
        if girdi.basili_mi(Tus::Yukari) {
            self.kamera_dikey += KLAVYE_KAMERA_HIZI * kare_saniyesi;
        }
        if girdi.basili_mi(Tus::Asagi) {
            self.kamera_dikey -= KLAVYE_KAMERA_HIZI * kare_saniyesi;
        }
        self.kamera_dikey = self.kamera_dikey.clamp(-0.65, 1.05);
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
            let oyuncu_govdesi = self
                .fizik
                .govde_mut(self.oyuncu)
                .expect("Oyuncu fizik gövdesi oyun boyunca kalmalı.");
            oyuncu_govdesi.yatay_hizi_ayarla(yon * OYUNCU_HIZI);
            if girdi.bu_kare_basildi_mi(Tus::Bosluk) {
                oyuncu_govdesi.ziplat(ZIPLAMA_HIZI);
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
        if self.diyalog_girdisini_isle(girdi, macera) {
            return;
        }

        if girdi.bu_kare_basildi_mi(Tus::E) {
            self.etkilesimi_isle(macera, oyuncu_konumu);
        }
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
        if girdi.bu_kare_basildi_mi(Tus::R) {
            match macera.kontrol_noktasina_don() {
                Ok(()) => println!("Son kontrol noktasına dönüş istendi."),
                Err(hata) => eprintln!("Kontrol noktasına dönülemedi: {hata}"),
            }
        }
        if girdi.bu_kare_basildi_mi(Tus::Sekme) {
            self.durum_yazdir(macera);
        }
    }

    fn diyalog_girdisini_isle(&self, girdi: &Girdi, macera: &mut Macera) -> bool {
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
            if let Some(secenek) = gorunum.secenekler.get(sira) {
                let kimlik = secenek.kimlik.clone();
                match macera.diyalog_sec(&kimlik) {
                    Ok(()) => diyalogu_yazdir(macera),
                    Err(hata) => eprintln!("Diyalog seçimi uygulanamadı: {hata}"),
                }
            }
        } else if gorunum.ilerletilebilir && girdi.bu_kare_basildi_mi(Tus::Enter) {
            match macera.diyalog_ilerlet() {
                Ok(()) => diyalogu_yazdir(macera),
                Err(hata) => eprintln!("Diyalog ilerletilemedi: {hata}"),
            }
        }
        true
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
            gecis.onceki.as_ref().map(|sahne| sahne.deger()),
            gecis.hedef.deger(),
            gecis.giris_noktasi.as_deref().unwrap_or("varsayılan")
        );
    }

    fn yuklenen_konuma_tasi(&mut self, dunya: &mut Dunya, macera: &Macera) {
        let kontrol = macera.kontrol_noktasi().cloned();
        let giris = kontrol
            .as_ref()
            .and_then(|kontrol| kontrol.giris_noktasi.as_deref())
            .or_else(|| {
                if macera.etkin_sahne() == Some(&self.kimlikler.mahzen) {
                    Some("sunak")
                } else {
                    Some("baslangic")
                }
            });
        let konum = giris
            .and_then(|ad| macera.etkin_giris_konumu(ad))
            .unwrap_or(Vektor3::yeni(0.0, 2.5, 3.0));
        self.oyuncuyu_tasi(dunya, konum);
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
        let adim = ilerleme.map_or(0, |ilerleme| ilerleme.adim());
        let hedefler = ilerleme.map_or(&[][..], |ilerleme| ilerleme.hedefler());
        println!("\n--- KAYIP MÜHÜR GÜNLÜĞÜ ---");
        println!("Sahne: {}", macera.etkin_sahne().map_or("yok", SahneKimligi::deger));
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
            "Unvan: {} — bölüm tamamlandı: {} — oynama: {:.1} sn",
            macera.durum().metin("oyuncu_unvani").unwrap_or("Yolcu"),
            macera.durum().bayrak("bolum_tamamlandi"),
            macera.oynama_suresi_milisaniye() as f64 / 1_000.0
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

fn macerayi_olustur(kimlikler: &MaceraKimlikleri) -> OyunSonucu<Macera> {
    let mut macera = Macera::yeni();
    macera.esya_tanimla(
        EsyaTanimi::yeni(kimlikler.pusula.clone(), "Sis Pusulası")
            .aciklama("Kadim tapınağın yönünü gösteren, kaybedilemeyen görev eşyası.")
            .gorev_esyasi(),
    );
    macera.esya_tanimla(
        EsyaTanimi::yeni(kimlikler.muhur.clone(), "Mühür Parçası")
            .aciklama("Tapınak kapısını açmak için üç parça gerekir.")
            .yigin_siniri(3),
    );

    macera.gorev_tanimla(GorevTanimi::yeni(
        kimlikler.gorev.clone(),
        "Kayıp Mühür",
        vec![
            GorevAdimi::yeni(
                "Üç mühür parçasını bul",
                vec![GorevHedefi::yeni(
                    "Mühür parçalarını topla",
                    OlayFiltresi::EsyaEklendi(kimlikler.muhur.clone()),
                    3,
                )],
            ),
            GorevAdimi::yeni(
                "Tapınak eşiğine ulaş",
                vec![GorevHedefi::yeni(
                    "Tapınak alanına gir",
                    OlayFiltresi::AlanaGirdi(kimlikler.tapinak_alani.clone()),
                    1,
                )],
            ),
            GorevAdimi::yeni(
                "Taş Muhafız'ı yen",
                vec![GorevHedefi::yeni(
                    "Muhafızı etkisiz hâle getir",
                    OlayFiltresi::DusmanYenildi("Taş Muhafız".to_owned()),
                    1,
                )],
            ),
        ],
    ));

    macera.diyalog_tanimla(DiyalogTanimi::yeni(
        kimlikler.gozcu_diyalogu.clone(),
        "selam",
        vec![
            DiyalogDugumu::yeni(
                "selam",
                "Gözcü Aras",
                "Sis tapınağı yeniden uyandı. Mührün üç parçasını bulabilir misin?",
            )
            .secenekler(vec![
                DiyalogSecenegi::yeni("kabul", "Görevi kabul ediyorum.")
                    .eylemler(vec![
                        Eylem::GorevBaslat(kimlikler.gorev.clone()),
                        Eylem::EsyaEkle {
                            esya: kimlikler.pusula.clone(),
                            miktar: 1,
                        },
                        Eylem::BayrakAyarla {
                            ad: "aras_ile_anlasildi".to_owned(),
                            deger: true,
                        },
                    ])
                    .sonraki("yol"),
                DiyalogSecenegi::yeni("reddet", "Şimdilik hazır değilim."),
            ]),
            DiyalogDugumu::yeni(
                "yol",
                "Gözcü Aras",
                "Pusula parçalara yaklaştığında titreşecek. Üçünü de alınca kuzeydeki tapınağa git.",
            ),
        ],
    ));

    macera.sahne_tanimla(
        SahneTanimi::yeni(kimlikler.koy.clone(), "Sisli Köy Meydanı")
            .giris_noktasi("baslangic", Vektor3::yeni(0.0, 2.5, 3.0)),
    );
    macera.sahne_tanimla(
        SahneTanimi::yeni(kimlikler.mahzen.clone(), "Kadim Mahzen")
            .giris_noktasi("sunak", Vektor3::yeni(0.0, 2.5, -1.2)),
    );
    macera.baslangic_sahnesi_ayarla(&kimlikler.koy)?;

    macera.etkilesim_tanimla(
        EtkilesimNoktasi::yeni(
            kimlikler.gozcu.clone(),
            "Gözcü Aras ile konuş",
            Vektor3::yeni(0.0, 0.0, 2.0),
            2.2,
        )
        .eylemler(vec![Eylem::DiyalogBaslat(
            kimlikler.gozcu_diyalogu.clone(),
        )])
        .tekrarlama(Tekrarlama::HerZaman),
    );

    for (sira, konum) in [
        Vektor3::yeni(3.2, 0.0, 1.0),
        Vektor3::yeni(-3.2, 0.0, -1.0),
        Vektor3::yeni(2.4, 0.0, -3.2),
    ]
    .into_iter()
    .enumerate()
    {
        macera.etkilesim_tanimla(
            EtkilesimNoktasi::yeni(
                EtkilesimKimligi::from(format!("muhur_parcasi_{}", sira + 1)),
                format!("{}. mühür parçasını al", sira + 1),
                konum,
                1.6,
            )
            .kosullar(vec![Kosul::GorevAsamasi {
                gorev: kimlikler.gorev.clone(),
                asama: GorevAsamasi::Etkin,
            }])
            .eylemler(vec![Eylem::EsyaEkle {
                esya: kimlikler.muhur.clone(),
                miktar: 1,
            }]),
        );
    }

    macera.alan_tanimla(
        AlanTetikleyicisi::yeni(
            kimlikler.tapinak_alani.clone(),
            KutuAlan::yeni(
                Vektor3::yeni(0.0, 0.5, -4.4),
                Vektor3::yeni(2.2, 2.5, 1.5),
            ),
        )
        .kosullar(vec![
            Kosul::EsyaEnAz {
                esya: kimlikler.muhur.clone(),
                miktar: 3,
            },
            Kosul::GorevAsamasi {
                gorev: kimlikler.gorev.clone(),
                asama: GorevAsamasi::Etkin,
            },
        ])
        .giris_eylemleri(vec![Eylem::BayrakAyarla {
            ad: "tapinak_uyandi".to_owned(),
            deger: true,
        }])
        .cikis_eylemleri(vec![Eylem::SayacDegistir {
            ad: "tapinaktan_cikis".to_owned(),
            fark: 1,
        }])
        .tekrarlama(Tekrarlama::HerZaman),
    );

    macera.etkilesim_tanimla(
        EtkilesimNoktasi::yeni(
            kimlikler.muhafiz.clone(),
            "Taş Muhafız'ın mührünü çöz",
            Vektor3::yeni(0.0, 0.0, -4.8),
            2.0,
        )
        .kosullar(vec![
            Kosul::Bayrak {
                ad: "tapinak_uyandi".to_owned(),
                deger: true,
            },
            Kosul::EsyaEnAz {
                esya: kimlikler.muhur.clone(),
                miktar: 3,
            },
        ])
        .eylemler(vec![
            Eylem::OlayYayinla(Box::new(OyunOlayi::DusmanYenildi {
                dusman: "Taş Muhafız".to_owned(),
            })),
            Eylem::KontrolNoktasiAyarla {
                kimlik: KontrolNoktasiKimligi::from("mahzen_sunagi"),
                sahne: kimlikler.mahzen.clone(),
                giris_noktasi: Some("sunak".to_owned()),
            },
            Eylem::SahneGecisiIste {
                sahne: kimlikler.mahzen.clone(),
                giris_noktasi: Some("sunak".to_owned()),
            },
        ]),
    );

    macera.kural_tanimla(OlayKurali::yeni(
        KuralKimligi::from("kayip_muhur_tamamlandi"),
        OlayFiltresi::GorevTamamlandi(kimlikler.gorev.clone()),
        vec![
            Eylem::BayrakAyarla {
                ad: "bolum_tamamlandi".to_owned(),
                deger: true,
            },
            Eylem::MetinAyarla {
                ad: "oyuncu_unvani".to_owned(),
                deger: "Mührün Varisi".to_owned(),
            },
        ],
    ));

    Ok(macera)
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

fn sahneyi_olustur() -> OyunSonucu<SahneKurulumu> {
    let mut dunya = Dunya::yeni_3b();
    let mut fizik = FizikDunyasi::yeni();
    dunya.kamera3b_ayarla(
        Kamera3B::yeni()
            .konum(Vektor3::yeni(0.0, 4.2, 8.0))
            .hedef(Vektor3::SIFIR)
            .kirpma(0.1, 250.0),
    );

    zemin_ekle(&mut dunya, &mut fizik);
    sutunlari_ekle(&mut dunya, &mut fizik);
    macera_isaretlerini_ekle(&mut dunya);
    let piramit_mesh = piramit_meshini_yukle(&mut dunya)?;
    let alternatif_malzeme =
        dunya.malzeme_ekle(MalzemeVerisi::yeni(Renk::yeni(0.08, 0.85, 1.0, 1.0)));
    let piramitler = piramitleri_ekle(&mut dunya, &mut fizik, piramit_mesh, alternatif_malzeme);
    gorunurluk_stres_sahnesi_ekle(&mut dunya, piramit_mesh);

    let oyuncu = dunya.varlik_ekle(
        Varlik::kup("Oyuncu", Renk::SARI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 2.5, 3.0))
                .olcek(OYUNCU_OLCEGI),
        ),
    );
    fizik.govde_ekle(FizikGovdesi::dinamik_kup(oyuncu, OYUNCU_OLCEGI));

    let merkez_olcegi = Vektor3::yeni(1.7, 1.7, 1.7);
    let merkez = dunya.varlik_ekle(
        Varlik::kup("Dönen Tapınak Çekirdeği", Renk::KIRMIZI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 1.35, 0.0))
                .olcek(merkez_olcegi)
                .donus(Vektor3::yeni(0.25, 0.35, 0.1)),
        ),
    );
    fizik.govde_ekle(FizikGovdesi::statik_kup(merkez, merkez_olcegi));

    Ok(SahneKurulumu {
        dunya,
        fizik,
        oyuncu,
        merkez,
        piramitler,
    })
}

fn macera_isaretlerini_ekle(dunya: &mut Dunya) {
    dunya.varlik_ekle(
        Varlik::kup("Gözcü Aras", Renk::MAVI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 0.15, 2.0))
                .olcek(Vektor3::yeni(0.7, 1.6, 0.7)),
        ),
    );
    for (sira, konum) in [
        Vektor3::yeni(3.2, 0.0, 1.0),
        Vektor3::yeni(-3.2, 0.0, -1.0),
        Vektor3::yeni(2.4, 0.0, -3.2),
    ]
    .into_iter()
    .enumerate()
    {
        let renk = [Renk::YESIL, Renk::MAVI, Renk::SARI][sira];
        dunya.varlik_ekle(
            Varlik::kup("Mühür Parçası", renk).donusum3b(
                Donusum3B::yeni()
                    .konum(konum + Vektor3::YUKARI * 0.25)
                    .olcek(Vektor3::yeni(0.35, 0.35, 0.35)),
            ),
        );
    }
    dunya.varlik_ekle(
        Varlik::kup("Taş Muhafız", Renk::KIRMIZI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 0.45, -4.8))
                .olcek(Vektor3::yeni(1.2, 2.2, 1.2)),
        ),
    );
}

fn piramit_meshini_yukle(dunya: &mut Dunya) -> OyunSonucu<MeshKimligi> {
    let model = ModelVerisi::gltf_yukle("assets/piramit.gltf")?;
    dunya
        .model_ekle(model)
        .first()
        .copied()
        .ok_or_else(|| OyunHatasi::yeni("Piramit modeli kayıtlı mesh üretmedi."))
}

fn piramitleri_ekle(
    dunya: &mut Dunya,
    fizik: &mut FizikDunyasi,
    mesh: MeshKimligi,
    alternatif_malzeme: MalzemeKimligi,
) -> Vec<VarlikKimligi> {
    let piramitler = [
        (Vektor3::yeni(-3.2, -0.58, -2.8), Renk::MAVI),
        (Vektor3::yeni(-1.6, -0.58, -4.2), Renk::YESIL),
        (Vektor3::yeni(1.6, -0.58, -4.2), Renk::KIRMIZI),
        (Vektor3::yeni(3.2, -0.58, -2.8), Renk::SARI),
        (Vektor3::yeni(-3.2, -0.58, 2.8), Renk::KIRMIZI),
        (Vektor3::yeni(-1.6, -0.58, 4.2), Renk::SARI),
        (Vektor3::yeni(1.6, -0.58, 4.2), Renk::MAVI),
        (Vektor3::yeni(3.2, -0.58, 2.8), Renk::YESIL),
    ];
    let goruntu_olcegi = Vektor3::yeni(0.65, 0.65, 0.65);
    let carpisma_olcegi = Vektor3::yeni(1.3, 1.17, 1.3);
    let mut kimlikler = Vec::with_capacity(piramitler.len());

    for (sira, (konum, renk)) in piramitler.into_iter().enumerate() {
        let varlik = if sira.is_multiple_of(3) {
            Varlik::mesh_malzemeli(
                "Alternatif Malzemeli Piramit",
                mesh,
                alternatif_malzeme,
                renk,
            )
        } else {
            Varlik::mesh("Dokulu glTF Piramit", mesh, renk)
        };
        let kimlik = dunya
            .varlik_ekle(varlik.donusum3b(Donusum3B::yeni().konum(konum).olcek(goruntu_olcegi)));
        kimlikler.push(kimlik);

        let engel = dunya.varlik_ekle(
            Varlik::yeni("Piramit Çarpışması")
                .donusum3b(Donusum3B::yeni().konum(konum + Vektor3::YUKARI * 0.585)),
        );
        fizik.govde_ekle(FizikGovdesi::statik_kup(engel, carpisma_olcegi));
    }

    kimlikler
}

fn gorunurluk_stres_sahnesi_ekle(dunya: &mut Dunya, mesh: MeshKimligi) {
    let olcek = Vektor3::yeni(0.35, 0.35, 0.35);
    for sira in 0_u16..256 {
        let sutun = f32::from(sira % 16);
        let satir = f32::from(sira / 16);
        dunya.varlik_ekle(
            Varlik::mesh("Frustum Dışı Piramit", mesh, Renk::BEYAZ).donusum3b(
                Donusum3B::yeni()
                    .konum(Vektor3::yeni(400.0 + sutun * 2.0, -0.58, -satir * 2.0))
                    .olcek(olcek),
            ),
        );
    }
}

fn zemin_ekle(dunya: &mut Dunya, fizik: &mut FizikDunyasi) {
    let koordinatlar = [-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let olcek = Vektor3::yeni(0.96, 0.12, 0.96);
    let mut acik_renk = false;

    for z in koordinatlar {
        for x in koordinatlar {
            let renk = if acik_renk {
                Renk::yeni(0.18, 0.24, 0.34, 1.0)
            } else {
                Renk::yeni(0.10, 0.14, 0.22, 1.0)
            };
            let kimlik = dunya.varlik_ekle(
                Varlik::kup("Zemin", renk).donusum3b(
                    Donusum3B::yeni()
                        .konum(Vektor3::yeni(x, -0.65, z))
                        .olcek(olcek),
                ),
            );
            fizik.govde_ekle(FizikGovdesi::statik_kup(kimlik, olcek));
            acik_renk = !acik_renk;
        }
        acik_renk = !acik_renk;
    }
}

fn sutunlari_ekle(dunya: &mut Dunya, fizik: &mut FizikDunyasi) {
    let sutunlar = [
        (Vektor3::yeni(-4.0, 0.4, -4.0), Renk::MAVI, 2.0),
        (Vektor3::yeni(4.0, 0.9, -4.0), Renk::YESIL, 3.0),
        (Vektor3::yeni(-4.0, 1.4, 4.0), Renk::KIRMIZI, 4.0),
        (Vektor3::yeni(4.0, 0.65, 4.0), Renk::SARI, 2.5),
        (Vektor3::yeni(-2.5, 0.15, 0.0), Renk::YESIL, 1.5),
        (Vektor3::yeni(2.5, 0.15, 0.0), Renk::MAVI, 1.5),
    ];

    for (konum, renk, yukseklik) in sutunlar {
        let olcek = Vektor3::yeni(0.8, yukseklik, 0.8);
        let kimlik = dunya.varlik_ekle(
            Varlik::kup("Sütun", renk).donusum3b(Donusum3B::yeni().konum(konum).olcek(olcek)),
        );
        fizik.govde_ekle(FizikGovdesi::statik_kup(kimlik, olcek));
    }
}

#[cfg(test)]
mod testler {
    use super::{MaceraKimlikleri, macerayi_olustur};
    use tgame::onsoz::{GorevAsamasi, OyunOlayi, Vektor3};

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
            .olay_yayinla(OyunOlayi::DusmanYenildi {
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
