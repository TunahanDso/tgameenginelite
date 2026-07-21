/// Oyunun kalıcı durumunda uygulanabilen veri odaklı eylemdir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Eylem {
    /// Hikâye bayrağını ayarlar.
    BayrakAyarla { ad: String, deger: bool },
    /// Hikâye sayacını değiştirir.
    SayacDegistir { ad: String, fark: i64 },
    /// Hikâye metnini ayarlar.
    MetinAyarla { ad: String, deger: String },
    /// Envantere eşya ekler.
    EsyaEkle { esya: EsyaKimligi, miktar: u32 },
    /// Envanterden eşya çıkarır.
    EsyaCikar { esya: EsyaKimligi, miktar: u32 },
    /// Görevi başlatır.
    GorevBaslat(GorevKimligi),
    /// Görevi başarısız yapar.
    GorevBasarisiz(GorevKimligi),
    /// Diyalog ağacını açar.
    DiyalogBaslat(DiyalogKimligi),
    /// Sahne geçişi ister.
    SahneGecisiIste { sahne: SahneKimligi, giris_noktasi: Option<String> },
    /// Kontrol noktasını etkinleştirir.
    KontrolNoktasiAyarla {
        kimlik: KontrolNoktasiKimligi,
        sahne: SahneKimligi,
        giris_noktasi: Option<String>,
    },
    /// Yeni oyun olayı yayınlar.
    OlayYayinla(Box<OyunOlayi>),
}

/// Etkin yeniden doğuş ve kayıt konumudur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KontrolNoktasi {
    /// Kontrol noktası kimliği.
    pub kimlik: KontrolNoktasiKimligi,
    /// Yeniden açılacak sahne.
    pub sahne: SahneKimligi,
    /// Sahnedeki isteğe bağlı giriş noktası.
    pub giris_noktasi: Option<String>,
}

/// Motor başlangıcında raporlanabilecek macera içerik sayılarıdır.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaceraRaporu {
    /// Tanımlı eşya sayısı.
    pub esya: usize,
    /// Tanımlı görev sayısı.
    pub gorev: usize,
    /// Tanımlı diyalog sayısı.
    pub diyalog: usize,
    /// Tanımlı sahne sayısı.
    pub sahne: usize,
    /// Tanımlı etkileşim sayısı.
    pub etkilesim: usize,
    /// Tanımlı alan sayısı.
    pub alan: usize,
    /// Tanımlı olay kuralı sayısı.
    pub kural: usize,
}

/// Hikâye, görev, diyalog, sahne, etkileşim ve kayıt durumunu birlikte yönetir.
#[derive(Debug, Clone, Default)]
pub struct Macera {
    durum: OyunDurumu,
    esya_tanimlari: BTreeMap<EsyaKimligi, EsyaTanimi>,
    envanter: Envanter,
    gorev_tanimlari: BTreeMap<GorevKimligi, GorevTanimi>,
    gorevler: BTreeMap<GorevKimligi, GorevIlerlemesi>,
    diyalog_tanimlari: BTreeMap<DiyalogKimligi, DiyalogTanimi>,
    etkin_diyalog: Option<EtkinDiyalog>,
    sahne_tanimlari: BTreeMap<SahneKimligi, SahneTanimi>,
    etkin_sahne: Option<SahneKimligi>,
    bekleyen_gecis: Option<SahneGecisi>,
    etkilesimler: BTreeMap<EtkilesimKimligi, EtkilesimNoktasi>,
    tuketilen_etkilesimler: BTreeSet<EtkilesimKimligi>,
    alanlar: BTreeMap<AlanKimligi, AlanTetikleyicisi>,
    etkin_alanlar: BTreeSet<AlanKimligi>,
    tuketilen_alanlar: BTreeSet<AlanKimligi>,
    kurallar: BTreeMap<KuralKimligi, OlayKurali>,
    calismis_kurallar: BTreeSet<KuralKimligi>,
    olay_kuyrugu: VecDeque<OyunOlayi>,
    son_olaylar: VecDeque<OyunOlayi>,
    oynama_suresi_milisaniye: u64,
    kontrol_noktasi: Option<KontrolNoktasi>,
}
