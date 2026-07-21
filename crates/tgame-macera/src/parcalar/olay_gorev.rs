/// Görevlerin ve kuralların dinlediği yüksek seviyeli oyun olayıdır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OyunOlayi {
    /// Oyun kodu tarafından adlandırılmış özel bir olaydır.
    Ozel { ad: String, miktar: u32 },
    /// Envantere eşya eklenmiştir.
    EsyaEklendi { esya: EsyaKimligi, miktar: u32 },
    /// Envanterden eşya çıkarılmıştır.
    EsyaCikarildi { esya: EsyaKimligi, miktar: u32 },
    /// Bir dünya nesnesiyle etkileşilmiştir.
    Etkilesim { etkilesim: EtkilesimKimligi },
    /// Oyuncu bir alana girmiştir.
    AlanaGirdi { alan: AlanKimligi },
    /// Oyuncu bir alandan çıkmıştır.
    AlandanCikti { alan: AlanKimligi },
    /// Adlandırılmış bir düşman yenilmiştir.
    DusmanYenildi { dusman: String },
    /// Bir diyalog başlamıştır.
    DiyalogBasladi { diyalog: DiyalogKimligi },
    /// Diyalogda bir seçim yapılmıştır.
    DiyalogSecildi { diyalog: DiyalogKimligi, secenek: String },
    /// Bir diyalog sona ermiştir.
    DiyalogBitti { diyalog: DiyalogKimligi },
    /// Bir görev etkinleştirilmiştir.
    GorevBasladi { gorev: GorevKimligi },
    /// Bir görev adımı tamamlanmıştır.
    GorevAdimiTamamlandi { gorev: GorevKimligi, adim: usize },
    /// Bir görev tamamen bitirilmiştir.
    GorevTamamlandi { gorev: GorevKimligi },
    /// Etkin sahne değiştirilmiştir.
    SahneDegisti { onceki: Option<SahneKimligi>, yeni: SahneKimligi },
    /// Yeni bir kontrol noktası etkinleştirilmiştir.
    KontrolNoktasi { kontrol_noktasi: KontrolNoktasiKimligi },
}

impl OyunOlayi {
    /// Olayın görev ilerlemesine uygulanacak miktarını döndürür.
    #[must_use]
    pub const fn miktar(&self) -> u32 {
        match self {
            Self::Ozel { miktar, .. }
            | Self::EsyaEklendi { miktar, .. }
            | Self::EsyaCikarildi { miktar, .. } => *miktar,
            _ => 1,
        }
    }
}

/// Bir olay kuralının veya görev hedefinin dinlediği olay desenidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OlayFiltresi {
    /// Her oyun olayıyla eşleşir.
    Herhangi,
    /// Adlandırılmış özel olayla eşleşir.
    Ozel(String),
    /// Belirli eşyanın eklenmesiyle eşleşir.
    EsyaEklendi(EsyaKimligi),
    /// Belirli eşyanın çıkarılmasıyla eşleşir.
    EsyaCikarildi(EsyaKimligi),
    /// Belirli etkileşimle eşleşir.
    Etkilesim(EtkilesimKimligi),
    /// Belirli alana girişle eşleşir.
    AlanaGirdi(AlanKimligi),
    /// Belirli alandan çıkışla eşleşir.
    AlandanCikti(AlanKimligi),
    /// Adlandırılmış düşmanın yenilmesiyle eşleşir.
    DusmanYenildi(String),
    /// Belirli diyalog seçimiyle eşleşir.
    DiyalogSecildi { diyalog: DiyalogKimligi, secenek: String },
    /// Belirli görevin başlamasıyla eşleşir.
    GorevBasladi(GorevKimligi),
    /// Belirli görevin tamamlanmasıyla eşleşir.
    GorevTamamlandi(GorevKimligi),
    /// Belirli sahneye geçişle eşleşir.
    SahneDegisti(SahneKimligi),
}

