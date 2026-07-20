//! Tgame Engine Lite varlık, dönüşüm ve kamera katmanı.

use tgame_matematik::{Renk, Vektor2};

/// Oyun dünyasındaki bir varlığı benzersiz biçimde tanımlar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarlikKimligi(usize);

impl VarlikKimligi {
    /// Kimliğin dünya içindeki sayısal değerini döndürür.
    #[must_use]
    pub const fn deger(self) -> usize {
        self.0
    }
}

/// İki boyutlu konum, dönüş ve ölçek bilgisini taşır.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Donusum2B {
    /// Dünya konumu.
    pub konum: Vektor2,
    /// Radyan cinsinden saat yönünün tersine dönüş.
    pub donus_radyan: f32,
    /// Yerel ölçek.
    pub olcek: Vektor2,
}

impl Donusum2B {
    /// Birim dönüşüm oluşturur.
    #[must_use]
    pub const fn yeni() -> Self {
        Self {
            konum: Vektor2::SIFIR,
            donus_radyan: 0.0,
            olcek: Vektor2::BIR,
        }
    }

    /// Konumu değiştirir.
    #[must_use]
    pub const fn konum(mut self, konum: Vektor2) -> Self {
        self.konum = konum;
        self
    }

    /// Ölçeği değiştirir.
    #[must_use]
    pub const fn olcek(mut self, olcek: Vektor2) -> Self {
        self.olcek = olcek;
        self
    }

    /// Dönüşü radyan cinsinden değiştirir.
    #[must_use]
    pub const fn donus(mut self, radyan: f32) -> Self {
        self.donus_radyan = radyan;
        self
    }

    /// Dönüşümü dünya yönünde taşır.
    pub fn tasi(&mut self, hareket: Vektor2) {
        self.konum += hareket;
    }

    /// Dönüşü radyan cinsinden artırır.
    pub fn dondur(&mut self, radyan: f32) {
        self.donus_radyan += radyan;
    }
}

impl Default for Donusum2B {
    fn default() -> Self {
        Self::yeni()
    }
}

/// Bir varlığın çizilebilir görünümünü belirtir.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gorunum2B {
    /// Tek renkli üçgen görünümü.
    Ucgen {
        /// Üçgen rengi.
        renk: Renk,
    },
}

/// Oyun dünyasındaki kimlikli nesnedir.
#[derive(Debug, Clone, PartialEq)]
pub struct Varlik {
    kimlik: Option<VarlikKimligi>,
    ad: String,
    donusum: Donusum2B,
    gorunum: Option<Gorunum2B>,
    etkin: bool,
}

impl Varlik {
    /// Çizilebilir görünümü olmayan yeni bir varlık taslağı oluşturur.
    #[must_use]
    pub fn yeni(ad: impl Into<String>) -> Self {
        Self {
            kimlik: None,
            ad: ad.into(),
            donusum: Donusum2B::yeni(),
            gorunum: None,
            etkin: true,
        }
    }

    /// Tek renkli üçgen varlık taslağı oluşturur.
    #[must_use]
    pub fn ucgen(ad: impl Into<String>, renk: Renk) -> Self {
        Self::yeni(ad).gorunum(Gorunum2B::Ucgen { renk })
    }

    /// Başlangıç dönüşümünü değiştirir.
    #[must_use]
    pub const fn donusum(mut self, donusum: Donusum2B) -> Self {
        self.donusum = donusum;
        self
    }

    /// Çizilebilir görünümü değiştirir.
    #[must_use]
    pub const fn gorunum(mut self, gorunum: Gorunum2B) -> Self {
        self.gorunum = Some(gorunum);
        self
    }

    /// Varlık adını döndürür.
    #[must_use]
    pub fn ad(&self) -> &str {
        &self.ad
    }

    /// Dünya tarafından atanmış kimliği döndürür.
    #[must_use]
    pub const fn kimlik(&self) -> Option<VarlikKimligi> {
        self.kimlik
    }

    /// Dönüşümü döndürür.
    #[must_use]
    pub const fn donusumu(&self) -> &Donusum2B {
        &self.donusum
    }

    /// Dönüşümü değiştirilebilir olarak döndürür.
    #[must_use]
    pub const fn donusumu_mut(&mut self) -> &mut Donusum2B {
        &mut self.donusum
    }

    /// Görünümü döndürür.
    #[must_use]
    pub const fn gorunumu(&self) -> Option<Gorunum2B> {
        self.gorunum
    }

    /// Varlığın etkin olup olmadığını döndürür.
    #[must_use]
    pub const fn etkin_mi(&self) -> bool {
        self.etkin
    }

    /// Varlığın etkinlik durumunu değiştirir.
    pub const fn etkinlestir(&mut self, etkin: bool) {
        self.etkin = etkin;
    }
}

