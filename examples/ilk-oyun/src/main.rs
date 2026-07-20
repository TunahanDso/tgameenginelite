use tgame::onsoz::{Oyun, OyunAkisi, OyunSonucu, Sahne, Tus};

fn main() -> OyunSonucu {
    Oyun::yeni("İlk Tgame Oyunum")
        .cozunurluk(800, 600)
        .mod_klasoru("modlar")
        .sahne_ekle(Sahne::yeni("Başlangıç"))
        .her_kare(|girdi, zaman| {
            if girdi.bu_kare_basildi_mi(Tus::Bosluk) {
                println!(
                    "Boşluk tuşuna {}. karede, {:.3} saniyede basıldı.",
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