impl OlayFiltresi {
    /// Filtrenin verilen olayla eşleşip eşleşmediğini döndürür.
    #[must_use]
    pub fn eslesir(&self, olay: &OyunOlayi) -> bool {
        match (self, olay) {
            (Self::Herhangi, _) => true,
            (Self::Ozel(sol), OyunOlayi::Ozel { ad: sag, .. })
            | (
                Self::DusmanYenildi(sol),
                OyunOlayi::DusmanYenildi { dusman: sag },
            ) => sol == sag,
            (Self::EsyaEklendi(sol), OyunOlayi::EsyaEklendi { esya: sag, .. })
            | (Self::EsyaCikarildi(sol), OyunOlayi::EsyaCikarildi { esya: sag, .. }) => sol == sag,
            (Self::Etkilesim(sol), OyunOlayi::Etkilesim { etkilesim: sag }) => sol == sag,
            (Self::AlanaGirdi(sol), OyunOlayi::AlanaGirdi { alan: sag })
            | (Self::AlandanCikti(sol), OyunOlayi::AlandanCikti { alan: sag }) => sol == sag,
            (
                Self::DiyalogSecildi {
                    diyalog: sol_diyalog,
                    secenek: sol_secenek,
                },
                OyunOlayi::DiyalogSecildi {
                    diyalog: sag_diyalog,
                    secenek: sag_secenek,
                },
            ) => sol_diyalog == sag_diyalog && sol_secenek == sag_secenek,
            (Self::GorevBasladi(sol), OyunOlayi::GorevBasladi { gorev: sag })
            | (Self::GorevTamamlandi(sol), OyunOlayi::GorevTamamlandi { gorev: sag }) => sol == sag,
            (Self::SahneDegisti(sol), OyunOlayi::SahneDegisti { yeni: sag, .. }) => sol == sag,
            _ => false,
        }
    }
}

/// Bir görevin çalışma zamanı durumudur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GorevAsamasi {
    /// Görev henüz başlamamıştır.
    #[default]
    Kilitli,
    /// Görev oyuncunun güncel hedefleri arasındadır.
    Etkin,
    /// Görev başarıyla tamamlanmıştır.
    Tamamlandi,
    /// Görev başarısız olmuştur.
    Basarisiz,
}

/// İçerik koşullarının birleşebilir ifadesidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kosul {
    /// Her durumda sağlanır.
    HerZaman,
    /// Bayrağın istenen değerde olmasını bekler.
    Bayrak { ad: String, deger: bool },
    /// Sayacın alt sınıra ulaşmasını bekler.
    SayacEnAz { ad: String, deger: i64 },
    /// Sayacın üst sınırı aşmamasını bekler.
    SayacEnFazla { ad: String, deger: i64 },
    /// Metin değişkeninin istenen değer olmasını bekler.
    MetinEsit { ad: String, deger: String },
    /// Envanterde belirli miktarda eşya bulunmasını bekler.
    EsyaEnAz { esya: EsyaKimligi, miktar: u32 },
    /// Görevin belirli çalışma zamanı aşamasında olmasını bekler.
    GorevAsamasi { gorev: GorevKimligi, asama: GorevAsamasi },
    /// Belirli sahnenin etkin olmasını bekler.
    Sahne(SahneKimligi),
    /// Bütün alt koşulların sağlanmasını bekler.
    Tumu(Vec<Self>),
    /// Alt koşullardan en az birinin sağlanmasını bekler.
    Herhangi(Vec<Self>),
    /// Alt koşulun tersini bekler.
    Degil(Box<Self>),
}

/// Bir görev adımındaki otomatik olay hedefidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GorevHedefi {
    aciklama: String,
    filtre: OlayFiltresi,
    gereken: u32,
}

impl GorevHedefi {
    /// Olay filtresiyle ilerleyen görev hedefi oluşturur.
    #[must_use]
    pub fn yeni(aciklama: impl Into<String>, filtre: OlayFiltresi, gereken: u32) -> Self {
        Self {
            aciklama: aciklama.into(),
            filtre,
            gereken: gereken.max(1),
        }
    }

