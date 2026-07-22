use tgame::onsoz::{
    ArayuzPaneli, Dunya, EkranDikdortgeni, EkranMetni, EkranRengi, GorevAsamasi,
    GorevIlerlemesi, Macera, Renk, SahneKimligi, Vektor3,
};

use crate::macera_icerigi::MaceraKimlikleri;

const KARANLIK_PANEL: Renk = Renk::yeni(0.025, 0.035, 0.055, 0.88);
const IKINCIL_PANEL: Renk = Renk::yeni(0.045, 0.065, 0.095, 0.92);

pub(crate) fn arayuzu_guncelle(
    dunya: &mut Dunya,
    macera: &Macera,
    kimlikler: &MaceraKimlikleri,
    oyuncu_konumu: Vektor3,
    gunluk_acik: bool,
) {
    let gorev_ozeti = gorev_ozeti(macera, kimlikler);
    let yakin_etkilesim = macera
        .yakin_etkilesim(oyuncu_konumu)
        .map(|etkilesim| etkilesim.ileti);
    let diyalog = macera.diyalog_gorunumu().ok().flatten();
    let arayuz = dunya.arayuz_mut();
    arayuz.temizle();

    let gorev_alani = EkranDikdortgeni::yeni(18.0, 18.0, 390.0, 112.0);
    arayuz.panel_ekle(ArayuzPaneli::yeni(gorev_alani, KARANLIK_PANEL));
    arayuz.metin_ekle(
        EkranMetni::yeni("KAYIP MÜHÜR", EkranDikdortgeni::yeni(34.0, 30.0, 360.0, 28.0), 21.0)
            .renk(EkranRengi::SARI),
    );
    arayuz.metin_ekle(EkranMetni::yeni(
        gorev_ozeti,
        EkranDikdortgeni::yeni(34.0, 62.0, 350.0, 58.0),
        16.0,
    ));

    if gunluk_acik {
        gunluk_paneli_ekle(arayuz, macera, kimlikler);
    }

    if let Some(gorunum) = diyalog {
        diyalog_paneli_ekle(arayuz, &gorunum);
    } else if let Some(ileti) = yakin_etkilesim {
        let alan = EkranDikdortgeni::yeni(250.0, 566.0, 460.0, 52.0);
        arayuz.panel_ekle(ArayuzPaneli::yeni(alan, IKINCIL_PANEL));
        arayuz.metin_ekle(
            EkranMetni::yeni(
                format!("[E]  {ileti}"),
                EkranDikdortgeni::yeni(270.0, 580.0, 420.0, 28.0),
                18.0,
            )
            .renk(EkranRengi::MAVI),
        );
    }

    arayuz.metin_ekle(
        EkranMetni::yeni(
            "WASD Hareket  •  E Etkileşim  •  Tab Günlük  •  F5/F9 Kayıt",
            EkranDikdortgeni::yeni(18.0, 610.0, 700.0, 24.0),
            13.0,
        )
        .renk(EkranRengi::yeni(205, 215, 230, 220)),
    );
}

fn gorev_ozeti(macera: &Macera, kimlikler: &MaceraKimlikleri) -> String {
    let ilerleme = macera.gorev_ilerlemesi(&kimlikler.gorev);
    match macera.gorev_asamasi(&kimlikler.gorev) {
        GorevAsamasi::Kilitli => "Gözcü Aras'ı bul ve onunla konuş.".to_owned(),
        GorevAsamasi::Etkin => etkin_gorev_ozeti(macera, kimlikler, ilerleme),
        GorevAsamasi::Tamamlandi => {
            "Görev tamamlandı\nUnvan: Mührün Varisi".to_owned()
        }
        GorevAsamasi::Basarisiz => "Görev başarısız oldu.".to_owned(),
    }
}

fn etkin_gorev_ozeti(
    macera: &Macera,
    kimlikler: &MaceraKimlikleri,
    ilerleme: Option<&GorevIlerlemesi>,
) -> String {
    match ilerleme.map_or(0, GorevIlerlemesi::adim) {
        0 => format!(
            "Üç mühür parçasını bul\nToplanan: {}/3",
            macera.envanter().miktar(&kimlikler.muhur)
        ),
        1 => "Kuzeydeki tapınak eşiğine ulaş.".to_owned(),
        _ => "Taş Muhafız'ın kadim mührünü çöz.".to_owned(),
    }
}

fn gunluk_paneli_ekle(
    arayuz: &mut tgame::onsoz::Arayuz,
    macera: &Macera,
    kimlikler: &MaceraKimlikleri,
) {
    let alan = EkranDikdortgeni::yeni(610.0, 18.0, 332.0, 245.0);
    arayuz.panel_ekle(ArayuzPaneli::yeni(alan, KARANLIK_PANEL));
    arayuz.metin_ekle(
        EkranMetni::yeni(
            "MACERA GÜNLÜĞÜ",
            EkranDikdortgeni::yeni(630.0, 32.0, 290.0, 30.0),
            20.0,
        )
        .renk(EkranRengi::SARI),
    );
    let oynama = macera.oynama_suresi_milisaniye();
    let sahne = macera.etkin_sahne().map_or("Bilinmiyor", SahneKimligi::deger);
    let metin = format!(
        "Sahne: {sahne}\n\nSis Pusulası: {}\nMühür Parçası: {}/3\n\nUnvan: {}\nOynama: {}.{:03} sn\n\nF5: Kaydet\nF9: Yükle\nR: Kontrol noktasına dön",
        macera.envanter().miktar(&kimlikler.pusula),
        macera.envanter().miktar(&kimlikler.muhur),
        macera.durum().metin("oyuncu_unvani").unwrap_or("Yolcu"),
        oynama / 1_000,
        oynama % 1_000,
    );
    arayuz.metin_ekle(EkranMetni::yeni(
        metin,
        EkranDikdortgeni::yeni(630.0, 68.0, 285.0, 180.0),
        15.0,
    ));
}

fn diyalog_paneli_ekle(
    arayuz: &mut tgame::onsoz::Arayuz,
    gorunum: &tgame::onsoz::DiyalogGorunumu,
) {
    let alan = EkranDikdortgeni::yeni(60.0, 402.0, 840.0, 198.0);
    arayuz.panel_ekle(ArayuzPaneli::yeni(alan, KARANLIK_PANEL));
    arayuz.metin_ekle(
        EkranMetni::yeni(
            gorunum.konusan.clone(),
            EkranDikdortgeni::yeni(84.0, 420.0, 780.0, 28.0),
            21.0,
        )
        .renk(EkranRengi::SARI),
    );
    arayuz.metin_ekle(EkranMetni::yeni(
        gorunum.metin.clone(),
        EkranDikdortgeni::yeni(84.0, 454.0, 780.0, 60.0),
        17.0,
    ));

    let secenekler = if gorunum.secenekler.is_empty() {
        if gorunum.ilerletilebilir {
            "[Enter] Devam et".to_owned()
        } else {
            String::new()
        }
    } else {
        gorunum
            .secenekler
            .iter()
            .enumerate()
            .map(|(sira, secenek)| format!("[{}] {}", sira + 1, secenek.metin))
            .collect::<Vec<_>>()
            .join("\n")
    };
    arayuz.metin_ekle(
        EkranMetni::yeni(
            secenekler,
            EkranDikdortgeni::yeni(84.0, 520.0, 780.0, 66.0),
            16.0,
        )
        .renk(EkranRengi::MAVI),
    );
}