/// İki boyutlu dünya kamerasını tanımlar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kamera2B {
    /// Kamera merkezi.
    pub konum: Vektor2,
    /// Dikey dünya görüşünün yüksekliği.
    pub gorus_yuksekligi: f32,
}

impl Kamera2B {
    /// Varsayılan merkez ve görüş yüksekliğiyle kamera oluşturur.
    #[must_use]
    pub const fn yeni() -> Self {
        Self {
            konum: Vektor2::SIFIR,
            gorus_yuksekligi: 4.0,
        }
    }

    /// Kamera merkezini değiştirir.
    #[must_use]
    pub const fn konum(mut self, konum: Vektor2) -> Self {
        self.konum = konum;
        self
    }

    /// Dikey görüş yüksekliğini değiştirir.
    #[must_use]
    pub fn gorus_yuksekligi(mut self, yukseklik: f32) -> Self {
        if yukseklik.is_finite() && yukseklik > f32::EPSILON {
            self.gorus_yuksekligi = yukseklik;
        }
        self
    }
}

impl Default for Kamera2B {
    fn default() -> Self {
        Self::yeni()
    }
}

/// Varlıkları ve etkin kamerayı saklayan oyun dünyasıdır.
#[derive(Debug, Clone, Default)]
pub struct Dunya {
    varliklar: Vec<Varlik>,
    kamera: Kamera2B,
}

impl Dunya {
    /// Boş bir oyun dünyası oluşturur.
    #[must_use]
    pub fn yeni() -> Self {
        Self::default()
    }

    /// Dünyaya varlık ekler ve sabit kimliğini döndürür.
    pub fn varlik_ekle(&mut self, mut varlik: Varlik) -> VarlikKimligi {
        let kimlik = VarlikKimligi(self.varliklar.len());
        varlik.kimlik = Some(kimlik);
        self.varliklar.push(varlik);
        kimlik
    }

    /// Kimliği verilen varlığı döndürür.
    #[must_use]
    pub fn varlik(&self, kimlik: VarlikKimligi) -> Option<&Varlik> {
        self.varliklar.get(kimlik.0)
    }

    /// Kimliği verilen varlığı değiştirilebilir olarak döndürür.
    #[must_use]
    pub fn varlik_mut(&mut self, kimlik: VarlikKimligi) -> Option<&mut Varlik> {
        self.varliklar.get_mut(kimlik.0)
    }

    /// Dünyadaki bütün varlıkları eklenme sırasıyla döndürür.
    #[must_use]
    pub fn varliklar(&self) -> &[Varlik] {
        &self.varliklar
    }

    /// Etkin kamerayı döndürür.
    #[must_use]
    pub const fn kamera(&self) -> &Kamera2B {
        &self.kamera
    }

    /// Etkin kamerayı değiştirilebilir olarak döndürür.
    #[must_use]
    pub const fn kamera_mut(&mut self) -> &mut Kamera2B {
        &mut self.kamera
    }

    /// Etkin kamerayı değiştirir.
    pub const fn kamerayi_ayarla(&mut self, kamera: Kamera2B) {
        self.kamera = kamera;
    }
}

#[cfg(test)]
mod testler {
    use tgame_matematik::{Renk, Vektor2};

    use super::{Donusum2B, Dunya, Varlik};

    fn yakin(sol: f32, sag: f32) -> bool {
        (sol - sag).abs() < 0.000_01
    }

    #[test]
    fn eklenen_varlik_sabit_kimlikle_bulunur() {
        let mut dunya = Dunya::yeni();
        let kimlik = dunya.varlik_ekle(Varlik::ucgen("Oyuncu", Renk::MAVI));

        let varlik = dunya.varlik(kimlik).expect("Eklenen varlık bulunmalı.");
        assert_eq!(varlik.ad(), "Oyuncu");
        assert_eq!(varlik.kimlik(), Some(kimlik));
    }

    #[test]
    fn varlik_donusumu_hareket_ettirilebilir() {
        let mut dunya = Dunya::yeni();
        let kimlik = dunya.varlik_ekle(
            Varlik::ucgen("Oyuncu", Renk::YESIL)
                .donusum(Donusum2B::yeni().konum(Vektor2::yeni(1.0, 2.0))),
        );

        dunya
            .varlik_mut(kimlik)
            .expect("Oyuncu bulunmalı.")
            .donusumu_mut()
            .tasi(Vektor2::SAG * 3.0);

        let konum = dunya
            .varlik(kimlik)
            .expect("Oyuncu bulunmalı.")
            .donusumu()
            .konum;
        assert!(yakin(konum.x, 4.0));
        assert!(yakin(konum.y, 2.0));
    }
}
