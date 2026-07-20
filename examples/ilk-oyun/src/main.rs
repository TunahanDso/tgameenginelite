use tgame::onsoz::{Oyun, OyunSonucu, Sahne};

fn main() -> OyunSonucu {
    Oyun::yeni("İlk Tgame Oyunum")
        .cozunurluk(800, 600)
        .mod_klasoru("modlar")
        .sahne_ekle(Sahne::yeni("Başlangıç"))
        .calistir()
}
