use tgame::onsoz::{
    Donusum2B, Dunya, Kamera2B, Oyun, OyunAkisi, OyunSonucu, Renk, Sahne, Tus, Varlik,
    VarlikKimligi, Vektor2,
};

const OYUNCU_HIZI: f32 = 2.8;
const OYUNCU_DONUS_HIZI: f32 = 2.0;

fn main() -> OyunSonucu {
    let (dunya, oyuncu) = sahneyi_olustur();

    Oyun::yeni("İlk Tgame Oyunum")
        .cozunurluk(800, 600)
        .mod_klasoru("modlar")
        .sahne_ekle(Sahne::yeni("Başlangıç"))
        .dunya(dunya)
        .her_kare(move |girdi, zaman, dunya| {
            let mut yon = Vektor2::SIFIR;
            if girdi.basili_mi(Tus::W) || girdi.basili_mi(Tus::Yukari) {
                yon += Vektor2::YUKARI;
            }
            if girdi.basili_mi(Tus::S) || girdi.basili_mi(Tus::Asagi) {
                yon += Vektor2::ASAGI;
            }
            if girdi.basili_mi(Tus::A) || girdi.basili_mi(Tus::Sol) {
                yon += Vektor2::SOL;
            }
            if girdi.basili_mi(Tus::D) || girdi.basili_mi(Tus::Sag) {
                yon += Vektor2::SAG;
            }

            let kare_saniyesi = zaman.kare_saniyesi().min(0.05);
            let oyuncu_konumu = {
                let varlik = dunya
                    .varlik_mut(oyuncu)
                    .expect("Oyuncu varlığı oyun boyunca dünyada kalmalı.");
                let donusum = varlik.donusumu_mut();
                if yon.uzunluk_karesi() > 0.0 {
                    donusum.tasi(yon.birim() * OYUNCU_HIZI * kare_saniyesi);
                    donusum.dondur(OYUNCU_DONUS_HIZI * kare_saniyesi);
                }
                donusum.konum
            };
            dunya.kamera_mut().konum = oyuncu_konumu;

            if girdi.bu_kare_basildi_mi(Tus::Bosluk) {
                println!(
                    "Oyuncu ({:.2}, {:.2}) — {}. kare — {:.3} saniye",
                    oyuncu_konumu.x,
                    oyuncu_konumu.y,
                    zaman.kare_sayisi(),
                    zaman.toplam_saniye(),
                );
            }

            if girdi.bu_kare_basildi_mi(Tus::Kacis) {
                OyunAkisi::Kapat
            } else {
                OyunAkisi::DevamEt
            }
        })
        .calistir()
}

fn sahneyi_olustur() -> (Dunya, VarlikKimligi) {
    let mut dunya = Dunya::yeni();
    dunya.kamerayi_ayarla(Kamera2B::yeni().gorus_yuksekligi(6.0));

    let oyuncu = dunya.varlik_ekle(
        Varlik::ucgen("Oyuncu", Renk::SARI)
            .donusum(Donusum2B::yeni().olcek(Vektor2::yeni(0.55, 0.55))),
    );

    let dekorlar = [
        (Vektor2::yeni(-2.5, 1.5), Renk::KIRMIZI, 0.7),
        (Vektor2::yeni(2.2, 1.8), Renk::YESIL, 0.9),
        (Vektor2::yeni(-1.8, -1.7), Renk::MAVI, 0.6),
        (Vektor2::yeni(2.7, -1.4), Renk::KIRMIZI, 0.8),
        (Vektor2::yeni(0.0, 2.6), Renk::MAVI, 0.5),
        (Vektor2::yeni(0.4, -2.5), Renk::YESIL, 0.65),
        (Vektor2::yeni(-3.6, -0.2), Renk::SARI, 0.45),
        (Vektor2::yeni(3.8, 0.3), Renk::MAVI, 0.55),
    ];

    for (sira, (konum, renk, olcek)) in dekorlar.into_iter().enumerate() {
        dunya.varlik_ekle(
            Varlik::ucgen(format!("Dekor {sira}"), renk).donusum(
                Donusum2B::yeni()
                    .konum(konum)
                    .olcek(Vektor2::yeni(olcek, olcek))
                    .donus(sira as f32 * 0.35),
            ),
        );
    }

    (dunya, oyuncu)
}
