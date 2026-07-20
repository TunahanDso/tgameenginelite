//! Tgame Engine Lite kare zamanı katmanı.

use std::time::{Duration, Instant};

/// Oyun geliştiricisine sunulan güncel kare zamanı bilgisidir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zaman {
    kare_suresi: Duration,
    toplam_sure: Duration,
    kare_sayisi: u64,
}

impl Zaman {
    /// Henüz hiçbir kare işlenmemiş başlangıç zamanını döndürür.
    #[must_use]
    pub const fn sifir() -> Self {
        Self {
            kare_suresi: Duration::ZERO,
            toplam_sure: Duration::ZERO,
            kare_sayisi: 0,
        }
    }

    /// Son iki kare arasındaki süreyi döndürür.
    #[must_use]
    pub const fn kare_suresi(&self) -> Duration {
        self.kare_suresi
    }

    /// Son iki kare arasındaki süreyi saniye cinsinden döndürür.
    #[must_use]
    pub fn kare_saniyesi(&self) -> f32 {
        self.kare_suresi.as_secs_f32()
    }

    /// Motorun çalışmaya başlamasından beri geçen toplam süreyi döndürür.
    #[must_use]
    pub const fn toplam_sure(&self) -> Duration {
        self.toplam_sure
    }

    /// Motorun çalışmaya başlamasından beri geçen süreyi saniye cinsinden döndürür.
    #[must_use]
    pub fn toplam_saniye(&self) -> f64 {
        self.toplam_sure.as_secs_f64()
    }

    /// İşlenmekte olan karenin sıra numarasını döndürür.
    #[must_use]
    pub const fn kare_sayisi(&self) -> u64 {
        self.kare_sayisi
    }
}

impl Default for Zaman {
    fn default() -> Self {
        Self::sifir()
    }
}

/// Motorun kare sürelerini tekdüze saat üzerinden ölçmesini sağlar.
#[doc(hidden)]
#[derive(Debug)]
pub struct ZamanYoneticisi {
    baslangic: Instant,
    onceki_kare: Instant,
    kare_sayisi: u64,
}

impl ZamanYoneticisi {
    /// Geçerli anı başlangıç kabul eden bir zaman yöneticisi oluşturur.
    #[must_use]
    pub fn yeni() -> Self {
        Self::baslangicla(Instant::now())
    }

    /// Yeni karenin süre bilgisini üretir.
    #[must_use]
    pub fn kareyi_baslat(&mut self) -> Zaman {
        self.kareyi_baslat_aninda(Instant::now())
    }

    fn baslangicla(baslangic: Instant) -> Self {
        Self {
            baslangic,
            onceki_kare: baslangic,
            kare_sayisi: 0,
        }
    }

    fn kareyi_baslat_aninda(&mut self, simdi: Instant) -> Zaman {
        let kare_suresi = simdi
            .checked_duration_since(self.onceki_kare)
            .unwrap_or(Duration::ZERO);
        let toplam_sure = simdi
            .checked_duration_since(self.baslangic)
            .unwrap_or(Duration::ZERO);

        self.onceki_kare = simdi;
        self.kare_sayisi = self.kare_sayisi.saturating_add(1);

        Zaman {
            kare_suresi,
            toplam_sure,
            kare_sayisi: self.kare_sayisi,
        }
    }
}

impl Default for ZamanYoneticisi {
    fn default() -> Self {
        Self::yeni()
    }
}

#[cfg(test)]
mod testler {
    use std::time::{Duration, Instant};

    use super::{Zaman, ZamanYoneticisi};

    #[test]
    fn sifir_zamani_bos_degerler_tasir() {
        let zaman = Zaman::sifir();

        assert_eq!(zaman.kare_suresi(), Duration::ZERO);
        assert_eq!(zaman.toplam_sure(), Duration::ZERO);
        assert_eq!(zaman.kare_sayisi(), 0);
    }

    #[test]
    fn kare_suresi_tekduze_saatten_hesaplanir() {
        let baslangic = Instant::now();
        let mut yonetici = ZamanYoneticisi::baslangicla(baslangic);

        let ilk = yonetici.kareyi_baslat_aninda(baslangic + Duration::from_millis(16));
        let ikinci =
            yonetici.kareyi_baslat_aninda(baslangic + Duration::from_millis(36));

        assert_eq!(ilk.kare_suresi(), Duration::from_millis(16));
        assert_eq!(ilk.toplam_sure(), Duration::from_millis(16));
        assert_eq!(ilk.kare_sayisi(), 1);
        assert_eq!(ikinci.kare_suresi(), Duration::from_millis(20));
        assert_eq!(ikinci.toplam_sure(), Duration::from_millis(36));
        assert_eq!(ikinci.kare_sayisi(), 2);
    }

    #[test]
    fn saat_geri_giderse_sure_sifira_sinirlanir() {
        let baslangic = Instant::now();
        let mut yonetici = ZamanYoneticisi::baslangicla(baslangic);

        let zaman = yonetici.kareyi_baslat_aninda(baslangic - Duration::from_millis(1));

        assert_eq!(zaman.kare_suresi(), Duration::ZERO);
        assert_eq!(zaman.toplam_sure(), Duration::ZERO);
    }
}
