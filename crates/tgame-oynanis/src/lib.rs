//! Tgame Engine Lite sağlık, savaş, basit yapay zekâ ve ses olay katmanı.

use std::{collections::BTreeMap, time::Duration};

use tgame_matematik::Vektor3;
use tgame_varlik::VarlikKimligi;

/// Bir varlığın ait olduğu savaş takımını belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Takim {
    /// Oyuncu ve dostları.
    Oyuncu,
    /// Oyuncuya saldıran düşmanlar.
    Dusman,
    /// Savaşa katılmayan varlıklar.
    Tarafsiz,
}

/// Azami ve güncel can değerlerini güvenli biçimde taşır.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Can {
    guncel: f32,
    azami: f32,
}

impl Can {
    /// Pozitif azami değerle dolu can oluşturur.
    #[must_use]
    pub fn yeni(azami: f32) -> Self {
        let azami = if azami.is_finite() && azami > 0.0 {
            azami
        } else {
            1.0
        };
        Self {
            guncel: azami,
            azami,
        }
    }

    /// Güncel canı döndürür.
    #[must_use]
    pub const fn guncel(self) -> f32 {
        self.guncel
    }

    /// Azami canı döndürür.
    #[must_use]
    pub const fn azami(self) -> f32 {
        self.azami
    }

    /// Can oranını 0–1 aralığında döndürür.
    #[must_use]
    pub fn oran(self) -> f32 {
        (self.guncel / self.azami).clamp(0.0, 1.0)
    }

    /// Varlığın canlı olup olmadığını döndürür.
    #[must_use]
    pub fn canli_mi(self) -> bool {
        self.guncel > 0.0
    }

    /// Pozitif hasarı uygular ve gerçekten azalan miktarı döndürür.
    pub fn hasar_al(&mut self, miktar: f32) -> f32 {
        if !miktar.is_finite() || miktar <= 0.0 || !self.canli_mi() {
            return 0.0;
        }
        let onceki = self.guncel;
        self.guncel = (self.guncel - miktar).max(0.0);
        onceki - self.guncel
    }

    /// Pozitif iyileştirmeyi uygular ve gerçekten artan miktarı döndürür.
    pub fn iyilestir(&mut self, miktar: f32) -> f32 {
        if !miktar.is_finite() || miktar <= 0.0 || !self.canli_mi() {
            return 0.0;
        }
        let onceki = self.guncel;
        self.guncel = (self.guncel + miktar).min(self.azami);
        self.guncel - onceki
    }

    /// Canı azami değere doldurur.
    pub fn tamamen_iyilestir(&mut self) {
        self.guncel = self.azami;
    }
}

/// Bir savaşçının saldırı ve can ayarlarını taşır.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Savasci {
    /// Savaş takımı.
    pub takim: Takim,
    /// Can durumu.
    pub can: Can,
    /// Başarılı saldırının hasarı.
    pub saldiri_hasari: f32,
    /// Saldırı menzili.
    pub saldiri_menzili: f32,
    /// İki saldırı arasındaki süre.
    pub saldiri_araligi: Duration,
    kalan_bekleme: Duration,
}

impl Savasci {
    /// Doğrulanmış savaşçı oluşturur.
    #[must_use]
    pub fn yeni(takim: Takim, azami_can: f32, hasar: f32, menzil: f32, aralik: Duration) -> Self {
        Self {
            takim,
            can: Can::yeni(azami_can),
            saldiri_hasari: guvenli_pozitif(hasar, 1.0),
            saldiri_menzili: guvenli_pozitif(menzil, 1.0),
            saldiri_araligi: if aralik.is_zero() {
                Duration::from_millis(250)
            } else {
                aralik
            },
            kalan_bekleme: Duration::ZERO,
        }
    }

    /// Saldırı bekleme süresinin bitip bitmediğini döndürür.
    #[must_use]
    pub fn saldirabilir_mi(self) -> bool {
        self.can.canli_mi() && self.kalan_bekleme.is_zero()
    }

    fn guncelle(&mut self, sure: Duration) {
        self.kalan_bekleme = self.kalan_bekleme.saturating_sub(sure);
    }

    fn saldiriyi_baslat(&mut self) {
        self.kalan_bekleme = self.saldiri_araligi;
    }
}

/// Savaş katmanında oluşan anlamlı olaydır.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SavasOlayi {
    /// Kaynak hedefe hasar verdi.
    Hasar {
        /// Saldıran varlık.
        kaynak: VarlikKimligi,
        /// Hasar alan varlık.
        hedef: VarlikKimligi,
        /// Uygulanan gerçek hasar.
        miktar: f32,
    },
    /// Varlığın canı sıfıra indi.
    Yenildi {
        /// Yenilen varlık.
        varlik: VarlikKimligi,
        /// Son hasarı veren varlık.
        kaynak: VarlikKimligi,
    },
}