    /// Hedef açıklamasını döndürür.
    #[must_use]
    pub fn aciklama(&self) -> &str {
        &self.aciklama
    }

    /// Hedef için gereken toplam ilerlemeyi döndürür.
    #[must_use]
    pub const fn gereken(&self) -> u32 {
        self.gereken
    }
}

/// Bir görevin aynı anda izlenen hedef grubudur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GorevAdimi {
    baslik: String,
    aciklama: String,
    hedefler: Vec<GorevHedefi>,
}

impl GorevAdimi {
    /// Yeni bir görev adımı oluşturur.
    #[must_use]
    pub fn yeni(baslik: impl Into<String>, hedefler: Vec<GorevHedefi>) -> Self {
        Self {
            baslik: baslik.into(),
            aciklama: String::new(),
            hedefler,
        }
    }

    /// Adımın ayrıntılı açıklamasını değiştirir.
    #[must_use]
    pub fn aciklama(mut self, aciklama: impl Into<String>) -> Self {
        self.aciklama = aciklama.into();
        self
    }

    /// Adım başlığını döndürür.
    #[must_use]
    pub fn baslik(&self) -> &str {
        &self.baslik
    }

    /// Adım açıklamasını döndürür.
    #[must_use]
    pub fn aciklamasi(&self) -> &str {
        &self.aciklama
    }

    /// Adım hedeflerini döndürür.
    #[must_use]
    pub fn hedefler(&self) -> &[GorevHedefi] {
        &self.hedefler
    }
}

/// Birden fazla adımdan oluşan görev tanımıdır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GorevTanimi {
    kimlik: GorevKimligi,
    baslik: String,
    aciklama: String,
    adimlar: Vec<GorevAdimi>,
}

impl GorevTanimi {
    /// Yeni bir görev tanımı oluşturur.
    #[must_use]
    pub fn yeni(
        kimlik: impl Into<GorevKimligi>,
        baslik: impl Into<String>,
        adimlar: Vec<GorevAdimi>,
    ) -> Self {
        Self {
            kimlik: kimlik.into(),
            baslik: baslik.into(),
            aciklama: String::new(),
            adimlar,
        }
    }

    /// Görev açıklamasını değiştirir.
    #[must_use]
    pub fn aciklama(mut self, aciklama: impl Into<String>) -> Self {
        self.aciklama = aciklama.into();
        self
    }

    /// Görev kimliğini döndürür.
    #[must_use]
    pub const fn kimlik(&self) -> &GorevKimligi {
        &self.kimlik
    }

    /// Görev başlığını döndürür.
    #[must_use]
    pub fn baslik(&self) -> &str {
        &self.baslik
    }

    /// Görev açıklamasını döndürür.
    #[must_use]
    pub fn aciklamasi(&self) -> &str {
        &self.aciklama
    }

    /// Görev adımlarını döndürür.
    #[must_use]
    pub fn adimlar(&self) -> &[GorevAdimi] {
        &self.adimlar
    }
}

/// Bir görevin kayıt dosyasına yazılan çalışma zamanı ilerlemesidir.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GorevIlerlemesi {
    asama: GorevAsamasi,
    adim: usize,
    hedefler: Vec<u32>,
}

impl GorevIlerlemesi {
    /// Görevin çalışma zamanı aşamasını döndürür.
    #[must_use]
    pub const fn asama(&self) -> GorevAsamasi {
        self.asama
    }

    /// Etkin görev adımının sıra numarasını döndürür.
    #[must_use]
    pub const fn adim(&self) -> usize {
        self.adim
    }

    /// Etkin adımdaki hedef ilerlemelerini döndürür.
    #[must_use]
    pub fn hedefler(&self) -> &[u32] {
        &self.hedefler
    }
}
