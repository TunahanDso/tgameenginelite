/// Bir sahnedeki adlandırılmış giriş noktalarını tanımlar.
#[derive(Debug, Clone, PartialEq)]
pub struct SahneTanimi {
    kimlik: SahneKimligi,
    ad: String,
    giris_noktalari: BTreeMap<String, Vektor3>,
}

impl SahneTanimi {
    /// Yeni bir sahne tanımı oluşturur.
    #[must_use]
    pub fn yeni(kimlik: impl Into<SahneKimligi>, ad: impl Into<String>) -> Self {
        Self {
            kimlik: kimlik.into(),
            ad: ad.into(),
            giris_noktalari: BTreeMap::new(),
        }
    }

    /// Sahneye adlandırılmış bir giriş noktası ekler.
    #[must_use]
    pub fn giris_noktasi(mut self, ad: impl Into<String>, konum: Vektor3) -> Self {
        self.giris_noktalari.insert(ad.into(), konum);
        self
    }

    /// Sahne kimliğini döndürür.
    #[must_use]
    pub const fn kimlik(&self) -> &SahneKimligi {
        &self.kimlik
    }

    /// Oyuncuya gösterilecek sahne adını döndürür.
    #[must_use]
    pub fn ad(&self) -> &str {
        &self.ad
    }

    /// Adlandırılmış giriş noktasının konumunu döndürür.
    #[must_use]
    pub fn giris_konumu(&self, ad: &str) -> Option<Vektor3> {
        self.giris_noktalari.get(ad).copied()
    }
}

/// Oyun kodunun dünyayı yeniden kurmak için tükettiği sahne geçiş isteğidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SahneGecisi {
    /// Geçişten önceki sahne.
    pub onceki: Option<SahneKimligi>,
    /// Açılacak yeni sahne.
    pub hedef: SahneKimligi,
    /// Yeni sahnedeki isteğe bağlı giriş noktası.
    pub giris_noktasi: Option<String>,
}

/// Oyuncuya gösterilecek en yakın etkileşim bilgisidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtkilesimGorunumu {
    /// Etkileşim kimliği.
    pub kimlik: EtkilesimKimligi,
    /// Oyuncuya gösterilecek komut metni.
    pub ileti: String,
}

/// Dünya konumu, menzil ve eylemler taşıyan etkileşim noktasıdır.
#[derive(Debug, Clone, PartialEq)]
pub struct EtkilesimNoktasi {
    kimlik: EtkilesimKimligi,
    ileti: String,
    konum: Vektor3,
    yaricap: f32,
    kosullar: Vec<Kosul>,
    eylemler: Vec<Eylem>,
    tekrarlama: Tekrarlama,
}

impl EtkilesimNoktasi {
    /// Yeni bir dünya etkileşimi oluşturur.
    #[must_use]
    pub fn yeni(
        kimlik: impl Into<EtkilesimKimligi>,
        ileti: impl Into<String>,
        konum: Vektor3,
        yaricap: f32,
    ) -> Self {
        let yaricap = if yaricap.is_finite() { yaricap.max(0.0) } else { 0.0 };
        Self {
            kimlik: kimlik.into(),
            ileti: ileti.into(),
            konum,
            yaricap,
            kosullar: Vec::new(),
            eylemler: Vec::new(),
            tekrarlama: Tekrarlama::BirKez,
        }
    }

    /// Etkileşim koşullarını değiştirir.
    #[must_use]
    pub fn kosullar(mut self, kosullar: Vec<Kosul>) -> Self {
        self.kosullar = kosullar;
        self
    }

    /// Etkileşim eylemlerini değiştirir.
    #[must_use]
    pub fn eylemler(mut self, eylemler: Vec<Eylem>) -> Self {
        self.eylemler = eylemler;
        self
    }

    /// Etkileşimin tekrar davranışını değiştirir.
    #[must_use]
    pub const fn tekrarlama(mut self, tekrarlama: Tekrarlama) -> Self {
        self.tekrarlama = tekrarlama;
        self
    }

    /// Etkileşim kimliğini döndürür.
    #[must_use]
    pub const fn kimlik(&self) -> &EtkilesimKimligi {
        &self.kimlik
    }
}

/// Eksenlere hizalı hikâye tetikleme hacmidir.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KutuAlan {
    merkez: Vektor3,
    yari_boyut: Vektor3,
}

