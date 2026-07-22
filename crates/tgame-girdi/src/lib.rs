//! Tgame Engine Lite Türkçe klavye ve fare girdi katmanı.

use std::collections::HashSet;

/// Oyunlarda kullanılabilen fiziksel klavye tuşlarını temsil eder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tus {
    /// A harfi tuşu.
    A,
    /// B harfi tuşu.
    B,
    /// C harfi tuşu.
    C,
    /// D harfi tuşu.
    D,
    /// E harfi tuşu.
    E,
    /// F harfi tuşu.
    F,
    /// G harfi tuşu.
    G,
    /// H harfi tuşu.
    H,
    /// I harfi tuşu.
    I,
    /// J harfi tuşu.
    J,
    /// K harfi tuşu.
    K,
    /// L harfi tuşu.
    L,
    /// M harfi tuşu.
    M,
    /// N harfi tuşu.
    N,
    /// O harfi tuşu.
    O,
    /// P harfi tuşu.
    P,
    /// Q harfi tuşu.
    Q,
    /// R harfi tuşu.
    R,
    /// S harfi tuşu.
    S,
    /// T harfi tuşu.
    T,
    /// U harfi tuşu.
    U,
    /// V harfi tuşu.
    V,
    /// W harfi tuşu.
    W,
    /// X harfi tuşu.
    X,
    /// Y harfi tuşu.
    Y,
    /// Z harfi tuşu.
    Z,
    /// Üst sıradaki 0 tuşu.
    Sayi0,
    /// Üst sıradaki 1 tuşu.
    Sayi1,
    /// Üst sıradaki 2 tuşu.
    Sayi2,
    /// Üst sıradaki 3 tuşu.
    Sayi3,
    /// Üst sıradaki 4 tuşu.
    Sayi4,
    /// Üst sıradaki 5 tuşu.
    Sayi5,
    /// Üst sıradaki 6 tuşu.
    Sayi6,
    /// Üst sıradaki 7 tuşu.
    Sayi7,
    /// Üst sıradaki 8 tuşu.
    Sayi8,
    /// Üst sıradaki 9 tuşu.
    Sayi9,
    /// Yukarı yön tuşu.
    Yukari,
    /// Aşağı yön tuşu.
    Asagi,
    /// Sol yön tuşu.
    Sol,
    /// Sağ yön tuşu.
    Sag,
    /// Boşluk tuşu.
    Bosluk,
    /// Enter tuşu.
    Enter,
    /// Escape tuşu.
    Kacis,
    /// Tab tuşu.
    Sekme,
    /// Geri silme tuşu.
    GeriSil,
    /// Sol Shift tuşu.
    SolShift,
    /// Sağ Shift tuşu.
    SagShift,
    /// Sol Control tuşu.
    SolKontrol,
    /// Sağ Control tuşu.
    SagKontrol,
    /// Sol Alt tuşu.
    SolAlt,
    /// Sağ Alt tuşu.
    SagAlt,
    /// F1 işlev tuşu.
    F1,
    /// F2 işlev tuşu.
    F2,
    /// F3 işlev tuşu.
    F3,
    /// F4 işlev tuşu.
    F4,
    /// F5 işlev tuşu.
    F5,
    /// F6 işlev tuşu.
    F6,
    /// F7 işlev tuşu.
    F7,
    /// F8 işlev tuşu.
    F8,
    /// F9 işlev tuşu.
    F9,
    /// F10 işlev tuşu.
    F10,
    /// F11 işlev tuşu.
    F11,
    /// F12 işlev tuşu.
    F12,
}

/// Bir kare boyunca biriken göreli fare hareketini taşır.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FareHareketi {
    /// Sağa doğru pozitif yatay hareket.
    pub x: f32,
    /// Aşağı doğru pozitif dikey hareket.
    pub y: f32,
}

