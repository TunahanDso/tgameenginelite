use tgame::onsoz::{
    Donusum3B, Dunya, Kamera3B, Oyun, OyunAkisi, OyunSonucu, Renk, Sahne, Tus, Varlik,
    VarlikKimligi, Vektor3,
};

const OYUNCU_HIZI: f32 = 4.2;
const KAMERA_DONUS_HIZI: f32 = 1.5;
const KAMERA_YUKSEKLIK_HIZI: f32 = 3.0;
const KAMERA_UZAKLIGI: f32 = 8.0;

fn main() -> OyunSonucu {
    let (dunya, oyuncu, merkez) = sahneyi_olustur();
    let mut kamera_acisi = 0.0_f32;
    let mut kamera_yuksekligi = 4.2_f32;

    Oyun::yeni("İlk Tgame 3B Dünyam")
        .cozunurluk(960, 640)
        .mod_klasoru("modlar")
        .sahne_ekle(Sahne::yeni("3B Başlangıç"))
        .dunya(dunya)
        .her_kare(move |girdi, zaman, dunya| {
            let kare_saniyesi = zaman.kare_saniyesi().min(0.05);
            let mut yon = Vektor3::SIFIR;
            if girdi.basili_mi(Tus::W) {
                yon += Vektor3::ILERI;
            }
            if girdi.basili_mi(Tus::S) {
                yon += Vektor3::GERI;
            }
            if girdi.basili_mi(Tus::A) {
                yon += Vektor3::SOL;
            }
            if girdi.basili_mi(Tus::D) {
                yon += Vektor3::SAG;
            }

            if girdi.basili_mi(Tus::Sol) {
                kamera_acisi -= KAMERA_DONUS_HIZI * kare_saniyesi;
            }
            if girdi.basili_mi(Tus::Sag) {
                kamera_acisi += KAMERA_DONUS_HIZI * kare_saniyesi;
            }
            if girdi.basili_mi(Tus::Yukari) {
                kamera_yuksekligi += KAMERA_YUKSEKLIK_HIZI * kare_saniyesi;
            }
            if girdi.basili_mi(Tus::Asagi) {
                kamera_yuksekligi -= KAMERA_YUKSEKLIK_HIZI * kare_saniyesi;
            }
            kamera_yuksekligi = kamera_yuksekligi.clamp(2.0, 10.0);

            let oyuncu_konumu = {
                let varlik = dunya
                    .varlik_mut(oyuncu)
                    .expect("Oyuncu varlığı oyun boyunca dünyada kalmalı.");
                let donusum = varlik.donusumu3b_mut();
                if yon.uzunluk_karesi() > 0.0 {
                    donusum.tasi(yon.birim() * OYUNCU_HIZI * kare_saniyesi);
                }
                donusum.dondur(Vektor3::yeni(
                    0.35 * kare_saniyesi,
                    0.9 * kare_saniyesi,
                    0.2 * kare_saniyesi,
                ));
                donusum.konum
            };

            dunya
                .varlik_mut(merkez)
                .expect("Merkez küp oyun boyunca dünyada kalmalı.")
                .donusumu3b_mut()
                .dondur(Vektor3::yeni(
                    0.25 * kare_saniyesi,
                    0.7 * kare_saniyesi,
                    0.15 * kare_saniyesi,
                ));

            let yatay_uzaklik = KAMERA_UZAKLIGI;
            let kamera_konumu = oyuncu_konumu
                + Vektor3::yeni(
                    kamera_acisi.sin() * yatay_uzaklik,
                    kamera_yuksekligi,
                    kamera_acisi.cos() * yatay_uzaklik,
                );
            let kamera = dunya.kamera3b_mut();
            kamera.konum = kamera_konumu;
            kamera.hedef = oyuncu_konumu + Vektor3::YUKARI * 0.4;

            if girdi.bu_kare_basildi_mi(Tus::Bosluk) {
                println!(
                    "Oyuncu ({:.2}, {:.2}, {:.2}) — {}. kare — {:.3} saniye",
                    oyuncu_konumu.x,
                    oyuncu_konumu.y,
                    oyuncu_konumu.z,
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

fn sahneyi_olustur() -> (Dunya, VarlikKimligi, VarlikKimligi) {
    let mut dunya = Dunya::yeni_3b();
    dunya.kamera3b_ayarla(
        Kamera3B::yeni()
            .konum(Vektor3::yeni(0.0, 4.2, 8.0))
            .hedef(Vektor3::SIFIR)
            .kirpma(0.1, 250.0),
    );

    zemin_ekle(&mut dunya);
    sutunlari_ekle(&mut dunya);

    let oyuncu = dunya.varlik_ekle(
        Varlik::kup("Oyuncu", Renk::SARI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 0.15, 3.0))
                .olcek(Vektor3::yeni(0.75, 0.75, 0.75)),
        ),
    );
    let merkez = dunya.varlik_ekle(
        Varlik::kup("Dönen Merkez", Renk::KIRMIZI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 1.0, 0.0))
                .olcek(Vektor3::yeni(1.7, 1.7, 1.7))
                .donus(Vektor3::yeni(0.25, 0.35, 0.1)),
        ),
    );

    (dunya, oyuncu, merkez)
}

fn zemin_ekle(dunya: &mut Dunya) {
    let koordinatlar = [-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let mut acik_renk = false;

    for z in koordinatlar {
        for x in koordinatlar {
            let renk = if acik_renk {
                Renk::yeni(0.18, 0.24, 0.34, 1.0)
            } else {
                Renk::yeni(0.10, 0.14, 0.22, 1.0)
            };
            dunya.varlik_ekle(
                Varlik::kup("Zemin", renk).donusum3b(
                    Donusum3B::yeni()
                        .konum(Vektor3::yeni(x, -0.65, z))
                        .olcek(Vektor3::yeni(0.96, 0.12, 0.96)),
                ),
            );
            acik_renk = !acik_renk;
        }
        acik_renk = !acik_renk;
    }
}

fn sutunlari_ekle(dunya: &mut Dunya) {
    let sutunlar = [
        (Vektor3::yeni(-4.0, 0.4, -4.0), Renk::MAVI, 2.0),
        (Vektor3::yeni(4.0, 0.9, -4.0), Renk::YESIL, 3.0),
        (Vektor3::yeni(-4.0, 1.4, 4.0), Renk::KIRMIZI, 4.0),
        (Vektor3::yeni(4.0, 0.65, 4.0), Renk::SARI, 2.5),
        (Vektor3::yeni(-2.5, 0.15, 0.0), Renk::YESIL, 1.5),
        (Vektor3::yeni(2.5, 0.15, 0.0), Renk::MAVI, 1.5),
    ];

    for (konum, renk, yukseklik) in sutunlar {
        dunya.varlik_ekle(
            Varlik::kup("Sütun", renk).donusum3b(
                Donusum3B::yeni()
                    .konum(konum)
                    .olcek(Vektor3::yeni(0.8, yukseklik, 0.8)),
            ),
        );
    }
}
