use tgame_macera::{
    DiyalogDugumu, DiyalogKimligi, DiyalogSecenegi, DiyalogTanimi, Eylem, Macera,
};

#[test]
fn diyalog_secimi_hikaye_bayragini_degistirir() {
    let diyalog = DiyalogKimligi::from("muhtar");
    let mut macera = Macera::yeni();
    macera.diyalog_tanimla(DiyalogTanimi::yeni(
        diyalog.clone(),
        "baslangic",
        vec![DiyalogDugumu::yeni("baslangic", "Muhtar", "Yardım eder misin?")
            .secenekler(vec![DiyalogSecenegi::yeni("evet", "Evet").eylemler(vec![
                Eylem::BayrakAyarla {
                    ad: "yardim_ediyor".to_owned(),
                    deger: true,
                },
            ])])],
    ));

    macera
        .diyalog_baslat(&diyalog)
        .expect("Diyalog başlamalı.");
    macera.diyalog_sec("evet").expect("Seçim uygulanmalı.");

    assert!(macera.durum().bayrak("yardim_ediyor"));
}
