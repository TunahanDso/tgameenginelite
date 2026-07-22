//! Tgame Engine Lite sabit adımlı üç boyutlu fizik katmanı.

mod aabb;
mod govde;

use std::time::Duration;

pub use aabb::Aabb3;
pub use govde::{FizikGovdesi, GovdeTuru};
use tgame_matematik::Vektor3;
use tgame_varlik::{Dunya, VarlikKimligi};

const VARSAYILAN_ADIM: Duration = Duration::from_nanos(16_666_667);
const VARSAYILAN_AZAMI_ALT_ADIM: u32 = 8;
const AZAMI_KARE_SURESI: Duration = Duration::from_millis(250);

/// Bir render karesinde fizik simülasyonunun ne yaptığını bildirir.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FizikRaporu {
    /// Çalıştırılan sabit fizik adımı sayısı.
    pub adim_sayisi: u32,
    /// Uzun takılma nedeniyle kalan birikimin bırakılıp bırakılmadığı.
    pub birikim_birakildi: bool,
}

/// Sabit zaman adımı, yerçekimi ve 3B AABB çarpışmalarını yönetir.
#[derive(Debug, Clone)]
pub struct FizikDunyasi {
    govdeler: Vec<FizikGovdesi>,
    yercekimi: Vektor3,
    sabit_adim: Duration,
    birikim: Duration,
    azami_alt_adim: u32,
}

impl FizikDunyasi {
    /// Saniyede yaklaşık 60 fizik adımı ve Dünya yerçekimiyle simülatör oluşturur.
    #[must_use]
    pub fn yeni() -> Self {
        Self {
            govdeler: Vec::new(),
            yercekimi: Vektor3::yeni(0.0, -9.81, 0.0),
            sabit_adim: VARSAYILAN_ADIM,
            birikim: Duration::ZERO,
            azami_alt_adim: VARSAYILAN_AZAMI_ALT_ADIM,
        }
    }

    /// Sabit fizik adımını değiştirir; sıfır süre kabul edilmez.
    #[must_use]
    pub fn sabit_adim(mut self, adim: Duration) -> Self {
        if !adim.is_zero() {
            self.sabit_adim = adim;
        }
        self
    }

    /// Dünya yerçekimi ivmesini değiştirir.
    #[must_use]
    pub const fn yercekimi(mut self, yercekimi: Vektor3) -> Self {
        self.yercekimi = yercekimi;
        self
    }

    /// Tek render karesinde çalıştırılabilecek azami fizik alt adımını değiştirir.
    #[must_use]
    pub fn azami_alt_adim(mut self, adim: u32) -> Self {
        self.azami_alt_adim = adim.max(1);
        self
    }

    /// Simülasyona fizik gövdesi ekler.
    pub fn govde_ekle(&mut self, govde: FizikGovdesi) {
        if let Some(mevcut) = self
            .govdeler
            .iter_mut()
            .find(|mevcut| mevcut.varlik() == govde.varlik())
        {
            *mevcut = govde;
        } else {
            self.govdeler.push(govde);
        }
    }

    /// Kimliği verilen varlığın fizik gövdesini döndürür.
    #[must_use]
    pub fn govde(&self, varlik: VarlikKimligi) -> Option<&FizikGovdesi> {
        self.govdeler.iter().find(|govde| govde.varlik() == varlik)
    }

    /// Kimliği verilen varlığın fizik gövdesini değiştirilebilir döndürür.
    #[must_use]
    pub fn govde_mut(&mut self, varlik: VarlikKimligi) -> Option<&mut FizikGovdesi> {
        self.govdeler
            .iter_mut()
            .find(|govde| govde.varlik() == varlik)
    }

    /// Render karesinde geçen süreyi biriktirip gereken sabit fizik adımlarını çalıştırır.
    pub fn guncelle(&mut self, dunya: &mut Dunya, gecen: Duration) -> FizikRaporu {
        self.birikim = self.birikim.saturating_add(gecen.min(AZAMI_KARE_SURESI));
        let mut rapor = FizikRaporu::default();

        while self.birikim >= self.sabit_adim && rapor.adim_sayisi < self.azami_alt_adim {
            self.adim(dunya, self.sabit_adim.as_secs_f32());
            self.birikim -= self.sabit_adim;
            rapor.adim_sayisi += 1;
        }

        if self.birikim >= self.sabit_adim {
            self.birikim = Duration::ZERO;
            rapor.birikim_birakildi = true;
        }

        rapor
    }

