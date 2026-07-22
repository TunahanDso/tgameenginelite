/// Bir diyalog seçimidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiyalogSecenegi {
    kimlik: String,
    metin: String,
    kosullar: Vec<Kosul>,
    eylemler: Vec<Eylem>,
    sonraki: Option<String>,
}

impl DiyalogSecenegi {
    /// Yeni bir diyalog seçeneği oluşturur.
    #[must_use]
    pub fn yeni(kimlik: impl Into<String>, metin: impl Into<String>) -> Self {
        Self {
            kimlik: kimlik.into(),
            metin: metin.into(),
            kosullar: Vec::new(),
            eylemler: Vec::new(),
            sonraki: None,
        }
    }

    /// Seçeneğin görünme koşullarını değiştirir.
    #[must_use]
    pub fn kosullar(mut self, kosullar: Vec<Kosul>) -> Self {
        self.kosullar = kosullar;
        self
    }

    /// Seçildiğinde uygulanacak eylemleri değiştirir.
    #[must_use]
    pub fn eylemler(mut self, eylemler: Vec<Eylem>) -> Self {
        self.eylemler = eylemler;
        self
    }

    /// Seçimden sonra açılacak düğümü belirler.
    #[must_use]
    pub fn sonraki(mut self, dugum: impl Into<String>) -> Self {
        self.sonraki = Some(dugum.into());
        self
    }
}

/// Bir konuşmacı satırı ve dallanma seçeneklerinden oluşan diyalog düğümüdür.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiyalogDugumu {
    kimlik: String,
    konusan: String,
    metin: String,
    secenekler: Vec<DiyalogSecenegi>,
    sonraki: Option<String>,
    giris_eylemleri: Vec<Eylem>,
}

impl DiyalogDugumu {
    /// Yeni bir diyalog düğümü oluşturur.
    #[must_use]
    pub fn yeni(
        kimlik: impl Into<String>,
        konusan: impl Into<String>,
        metin: impl Into<String>,
    ) -> Self {
        Self {
            kimlik: kimlik.into(),
            konusan: konusan.into(),
            metin: metin.into(),
            secenekler: Vec::new(),
            sonraki: None,
            giris_eylemleri: Vec::new(),
        }
    }

    /// Düğüme oyuncu seçenekleri ekler.
    #[must_use]
    pub fn secenekler(mut self, secenekler: Vec<DiyalogSecenegi>) -> Self {
        self.secenekler = secenekler;
        self
    }

    /// Seçeneksiz satırdan sonra açılacak düğümü belirler.
    #[must_use]
    pub fn sonraki(mut self, dugum: impl Into<String>) -> Self {
        self.sonraki = Some(dugum.into());
        self
    }

    /// Düğüm açıldığında uygulanacak eylemleri belirler.
    #[must_use]
    pub fn giris_eylemleri(mut self, eylemler: Vec<Eylem>) -> Self {
        self.giris_eylemleri = eylemler;
        self
    }
}

/// Kimlikli düğümlerden oluşan diyalog ağacıdır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiyalogTanimi {
    kimlik: DiyalogKimligi,
    baslangic: String,
    dugumler: BTreeMap<String, DiyalogDugumu>,
}

impl DiyalogTanimi {
    /// Yeni bir diyalog ağacı oluşturur.
    #[must_use]
    pub fn yeni(
        kimlik: impl Into<DiyalogKimligi>,
        baslangic: impl Into<String>,
        dugumler: Vec<DiyalogDugumu>,
    ) -> Self {
        Self {
            kimlik: kimlik.into(),
            baslangic: baslangic.into(),
            dugumler: dugumler
                .into_iter()
                .map(|dugum| (dugum.kimlik.clone(), dugum))
                .collect(),
        }
    }

    /// Diyalog kimliğini döndürür.
    #[must_use]
    pub const fn kimlik(&self) -> &DiyalogKimligi {
        &self.kimlik
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EtkinDiyalog {
    diyalog: DiyalogKimligi,
    dugum: String,
}

/// Oyuncuya gösterilecek kullanılabilir diyalog seçeneğidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiyalogSecenegiGorunumu {
    /// Seçimin oyun kodunda kullanılan kimliği.
    pub kimlik: String,
    /// Oyuncuya gösterilecek seçim metni.
    pub metin: String,
}

/// Etkin diyalog satırının kullanıcı arayüzü görünümüdür.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiyalogGorunumu {
    /// Etkin diyalog kimliği.
    pub diyalog: DiyalogKimligi,
    /// Etkin düğüm kimliği.
    pub dugum: String,
    /// Konuşmacının adı.
    pub konusan: String,
    /// Gösterilecek konuşma metni.
    pub metin: String,
    /// Koşulları sağlanan seçimler.
    pub secenekler: Vec<DiyalogSecenegiGorunumu>,
    /// Seçenek yokken satırın ilerletilip ilerletilemeyeceği.
    pub ilerletilebilir: bool,
}
