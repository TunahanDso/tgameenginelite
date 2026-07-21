use std::{fs, process, time::Duration};

use tgame_macera::{
    EsyaKimligi, EsyaTanimi, GorevAdimi, GorevAsamasi, GorevHedefi, GorevKimligi, GorevTanimi,
    KayitYoneticisi, Macera, OlayFiltresi, SahneKimligi, SahneTanimi,
};
use tgame_matematik::Vektor3;

fn icerigi_tanimla() -> (Macera, EsyaKimligi, GorevKimligi, SahneKimligi) {
    let pusula = EsyaKimligi::from("eski_pusula");
    let gorev = GorevKimligi::from("kayip_gecit");
    let sahne = SahneKimligi::from("dag_gecidi");
    let mut macera = Macera::yeni();

    macera.esya_tanimla(
        EsyaTanimi::yeni(pusula.clone(), "Eski Pusula")
            .aciklama("Sis içindeki geçidi gösteren görev eşyası.")
            .gorev_esyasi(),
    );
    macera.gorev_tanimla(GorevTanimi::yeni(
        gorev.clone(),
        "Kayıp Geçit",
        vec![GorevAdimi::yeni(
            "Pusulayı bul",
            vec![GorevHedefi::yeni(
                "Eski pusulayı edin",
                OlayFiltresi::EsyaEklendi(pusula.clone()),
                1,
            )],
        )],
    ));
    macera.sahne_tanimla(
        SahneTanimi::yeni(sahne.clone(), "Dağ Geçidi")
            .giris_noktasi("kamp", Vektor3::yeni(4.0, 1.0, -8.0)),
    );

    (macera, pusula, gorev, sahne)
}

#[test]
fn uzun_oyun_durumu_yuvadan_eksiksiz_geri_yuklenir() {
    let (mut macera, pusula, gorev, sahne) = icerigi_tanimla();
    macera
        .baslangic_sahnesi_ayarla(&sahne)
        .expect("Başlangıç sahnesi ayarlanmalı.");
    macera.gorev_baslat(&gorev).expect("Görev başlamalı.");
    macera.esya_ekle(&pusula, 1).expect("Görev eşyası eklenmeli.");
    macera.durum_mut().bayrak_ayarla("gecit_acildi", true);
    macera.durum_mut().sayac_ayarla("bulunan_sir", 7);
    macera
        .durum_mut()
        .metin_ayarla("son_konusulan", "Gözcü Aras");
    macera.sure_ekle(Duration::from_secs(7_321));

    let klasor = std::env::temp_dir().join(format!(
        "tgame-macera-kayit-{}-{}",
        process::id(),
        macera.oynama_suresi_milisaniye()
    ));
    let kayitlar = KayitYoneticisi::yeni(&klasor, 3);
    kayitlar.kaydet(1, &macera).expect("Macera diske kaydedilmeli.");
    assert!(kayitlar.var_mi(1).expect("Kayıt yuvası sorgulanmalı."));

    let (mut yuklenen, _, _, _) = icerigi_tanimla();
    kayitlar
        .yukle(1, &mut yuklenen)
        .expect("Macera kayıt yuvasından yüklenmeli.");

    assert_eq!(yuklenen.etkin_sahne(), Some(&sahne));
    assert_eq!(yuklenen.gorev_asamasi(&gorev), GorevAsamasi::Tamamlandi);
    assert_eq!(yuklenen.envanter().miktar(&pusula), 1);
    assert!(yuklenen.durum().bayrak("gecit_acildi"));
    assert_eq!(yuklenen.durum().sayac("bulunan_sir"), 7);
    assert_eq!(yuklenen.durum().metin("son_konusulan"), Some("Gözcü Aras"));
    assert_eq!(yuklenen.oynama_suresi_milisaniye(), 7_321_000);

    kayitlar.sil(1).expect("Kayıt yuvası silinmeli.");
    assert!(!kayitlar.var_mi(1).expect("Silinen yuva sorgulanmalı."));
    fs::remove_dir_all(&klasor).expect("Geçici kayıt klasörü temizlenmeli.");
}