impl KutuAlan {
    /// Merkez ve pozitif yarı boyuttan alan oluşturur.
    #[must_use]
    pub fn yeni(merkez: Vektor3, yari_boyut: Vektor3) -> Self {
        Self {
            merkez,
            yari_boyut: Vektor3::yeni(
                yari_boyut.x.abs(),
                yari_boyut.y.abs(),
                yari_boyut.z.abs(),
            ),
        }
    }

    /// Noktanın alan içinde olup olmadığını döndürür.
    #[must_use]
    pub fn icerir(self, nokta: Vektor3) -> bool {
        let fark = nokta - self.merkez;
        fark.x.abs() <= self.yari_boyut.x
            && fark.y.abs() <= self.yari_boyut.y
            && fark.z.abs() <= self.yari_boyut.z
    }
}

/// Alana giriş ve çıkışta hikâye eylemleri çalıştırır.
#[derive(Debug, Clone, PartialEq)]
pub struct AlanTetikleyicisi {
    kimlik: AlanKimligi,
    alan: KutuAlan,
    kosullar: Vec<Kosul>,
    giris_eylemleri: Vec<Eylem>,
    cikis_eylemleri: Vec<Eylem>,
    tekrarlama: Tekrarlama,
}

impl AlanTetikleyicisi {
    /// Yeni bir alan tetikleyicisi oluşturur.
    #[must_use]
    pub fn yeni(kimlik: impl Into<AlanKimligi>, alan: KutuAlan) -> Self {
        Self {
            kimlik: kimlik.into(),
            alan,
            kosullar: Vec::new(),
            giris_eylemleri: Vec::new(),
            cikis_eylemleri: Vec::new(),
            tekrarlama: Tekrarlama::BirKez,
        }
    }

    /// Tetikleyicinin çalışma koşullarını değiştirir.
    #[must_use]
    pub fn kosullar(mut self, kosullar: Vec<Kosul>) -> Self {
        self.kosullar = kosullar;
        self
    }

    /// Alana girişte uygulanacak eylemleri değiştirir.
    #[must_use]
    pub fn giris_eylemleri(mut self, eylemler: Vec<Eylem>) -> Self {
        self.giris_eylemleri = eylemler;
        self
    }

    /// Alandan çıkışta uygulanacak eylemleri değiştirir.
    #[must_use]
    pub fn cikis_eylemleri(mut self, eylemler: Vec<Eylem>) -> Self {
        self.cikis_eylemleri = eylemler;
        self
    }

    /// Tetikleyicinin tekrar davranışını değiştirir.
    #[must_use]
    pub const fn tekrarlama(mut self, tekrarlama: Tekrarlama) -> Self {
        self.tekrarlama = tekrarlama;
        self
    }

    /// Alan kimliğini döndürür.
    #[must_use]
    pub const fn kimlik(&self) -> &AlanKimligi {
        &self.kimlik
    }
}

/// Olay geldiğinde koşullu eylemler çalıştıran veri odaklı kuraldır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OlayKurali {
    kimlik: KuralKimligi,
    filtre: OlayFiltresi,
    kosullar: Vec<Kosul>,
    eylemler: Vec<Eylem>,
    tekrarlama: Tekrarlama,
}

impl OlayKurali {
    /// Yeni bir olay kuralı oluşturur.
    #[must_use]
    pub fn yeni(
        kimlik: impl Into<KuralKimligi>,
        filtre: OlayFiltresi,
        eylemler: Vec<Eylem>,
    ) -> Self {
        Self {
            kimlik: kimlik.into(),
            filtre,
            kosullar: Vec::new(),
            eylemler,
            tekrarlama: Tekrarlama::BirKez,
        }
    }

    /// Kural koşullarını değiştirir.
    #[must_use]
    pub fn kosullar(mut self, kosullar: Vec<Kosul>) -> Self {
        self.kosullar = kosullar;
        self
    }

    /// Kuralın tekrar davranışını değiştirir.
    #[must_use]
    pub const fn tekrarlama(mut self, tekrarlama: Tekrarlama) -> Self {
        self.tekrarlama = tekrarlama;
        self
    }

    /// Kural kimliğini döndürür.
    #[must_use]
    pub const fn kimlik(&self) -> &KuralKimligi {
        &self.kimlik
    }
}