/// Savaşçı kayıtlarını ve karelik savaş olaylarını yönetir.
#[derive(Debug, Default)]
pub struct SavasDunyasi {
    savascilar: BTreeMap<usize, Savasci>,
    olaylar: Vec<SavasOlayi>,
}

impl SavasDunyasi {
    /// Boş savaş dünyası oluşturur.
    #[must_use]
    pub const fn yeni() -> Self {
        Self {
            savascilar: BTreeMap::new(),
            olaylar: Vec::new(),
        }
    }

    /// Varlığa savaşçı bileşeni ekler veya mevcut bileşeni değiştirir.
    pub fn savasci_ekle(&mut self, varlik: VarlikKimligi, savasci: Savasci) {
        self.savascilar.insert(varlik.deger(), savasci);
    }

    /// Varlığın savaşçı bileşenini döndürür.
    #[must_use]
    pub fn savasci(&self, varlik: VarlikKimligi) -> Option<&Savasci> {
        self.savascilar.get(&varlik.deger())
    }

    /// Varlığın savaşçı bileşenini değiştirilebilir döndürür.
    #[must_use]
    pub fn savasci_mut(&mut self, varlik: VarlikKimligi) -> Option<&mut Savasci> {
        self.savascilar.get_mut(&varlik.deger())
    }

    /// Bütün saldırı beklemelerini ilerletir.
    pub fn guncelle(&mut self, sure: Duration) {
        for savasci in self.savascilar.values_mut() {
            savasci.guncelle(sure);
        }
    }

    /// Kaynaktan hedefe menzil ve takım kurallarıyla saldırı uygular.
    pub fn saldir(&mut self, kaynak: VarlikKimligi, hedef: VarlikKimligi, mesafe: f32) -> bool {
        if kaynak == hedef || !mesafe.is_finite() {
            return false;
        }
        let Some(kaynak_bilgisi) = self.savascilar.get(&kaynak.deger()).copied() else {
            return false;
        };
        let Some(hedef_bilgisi) = self.savascilar.get(&hedef.deger()).copied() else {
            return false;
        };
        if kaynak_bilgisi.takim == hedef_bilgisi.takim
            || kaynak_bilgisi.takim == Takim::Tarafsiz
            || hedef_bilgisi.takim == Takim::Tarafsiz
            || mesafe > kaynak_bilgisi.saldiri_menzili
            || !kaynak_bilgisi.saldirabilir_mi()
            || !hedef_bilgisi.can.canli_mi()
        {
            return false;
        }

        self.savascilar
            .get_mut(&kaynak.deger())
            .expect("Kopyalanan kaynak savaşçı kaydı korunmalı.")
            .saldiriyi_baslat();
        let hedef_savascisi = self
            .savascilar
            .get_mut(&hedef.deger())
            .expect("Kopyalanan hedef savaşçı kaydı korunmalı.");
        let hasar = hedef_savascisi.can.hasar_al(kaynak_bilgisi.saldiri_hasari);
        self.olaylar.push(SavasOlayi::Hasar {
            kaynak,
            hedef,
            miktar: hasar,
        });
        if !hedef_savascisi.can.canli_mi() {
            self.olaylar.push(SavasOlayi::Yenildi {
                varlik: hedef,
                kaynak,
            });
        }
        true
    }

    /// Biriken savaş olaylarını tüketerek döndürür.
    #[must_use]
    pub fn olaylari_al(&mut self) -> Vec<SavasOlayi> {
        std::mem::take(&mut self.olaylar)
    }
}

/// Basit düşman davranış ayarlarıdır.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BasitAjan {
    /// Hedefi algılama yarıçapı.
    pub algi_menzili: f32,
    /// Hedefi kaybetme yarıçapı.
    pub birakma_menzili: f32,
    /// Takip hareket hızı.
    pub hareket_hizi: f32,
    /// Saldırıya geçme mesafesi.
    pub saldiri_menzili: f32,
}

impl BasitAjan {
    /// Dengeli varsayılan değerlerle ajan oluşturur.
    #[must_use]
    pub const fn yeni() -> Self {
        Self {
            algi_menzili: 9.0,
            birakma_menzili: 13.0,
            hareket_hizi: 2.2,
            saldiri_menzili: 1.7,
        }
    }

    /// İki konumdan takip veya saldırı kararı üretir.
    #[must_use]
    pub fn karar(self, konum: Vektor3, hedef: Vektor3, hedef_aktif: bool) -> AjanKarari {
        if !hedef_aktif {
            return AjanKarari::Bekle;
        }
        let fark = hedef - konum;
        let mesafe = fark.uzunluk();
        if mesafe <= self.saldiri_menzili {
            AjanKarari::Saldir
        } else if mesafe <= self.algi_menzili {
            AjanKarari::TakipEt {
                yon: Vektor3::yeni(fark.x, 0.0, fark.z).birim(),
                hiz: self.hareket_hizi,
            }
        } else {
            AjanKarari::Bekle
        }
    }
}

