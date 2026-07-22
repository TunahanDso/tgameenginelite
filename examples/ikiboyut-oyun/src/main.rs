use tgame::onsoz::{
    Donusum2B, Dunya, Oyun, OyunAkisi, OyunSonucu, Renk, Sahne, Tus, Varlik, Vektor2,
};

fn main() -> OyunSonucu {
    let mut dunya = Dunya::yeni();
    let oyuncu = dunya.varlik_ekle(
        Varlik::ucgen("2B Oyuncu", Renk::SARI)
            .donusum(Donusum2B::yeni().olcek(Vektor2::yeni(0.6, 0.6))),
    );
    dunya.varlik_ekle(
        Varlik::ucgen("Dekor", Renk::MAVI)
            .donusum(Donusum2B::yeni().konum(Vektor2::yeni(2.0, 1.0))),
    );

    Oyun::yeni("Tgame 2B Uyumluluk")
        .sahne_ekle(Sahne::yeni("2B"))
        .dunya(dunya)
        .her_kare(move |girdi, zaman, dunya| {
            let mut yon = Vektor2::SIFIR;
            if girdi.basili_mi(Tus::W) {
                yon += Vektor2::YUKARI;
            }
            if girdi.basili_mi(Tus::S) {
                yon += Vektor2::ASAGI;
            }
            if girdi.basili_mi(Tus::A) {
                yon += Vektor2::SOL;
            }
            if girdi.basili_mi(Tus::D) {
                yon += Vektor2::SAG;
            }

            let konum = {
                let donusum = dunya
                    .varlik_mut(oyuncu)
                    .expect("2B oyuncu dünyada kalmalı.")
                    .donusumu_mut();
                donusum.tasi(yon.birim() * 2.8 * zaman.kare_saniyesi().min(0.05));
                donusum.dondur(0.8 * zaman.kare_saniyesi().min(0.05));
                donusum.konum
            };
            dunya.kamera_mut().konum = konum;

            if girdi.bu_kare_basildi_mi(Tus::Kacis) {
                OyunAkisi::Kapat
            } else {
                OyunAkisi::DevamEt
            }
        })
        .calistir()
}
