use tgame_macera::{GorevKimligi, SahneKimligi};

#[test]
fn kimlikler_metin_degerlerini_korur() {
    assert_eq!(GorevKimligi::from("ana_gorev").deger(), "ana_gorev");
    assert_eq!(SahneKimligi::from("koy").deger(), "koy");
}