    fn adim(&mut self, dunya: &mut Dunya, saniye: f32) {
        let statikler = self
            .govdeler
            .iter()
            .filter(|govde| govde.tur() == GovdeTuru::Statik)
            .filter_map(|govde| {
                dunya
                    .varlik(govde.varlik())
                    .map(|varlik| Aabb3::yeni(varlik.donusumu3b().konum, govde.yari_boyut()))
            })
            .collect::<Vec<_>>();
        let yercekimi = self.yercekimi;

        for govde in self
            .govdeler
            .iter_mut()
            .filter(|govde| govde.tur() == GovdeTuru::Dinamik)
        {
            let Some(varlik) = dunya.varlik(govde.varlik()) else {
                continue;
            };
            let mut konum = varlik.donusumu3b().konum;
            govde.zemini_ayarla(false);

            if govde.yercekimi_etkin() {
                govde.hizi_ayarla(govde.hiz() + yercekimi * saniye);
            }

            for eksen in 0..3 {
                eksende_hareket_et(govde, &mut konum, eksen, saniye, &statikler);
            }

            if let Some(varlik) = dunya.varlik_mut(govde.varlik()) {
                varlik.donusumu3b_mut().konum = konum;
            }
        }
    }
}

impl Default for FizikDunyasi {
    fn default() -> Self {
        Self::yeni()
    }
}

fn eksende_hareket_et(
    govde: &mut FizikGovdesi,
    konum: &mut Vektor3,
    eksen: usize,
    saniye: f32,
    statikler: &[Aabb3],
) {
    let hareket = eksen_degeri(govde.hiz(), eksen) * saniye;
    if hareket.abs() <= f32::EPSILON {
        return;
    }

    eksen_ayarla(konum, eksen, eksen_degeri(*konum, eksen) + hareket);

    for statik in statikler {
        let hareketli = Aabb3::yeni(*konum, govde.yari_boyut());
        if !hareketli.kesisiyor_mu(*statik) {
            continue;
        }

        let statik_merkez = eksen_degeri(statik.merkez(), eksen);
        let toplam_yari_boyut =
            eksen_degeri(statik.yari_boyut(), eksen) + eksen_degeri(govde.yari_boyut(), eksen);
        let cozulmus = if hareket > 0.0 {
            statik_merkez - toplam_yari_boyut
        } else {
            statik_merkez + toplam_yari_boyut
        };
        eksen_ayarla(konum, eksen, cozulmus);

        if eksen == 1 && hareket < 0.0 {
            govde.zemini_ayarla(true);
        }
        govde.eksen_hizini_sifirla(eksen);
    }
}

const fn eksen_degeri(vektor: Vektor3, eksen: usize) -> f32 {
    match eksen {
        0 => vektor.x,
        1 => vektor.y,
        _ => vektor.z,
    }
}

const fn eksen_ayarla(vektor: &mut Vektor3, eksen: usize, deger: f32) {
    match eksen {
        0 => vektor.x = deger,
        1 => vektor.y = deger,
        _ => vektor.z = deger,
    }
}

#[cfg(test)]
mod testler {
    use std::time::Duration;

    use tgame_matematik::{Renk, Vektor3};
    use tgame_varlik::{Donusum3B, Dunya, Varlik};

    use crate::{FizikDunyasi, FizikGovdesi, VARSAYILAN_ADIM};

    fn yakin(sol: f32, sag: f32) -> bool {
        (sol - sag).abs() < 0.01
    }

    #[test]
    fn dinamik_govde_zemine_oturur_ve_ziplayabilir() {
        let mut dunya = Dunya::yeni_3b();
        let zemin = dunya.varlik_ekle(
            Varlik::kup("Zemin", Renk::YESIL).donusum3b(
                Donusum3B::yeni()
                    .konum(Vektor3::yeni(0.0, -0.5, 0.0))
                    .olcek(Vektor3::yeni(10.0, 1.0, 10.0)),
            ),
        );
        let oyuncu = dunya.varlik_ekle(
            Varlik::kup("Oyuncu", Renk::SARI)
                .donusum3b(Donusum3B::yeni().konum(Vektor3::yeni(0.0, 3.0, 0.0))),
        );
        let mut fizik = FizikDunyasi::yeni();
        fizik.govde_ekle(FizikGovdesi::statik_kup(
            zemin,
            Vektor3::yeni(10.0, 1.0, 10.0),
        ));
        fizik.govde_ekle(FizikGovdesi::dinamik_kup(oyuncu, Vektor3::BIR));

        for _ in 0..180 {
            fizik.guncelle(&mut dunya, VARSAYILAN_ADIM);
        }

        let konum = dunya
            .varlik(oyuncu)
            .expect("Oyuncu bulunmalı.")
            .donusumu3b()
            .konum;
        assert!(yakin(konum.y, 0.5));
        assert!(
            fizik
                .govde(oyuncu)
                .expect("Fizik gövdesi bulunmalı.")
                .zeminde_mi()
        );

        assert!(
            fizik
                .govde_mut(oyuncu)
                .expect("Fizik gövdesi bulunmalı.")
                .ziplat(5.0)
        );
        fizik.guncelle(&mut dunya, Duration::from_millis(17));

        let ziplama_konumu = dunya
            .varlik(oyuncu)
            .expect("Oyuncu bulunmalı.")
            .donusumu3b()
            .konum;
        assert!(ziplama_konumu.y > konum.y);
    }
}
