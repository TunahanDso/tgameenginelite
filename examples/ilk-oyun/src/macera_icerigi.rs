use tgame::onsoz::{
    AlanKimligi, AlanTetikleyicisi, DiyalogDugumu, DiyalogKimligi, DiyalogSecenegi,
    DiyalogTanimi, EsyaKimligi, EsyaTanimi, EtkilesimKimligi, EtkilesimNoktasi, Eylem,
    GorevAdimi, GorevAsamasi, GorevHedefi, GorevKimligi, GorevTanimi,
    KontrolNoktasiKimligi, Kosul, KuralKimligi, KutuAlan, Macera, OlayFiltresi,
    OlayKurali, OyunOlayi, OyunSonucu, SahneKimligi, SahneTanimi, Tekrarlama, Vektor3,
};

#[derive(Clone)]
pub(crate) struct MaceraKimlikleri {
    pub(crate) pusula: EsyaKimligi,
    pub(crate) muhur: EsyaKimligi,
    pub(crate) gorev: GorevKimligi,
    pub(crate) gozcu_diyalogu: DiyalogKimligi,
    pub(crate) koy: SahneKimligi,
    pub(crate) mahzen: SahneKimligi,
    pub(crate) tapinak_alani: AlanKimligi,
    pub(crate) gozcu: EtkilesimKimligi,
    pub(crate) muhafiz: EtkilesimKimligi,
}

impl MaceraKimlikleri {
    pub(crate) fn yeni() -> Self {
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

pub(crate) fn macerayi_olustur(kimlikler: &MaceraKimlikleri) -> OyunSonucu<Macera> {
    let mut macera = Macera::yeni();
    esyalari_ve_gorevi_tanimla(&mut macera, kimlikler);
    diyalogu_tanimla(&mut macera, kimlikler);
    sahneleri_tanimla(&mut macera, kimlikler)?;
    etkilesimleri_tanimla(&mut macera, kimlikler);
    tapinagi_tanimla(&mut macera, kimlikler);
    tamamlanma_kuralini_tanimla(&mut macera, kimlikler);
    Ok(macera)
}

fn esyalari_ve_gorevi_tanimla(macera: &mut Macera, kimlikler: &MaceraKimlikleri) {
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
}

fn diyalogu_tanimla(macera: &mut Macera, kimlikler: &MaceraKimlikleri) {
    let kabul = DiyalogSecenegi::yeni("kabul", "Görevi kabul ediyorum.")
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
        .sonraki("yol");
    let reddet = DiyalogSecenegi::yeni("reddet", "Şimdilik hazır değilim.");
    macera.diyalog_tanimla(DiyalogTanimi::yeni(
        kimlikler.gozcu_diyalogu.clone(),
        "selam",
        vec![
            DiyalogDugumu::yeni(
                "selam",
                "Gözcü Aras",
                "Sis tapınağı yeniden uyandı. Mührün üç parçasını bulabilir misin?",
            )
            .secenekler(vec![kabul, reddet]),
            DiyalogDugumu::yeni(
                "yol",
                "Gözcü Aras",
                "Pusula parçalara yaklaştığında titreşecek. Üçünü de alınca kuzeydeki tapınağa git.",
            ),
        ],
    ));
}

fn sahneleri_tanimla(
    macera: &mut Macera,
    kimlikler: &MaceraKimlikleri,
) -> OyunSonucu {
    macera.sahne_tanimla(
        SahneTanimi::yeni(kimlikler.koy.clone(), "Sisli Köy Meydanı")
            .giris_noktasi("baslangic", Vektor3::yeni(0.0, 2.5, 3.0)),
    );
    macera.sahne_tanimla(
        SahneTanimi::yeni(kimlikler.mahzen.clone(), "Kadim Mahzen")
            .giris_noktasi("sunak", Vektor3::yeni(0.0, 2.5, -1.2)),
    );
    macera.baslangic_sahnesi_ayarla(&kimlikler.koy)
}

fn etkilesimleri_tanimla(macera: &mut Macera, kimlikler: &MaceraKimlikleri) {
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
    muhur_etkilesimlerini_tanimla(macera, kimlikler);
}

fn muhur_etkilesimlerini_tanimla(macera: &mut Macera, kimlikler: &MaceraKimlikleri) {
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
}

fn tapinagi_tanimla(macera: &mut Macera, kimlikler: &MaceraKimlikleri) {
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
    macera.etkilesim_tanimla(muhafiz_etkilesimi(kimlikler));
}

fn muhafiz_etkilesimi(kimlikler: &MaceraKimlikleri) -> EtkilesimNoktasi {
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
    ])
}

fn tamamlanma_kuralini_tanimla(macera: &mut Macera, kimlikler: &MaceraKimlikleri) {
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
}