impl FareHareketi {
    /// Yeni göreli fare hareketi oluşturur.
    #[must_use]
    pub const fn yeni(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Bir oyun karesindeki klavye ve fare durumunu saklar.
#[derive(Debug, Clone, Default)]
pub struct Girdi {
    basili_tuslar: HashSet<Tus>,
    bu_kare_basildi: HashSet<Tus>,
    bu_kare_birakildi: HashSet<Tus>,
    fare_hareketi: FareHareketi,
}

impl Girdi {
    /// Boş bir girdi durumu oluşturur.
    #[must_use]
    pub fn yeni() -> Self {
        Self::default()
    }

    /// Tuşun şu anda basılı olup olmadığını döndürür.
    #[must_use]
    pub fn basili_mi(&self, tus: Tus) -> bool {
        self.basili_tuslar.contains(&tus)
    }

    /// Tuşa bu kare içinde ilk kez basılıp basılmadığını döndürür.
    #[must_use]
    pub fn bu_kare_basildi_mi(&self, tus: Tus) -> bool {
        self.bu_kare_basildi.contains(&tus)
    }

    /// Tuşun bu kare içinde bırakılıp bırakılmadığını döndürür.
    #[must_use]
    pub fn bu_kare_birakildi_mi(&self, tus: Tus) -> bool {
        self.bu_kare_birakildi.contains(&tus)
    }

    /// Bu kare boyunca biriken göreli fare hareketini döndürür.
    #[must_use]
    pub const fn fare_hareketi(&self) -> FareHareketi {
        self.fare_hareketi
    }

    /// Motorun fiziksel tuş durumunu işlemesini sağlar.
    #[doc(hidden)]
    pub fn tus_durumunu_guncelle(&mut self, tus: Tus, basili: bool) {
        if basili {
            if self.basili_tuslar.insert(tus) {
                self.bu_kare_basildi.insert(tus);
            }
        } else if self.basili_tuslar.remove(&tus) {
            self.bu_kare_birakildi.insert(tus);
        }
    }

    /// Motorun göreli fare hareketini kare boyunca biriktirmesini sağlar.
    #[doc(hidden)]
    pub fn fare_hareketini_ekle(&mut self, x: f32, y: f32) {
        self.fare_hareketi.x += x;
        self.fare_hareketi.y += y;
    }

    /// Kareye özel geçici girdi durumlarını temizler.
    #[doc(hidden)]
    pub fn kareyi_bitir(&mut self) {
        self.bu_kare_basildi.clear();
        self.bu_kare_birakildi.clear();
        self.fare_hareketi = FareHareketi::default();
    }
}

#[cfg(test)]
mod testler {
    use super::{FareHareketi, Girdi, Tus};

    #[test]
    fn basma_tekrari_yalnizca_bir_kez_kaydedilir() {
        let mut girdi = Girdi::yeni();

        girdi.tus_durumunu_guncelle(Tus::W, true);
        girdi.tus_durumunu_guncelle(Tus::W, true);

        assert!(girdi.basili_mi(Tus::W));
        assert!(girdi.bu_kare_basildi_mi(Tus::W));
        assert!(!girdi.bu_kare_birakildi_mi(Tus::W));
    }

    #[test]
    fn birakilan_tus_durumdan_cikarilir() {
        let mut girdi = Girdi::yeni();

        girdi.tus_durumunu_guncelle(Tus::Bosluk, true);
        girdi.kareyi_bitir();
        girdi.tus_durumunu_guncelle(Tus::Bosluk, false);

        assert!(!girdi.basili_mi(Tus::Bosluk));
        assert!(!girdi.bu_kare_basildi_mi(Tus::Bosluk));
        assert!(girdi.bu_kare_birakildi_mi(Tus::Bosluk));
    }

    #[test]
    fn fare_hareketi_kare_boyunca_birikir_ve_temizlenir() {
        let mut girdi = Girdi::yeni();
        girdi.fare_hareketini_ekle(2.0, -1.0);
        girdi.fare_hareketini_ekle(0.5, 3.0);

        assert_eq!(girdi.fare_hareketi(), FareHareketi::yeni(2.5, 2.0));
        girdi.kareyi_bitir();
        assert_eq!(girdi.fare_hareketi(), FareHareketi::default());
    }

    #[test]
    fn kare_sonunda_gecici_durumlar_temizlenir() {
        let mut girdi = Girdi::yeni();

        girdi.tus_durumunu_guncelle(Tus::A, true);
        girdi.kareyi_bitir();

        assert!(girdi.basili_mi(Tus::A));
        assert!(!girdi.bu_kare_basildi_mi(Tus::A));
        assert!(!girdi.bu_kare_birakildi_mi(Tus::A));
    }
}
