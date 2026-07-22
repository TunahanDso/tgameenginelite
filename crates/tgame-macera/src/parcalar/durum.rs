/// Tek kullanımlı veya sürekli çalışan içerik davranışıdır.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tekrarlama {
    /// İçerik yalnızca ilk başarılı çalışmada tüketilir.
    #[default]
    BirKez,
    /// İçerik koşulları sağlandığı her seferde çalışabilir.
    HerZaman,
}

/// Hikâye değişkenlerini türlerine göre saklayan kalıcı kara tahtadır.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OyunDurumu {
    bayraklar: BTreeMap<String, bool>,
    sayaclar: BTreeMap<String, i64>,
    metinler: BTreeMap<String, String>,
}

impl OyunDurumu {
    /// Boş bir oyun durumu oluşturur.
    #[must_use]
    pub fn yeni() -> Self {
        Self::default()
    }

    /// Bir mantıksal hikâye bayrağını ayarlar.
    pub fn bayrak_ayarla(&mut self, ad: impl Into<String>, deger: bool) {
        self.bayraklar.insert(ad.into(), deger);
    }

    /// Bir hikâye bayrağının değerini döndürür.
    #[must_use]
    pub fn bayrak(&self, ad: &str) -> bool {
        self.bayraklar.get(ad).copied().unwrap_or(false)
    }

    /// Bir sayacı doğrudan ayarlar.
    pub fn sayac_ayarla(&mut self, ad: impl Into<String>, deger: i64) {
        self.sayaclar.insert(ad.into(), deger);
    }

    /// Bir sayacı taşma denetimiyle değiştirir.
    ///
    /// # Errors
    ///
    /// Toplama `i64` sınırlarını aşarsa [`OyunHatasi`] döndürür.
    pub fn sayac_degistir(&mut self, ad: impl Into<String>, fark: i64) -> OyunSonucu<i64> {
        let ad = ad.into();
        let mevcut = self.sayac(&ad);
        let yeni = mevcut
            .checked_add(fark)
            .ok_or_else(|| OyunHatasi::yeni("Hikâye sayacı desteklenen sayı sınırını aştı."))?;
        self.sayaclar.insert(ad, yeni);
        Ok(yeni)
    }

    /// Bir sayacın değerini döndürür.
    #[must_use]
    pub fn sayac(&self, ad: &str) -> i64 {
        self.sayaclar.get(ad).copied().unwrap_or(0)
    }

    /// Bir metin değişkenini ayarlar.
    pub fn metin_ayarla(&mut self, ad: impl Into<String>, deger: impl Into<String>) {
        self.metinler.insert(ad.into(), deger.into());
    }

    /// Bir metin değişkenini döndürür.
    #[must_use]
    pub fn metin(&self, ad: &str) -> Option<&str> {
        self.metinler.get(ad).map(String::as_str)
    }

    /// Bütün çalışma zamanı değişkenlerini temizler.
    pub fn temizle(&mut self) {
        self.bayraklar.clear();
        self.sayaclar.clear();
        self.metinler.clear();
    }
}

/// Oyunda tanımlı bir eşyanın içerik bilgisidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EsyaTanimi {
    kimlik: EsyaKimligi,
    ad: String,
    aciklama: String,
    yigin_siniri: u32,
    gorev_esyasi: bool,
}

impl EsyaTanimi {
    /// Yeni bir eşya tanımı oluşturur.
    #[must_use]
    pub fn yeni(kimlik: impl Into<EsyaKimligi>, ad: impl Into<String>) -> Self {
        Self {
            kimlik: kimlik.into(),
            ad: ad.into(),
            aciklama: String::new(),
            yigin_siniri: 99,
            gorev_esyasi: false,
        }
    }

    /// Eşyanın açıklamasını değiştirir.
    #[must_use]
    pub fn aciklama(mut self, aciklama: impl Into<String>) -> Self {
        self.aciklama = aciklama.into();
        self
    }

    /// Bir yığındaki azami eşya sayısını değiştirir.
    #[must_use]
    pub fn yigin_siniri(mut self, sinir: u32) -> Self {
        self.yigin_siniri = sinir.max(1);
        self
    }

    /// Eşyayı hikâye tarafından korunan görev eşyası olarak işaretler.
    #[must_use]
    pub const fn gorev_esyasi(mut self) -> Self {
        self.gorev_esyasi = true;
        self
    }

    /// Eşya kimliğini döndürür.
    #[must_use]
    pub const fn kimlik(&self) -> &EsyaKimligi {
        &self.kimlik
    }

    /// Oyuncuya gösterilen eşya adını döndürür.
    #[must_use]
    pub fn ad(&self) -> &str {
        &self.ad
    }

    /// Oyuncuya gösterilen açıklamayı döndürür.
    #[must_use]
    pub fn aciklamasi(&self) -> &str {
        &self.aciklama
    }

    /// Yığın sınırını döndürür.
    #[must_use]
    pub const fn yigin_siniri_degeri(&self) -> u32 {
        self.yigin_siniri
    }

    /// Eşyanın görev eşyası olup olmadığını döndürür.
    #[must_use]
    pub const fn gorev_esyasi_mi(&self) -> bool {
        self.gorev_esyasi
    }
}

/// Oyuncunun kalıcı eşya miktarlarını saklar.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Envanter {
    miktarlar: BTreeMap<EsyaKimligi, u32>,
}

impl Envanter {
    /// Boş bir envanter oluşturur.
    #[must_use]
    pub fn yeni() -> Self {
        Self::default()
    }

    /// Bir eşyanın miktarını döndürür.
    #[must_use]
    pub fn miktar(&self, kimlik: &EsyaKimligi) -> u32 {
        self.miktarlar.get(kimlik).copied().unwrap_or(0)
    }

    /// Envanterde istenen miktarın bulunup bulunmadığını döndürür.
    #[must_use]
    pub fn var_mi(&self, kimlik: &EsyaKimligi, miktar: u32) -> bool {
        self.miktar(kimlik) >= miktar
    }

    fn miktar_ayarla(&mut self, kimlik: EsyaKimligi, miktar: u32) {
        if miktar == 0 {
            self.miktarlar.remove(&kimlik);
        } else {
            self.miktarlar.insert(kimlik, miktar);
        }
    }
}
