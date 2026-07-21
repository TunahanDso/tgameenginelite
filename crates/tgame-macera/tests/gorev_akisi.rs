use tgame_macera::{
    EsyaKimligi, EsyaTanimi, GorevAdimi, GorevAsamasi, GorevHedefi, GorevKimligi,
    GorevTanimi, Macera, OlayFiltresi,
};

#[test]
fn esya_olayi_cok_adimli_gorevi_ilerletir() {
    let anahtar = EsyaKimligi::from("anahtar");
    let gorev = GorevKimligi::from("eski_kapi");
    let mut macera = Macera::yeni();
    macera.esya_tanimla(EsyaTanimi::yeni(anahtar.clone(), "Paslı Anahtar"));
    macera.gorev_tanimla(GorevTanimi::yeni(
        gorev.clone(),
        "Eski Kapı",
        vec![GorevAdimi::yeni(
            "Anahtarı bul",
            vec![GorevHedefi::yeni(
                "Bir anahtar edin",
                OlayFiltresi::EsyaEklendi(anahtar.clone()),
                1,
            )],
        )],
    ));

    macera.gorev_baslat(&gorev).expect("Görev başlamalı.");
    macera.esya_ekle(&anahtar, 1).expect("Eşya eklenmeli.");

    assert_eq!(macera.gorev_asamasi(&gorev), GorevAsamasi::Tamamlandi);
    assert_eq!(macera.envanter().miktar(&anahtar), 1);
}