impl Default for BasitAjan {
    fn default() -> Self {
        Self::yeni()
    }
}

/// Basit ajanın karelik kararıdır.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AjanKarari {
    /// Hareket veya saldırı yapmaz.
    Bekle,
    /// Hedef yönünde ilerler.
    TakipEt {
        /// Birim hareket yönü.
        yon: Vektor3,
        /// Saniyedeki hareket hızı.
        hiz: f32,
    },
    /// Hedefe saldırmayı dener.
    Saldir,
}

/// Ses motoruna aktarılacak soyut oynatma isteğidir.
#[derive(Debug, Clone, PartialEq)]
pub struct SesOlayi {
    /// İçerik sistemi tarafından kullanılan kalıcı ses adı.
    pub ses: String,
    /// İsteğe bağlı dünya konumu.
    pub konum: Option<Vektor3>,
    /// Doğrusal ses şiddeti.
    pub siddet: f32,
}

impl SesOlayi {
    /// İki boyutlu arayüz veya müzik sesi oluşturur.
    #[must_use]
    pub fn ekran(ses: impl Into<String>, siddet: f32) -> Self {
        Self {
            ses: ses.into(),
            konum: None,
            siddet: guvenli_siddet(siddet),
        }
    }

    /// Dünya konumlu ses oluşturur.
    #[must_use]
    pub fn dunya(ses: impl Into<String>, konum: Vektor3, siddet: f32) -> Self {
        Self {
            ses: ses.into(),
            konum: Some(konum),
            siddet: guvenli_siddet(siddet),
        }
    }
}

/// Kareler arasında biriken soyut ses olaylarını saklar.
#[derive(Debug, Default)]
pub struct SesKuyrugu {
    olaylar: Vec<SesOlayi>,
}

impl SesKuyrugu {
    /// Boş ses kuyruğu oluşturur.
    #[must_use]
    pub const fn yeni() -> Self {
        Self { olaylar: Vec::new() }
    }

    /// Kuyruğa ses isteği ekler.
    pub fn ekle(&mut self, olay: SesOlayi) {
        self.olaylar.push(olay);
    }

    /// Biriken ses isteklerini tüketir.
    #[must_use]
    pub fn olaylari_al(&mut self) -> Vec<SesOlayi> {
        std::mem::take(&mut self.olaylar)
    }
}

fn guvenli_pozitif(deger: f32, varsayilan: f32) -> f32 {
    if deger.is_finite() && deger > 0.0 {
        deger
    } else {
        varsayilan
    }
}

fn guvenli_siddet(deger: f32) -> f32 {
    if deger.is_finite() {
        deger.clamp(0.0, 1.0)
    } else {
        1.0
    }
}

#[cfg(test)]
mod testler {
    use std::time::Duration;

    use tgame_varlik::Dunya;

    use super::{BasitAjan, Can, SavasDunyasi, SavasOlayi, Savasci, Takim};

    #[test]
    fn can_sifirin_altina_inmez() {
        let mut can = Can::yeni(20.0);
        assert_eq!(can.hasar_al(50.0), 20.0);
        assert_eq!(can.guncel(), 0.0);
        assert!(!can.canli_mi());
    }

    #[test]
    fn savas_yenilgi_olayi_uretir() {
        let mut dunya = Dunya::yeni_3b();
        let oyuncu = dunya.varlik_ekle(tgame_varlik::Varlik::yeni("Oyuncu"));
        let dusman = dunya.varlik_ekle(tgame_varlik::Varlik::yeni("Düşman"));
        let mut savas = SavasDunyasi::yeni();
        savas.savasci_ekle(
            oyuncu,
            Savasci::yeni(Takim::Oyuncu, 100.0, 30.0, 2.0, Duration::from_millis(1)),
        );
        savas.savasci_ekle(
            dusman,
            Savasci::yeni(Takim::Dusman, 20.0, 5.0, 2.0, Duration::from_millis(1)),
        );

        assert!(savas.saldir(oyuncu, dusman, 1.0));
        assert!(savas
            .olaylari_al()
            .iter()
            .any(|olay| matches!(olay, SavasOlayi::Yenildi { varlik, .. } if *varlik == dusman)));
    }

    #[test]
    fn ajan_yakindaki_hedefi_takip_eder() {
        let karar = BasitAjan::yeni().karar(
            tgame_matematik::Vektor3::SIFIR,
            tgame_matematik::Vektor3::yeni(4.0, 0.0, 0.0),
            true,
        );
        assert!(matches!(karar, super::AjanKarari::TakipEt { .. }));
    }
}
