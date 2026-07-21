use tgame::onsoz::{
    Donusum3B, Dunya, FizikDunyasi, FizikGovdesi, Kamera3B, Oyun, OyunAkisi, OyunSonucu, Renk,
    Sahne, Tus, Varlik, VarlikKimligi, Vektor3,
};

const OYUNCU_HIZI: f32 = 4.8;
const ZIPLAMA_HIZI: f32 = 6.2;
const FARE_HASSASIYETI: f32 = 0.0025;
const KLAVYE_KAMERA_HIZI: f32 = 1.5;
const KAMERA_UZAKLIGI: f32 = 7.5;
const OYUNCU_OLCEGI: Vektor3 = Vektor3::yeni(0.75, 0.75, 0.75);

fn main() -> OyunSonucu {
    let (dunya, mut fizik, oyuncu, merkez) = sahneyi_olustur();
    let mut kamera_yatay = 0.0_f32;
    let mut kamera_dikey = 0.35_f32;

    Oyun::yeni("Tgame 3B Fizik Dünyası")
        .cozunurluk(960, 640)
        .mod_klasoru("modlar")
        .sahne_ekle(Sahne::yeni("3B Fizik Başlangıcı"))
        .dunya(dunya)
        .her_kare(move |girdi, zaman, dunya| {
            let fare = girdi.fare_hareketi();
            kamera_yatay -= fare.x * FARE_HASSASIYETI;
            kamera_dikey = (kamera_dikey - fare.y * FARE_HASSASIYETI).clamp(-0.65, 1.05);

            let kare_saniyesi = zaman.kare_saniyesi().min(0.05);
            if girdi.basili_mi(Tus::Sol) {
                kamera_yatay -= KLAVYE_KAMERA_HIZI * kare_saniyesi;
            }
            if girdi.basili_mi(Tus::Sag) {
                kamera_yatay += KLAVYE_KAMERA_HIZI * kare_saniyesi;
            }
            if girdi.basili_mi(Tus::Yukari) {
                kamera_dikey += KLAVYE_KAMERA_HIZI * kare_saniyesi;
            }
            if girdi.basili_mi(Tus::Asagi) {
                kamera_dikey -= KLAVYE_KAMERA_HIZI * kare_saniyesi;
            }
            kamera_dikey = kamera_dikey.clamp(-0.65, 1.05);

            let ileri = Vektor3::yeni(-kamera_yatay.sin(), 0.0, -kamera_yatay.cos());
            let sag = Vektor3::yeni(kamera_yatay.cos(), 0.0, -kamera_yatay.sin());
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

            let oyuncu_govdesi = fizik
                .govde_mut(oyuncu)
                .expect("Oyuncu fizik gövdesi oyun boyunca kalmalı.");
            oyuncu_govdesi.yatay_hizi_ayarla(yon.birim() * OYUNCU_HIZI);
            if girdi.bu_kare_basildi_mi(Tus::Bosluk) {
                oyuncu_govdesi.ziplat(ZIPLAMA_HIZI);
            }

            fizik.guncelle(dunya, zaman.kare_suresi());

            let oyuncu_konumu = dunya
                .varlik(oyuncu)
                .expect("Oyuncu varlığı oyun boyunca kalmalı.")
                .donusumu3b()
                .konum;
            dunya
                .varlik_mut(oyuncu)
                .expect("Oyuncu varlığı oyun boyunca kalmalı.")
                .donusumu3b_mut()
                .donus_radyan.y = kamera_yatay;
            dunya
                .varlik_mut(merkez)
                .expect("Merkez küp oyun boyunca kalmalı.")
                .donusumu3b_mut()
                .dondur(Vektor3::yeni(
                    0.25 * kare_saniyesi,
                    0.7 * kare_saniyesi,
                    0.15 * kare_saniyesi,
                ));

            let yatay_uzaklik = kamera_dikey.cos() * KAMERA_UZAKLIGI;
            let kamera_konumu = oyuncu_konumu
                + Vektor3::yeni(
                    kamera_yatay.sin() * yatay_uzaklik,
                    1.2 + kamera_dikey.sin() * KAMERA_UZAKLIGI,
                    kamera_yatay.cos() * yatay_uzaklik,
                );
            let kamera = dunya.kamera3b_mut();
            kamera.konum = kamera_konumu;
            kamera.hedef = oyuncu_konumu + Vektor3::YUKARI * 0.3;

            if girdi.bu_kare_basildi_mi(Tus::Enter) {
                let govde = fizik
                    .govde(oyuncu)
                    .expect("Oyuncu fizik gövdesi bulunmalı.");
                println!(
                    "Oyuncu ({:.2}, {:.2}, {:.2}) — hız ({:.2}, {:.2}, {:.2}) — zeminde: {} — {}. kare",
                    oyuncu_konumu.x,
                    oyuncu_konumu.y,
                    oyuncu_konumu.z,
                    govde.hiz().x,
                    govde.hiz().y,
                    govde.hiz().z,
                    govde.zeminde_mi(),
                    zaman.kare_sayisi(),
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

fn sahneyi_olustur() -> (Dunya, FizikDunyasi, VarlikKimligi, VarlikKimligi) {
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
        Varlik::kup("Dönen Merkez", Renk::KIRMIZI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 1.35, 0.0))
                .olcek(merkez_olcegi)
                .donus(Vektor3::yeni(0.25, 0.35, 0.1)),
        ),
    );
    fizik.govde_ekle(FizikGovdesi::statik_kup(merkez, merkez_olcegi));

    (dunya, fizik, oyuncu, merkez)
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
