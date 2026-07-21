//! Tgame Engine Lite varlık, dönüşüm, kamera ve dünya kaynak katmanı.

use tgame_cekirdek::{OyunHatasi, OyunSonucu};
use tgame_matematik::{Matris4, Renk, Vektor2, Vektor3};
use tgame_model::{DokuVerisi, MalzemeVerisi, MeshVerisi, ModelVerisi};

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

/// Dünya kayıt defterindeki bir mesh'i benzersiz biçimde tanımlar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeshKimligi(usize);

impl MeshKimligi {
    /// Kimliğin kayıt defterindeki sayısal değerini döndürür.
    #[must_use]
    pub const fn deger(self) -> usize {
        self.0
    }
}

/// Dünya kayıt defterindeki bir dokuyu benzersiz biçimde tanımlar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DokuKimligi(usize);

impl DokuKimligi {
    /// Kimliğin kayıt defterindeki sayısal değerini döndürür.
    #[must_use]
    pub const fn deger(self) -> usize {
        self.0
    }
}

/// Dünya kayıt defterindeki bir malzemeyi benzersiz biçimde tanımlar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MalzemeKimligi(usize);

impl MalzemeKimligi {
    /// Kimliğin kayıt defterindeki sayısal değerini döndürür.
    #[must_use]
    pub const fn deger(self) -> usize {
        self.0
    }
}

/// Dünya içindeki bağımsız malzeme kaydıdır.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MalzemeKaydi {
    temel_renk: Renk,
    temel_doku: Option<DokuKimligi>,
}

impl MalzemeKaydi {
    /// Renk ve isteğe bağlı doku kimliğinden malzeme kaydı oluşturur.
    #[must_use]
    pub const fn yeni(temel_renk: Renk, temel_doku: Option<DokuKimligi>) -> Self {
        Self {
            temel_renk,
            temel_doku,
        }
    }

    /// Malzemenin doğrusal temel rengini döndürür.
    #[must_use]
    pub const fn temel_renk(self) -> Renk {
        self.temel_renk
    }

    /// Malzemenin taban renk dokusu kimliğini döndürür.
    #[must_use]
    pub const fn temel_doku(self) -> Option<DokuKimligi> {
        self.temel_doku
    }
}

/// Dünyanın hangi grafik uzayında çizileceğini belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DunyaBoyutu {
    /// Ortografik iki boyutlu dünya.
    #[default]
    IkiBoyut,
    /// Perspektif ve derinlik tamponlu üç boyutlu dünya.
    UcBoyut,
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

/// Üç boyutlu konum, Euler dönüşü ve ölçek bilgisini taşır.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Donusum3B {
    /// Dünya konumu.
    pub konum: Vektor3,
    /// X, Y ve Z eksenlerindeki radyan dönüşler.
    pub donus_radyan: Vektor3,
    /// Yerel ölçek.
    pub olcek: Vektor3,
}

impl Donusum3B {
    /// Birim üç boyutlu dönüşüm oluşturur.
    #[must_use]
    pub const fn yeni() -> Self {
        Self {
            konum: Vektor3::SIFIR,
            donus_radyan: Vektor3::SIFIR,
            olcek: Vektor3::BIR,
        }
    }

    /// Konumu değiştirir.
    #[must_use]
    pub const fn konum(mut self, konum: Vektor3) -> Self {
        self.konum = konum;
        self
    }

    /// Ölçeği değiştirir.
    #[must_use]
    pub const fn olcek(mut self, olcek: Vektor3) -> Self {
        self.olcek = olcek;
        self
    }

    /// Euler dönüşünü radyan cinsinden değiştirir.
    #[must_use]
    pub const fn donus(mut self, donus_radyan: Vektor3) -> Self {
        self.donus_radyan = donus_radyan;
        self
    }

    /// Dönüşümü dünya yönünde taşır.
    pub fn tasi(&mut self, hareket: Vektor3) {
        self.konum += hareket;
    }

    /// Euler dönüşünü radyan cinsinden artırır.
    pub fn dondur(&mut self, donus_radyan: Vektor3) {
        self.donus_radyan += donus_radyan;
    }

    /// GPU çizimi için model matrisini oluşturur.
    #[must_use]
    pub fn model_matrisi(self) -> Matris4 {
        Matris4::model(self.konum, self.donus_radyan, self.olcek)
    }
}

impl Default for Donusum3B {
    fn default() -> Self {
        Self::yeni()
    }
}

/// Bir varlığın iki boyutlu çizilebilir görünümünü belirtir.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gorunum2B {
    /// Tek renkli üçgen görünümü.
    Ucgen {
        /// Üçgen rengi.
        renk: Renk,
    },
}

/// Bir varlığın üç boyutlu çizilebilir görünümünü belirtir.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gorunum3B {
    /// Yüzey normalleriyle aydınlatılan yerleşik küp görünümü.
    Kup {
        /// Küpün temel rengi.
        renk: Renk,
    },
    /// Dünya kayıt defterindeki genel mesh görünümü.
    Mesh {
        /// Çizilecek mesh'in kimliği.
        mesh: MeshKimligi,
        /// Varsayılanı geçersiz kılan isteğe bağlı malzeme kimliği.
        malzeme: Option<MalzemeKimligi>,
        /// Mesh'in varlık renk çarpanı.
        renk: Renk,
    },
}

/// Oyun dünyasındaki kimlikli nesnedir.
#[derive(Debug, Clone, PartialEq)]
pub struct Varlik {
    kimlik: Option<VarlikKimligi>,
    ad: String,
    donusum2b: Donusum2B,
    donusum3b: Donusum3B,
    kaynak_matrisi: Matris4,
    gorunum2b: Option<Gorunum2B>,
    gorunum3b: Option<Gorunum3B>,
    etkin: bool,
    gorunurluk_kirpmasini_atla: bool,
}

impl Varlik {
    /// Çizilebilir görünümü olmayan yeni bir varlık taslağı oluşturur.
    #[must_use]
    pub fn yeni(ad: impl Into<String>) -> Self {
        Self {
            kimlik: None,
            ad: ad.into(),
            donusum2b: Donusum2B::yeni(),
            donusum3b: Donusum3B::yeni(),
            kaynak_matrisi: Matris4::BIRIM,
            gorunum2b: None,
            gorunum3b: None,
            etkin: true,
            gorunurluk_kirpmasini_atla: false,
        }
    }

    /// Tek renkli üçgen varlık taslağı oluşturur.
    #[must_use]
    pub fn ucgen(ad: impl Into<String>, renk: Renk) -> Self {
        Self::yeni(ad).gorunum(Gorunum2B::Ucgen { renk })
    }

    /// Aydınlatılabilir tek renkli yerleşik küp varlık taslağı oluşturur.
    #[must_use]
    pub fn kup(ad: impl Into<String>, renk: Renk) -> Self {
        Self::yeni(ad).gorunum3b(Gorunum3B::Kup { renk })
    }

    /// Kayıtlı bir mesh'i varsayılan malzemesiyle kullanan varlık taslağı oluşturur.
    #[must_use]
    pub fn mesh(ad: impl Into<String>, mesh: MeshKimligi, renk: Renk) -> Self {
        Self::yeni(ad).gorunum3b(Gorunum3B::Mesh {
            mesh,
            malzeme: None,
            renk,
        })
    }

    /// Kayıtlı mesh'i seçilen bağımsız malzemeyle kullanan varlık taslağı oluşturur.
    #[must_use]
    pub fn mesh_malzemeli(
        ad: impl Into<String>,
        mesh: MeshKimligi,
        malzeme: MalzemeKimligi,
        renk: Renk,
    ) -> Self {
        Self::yeni(ad).gorunum3b(Gorunum3B::Mesh {
            mesh,
            malzeme: Some(malzeme),
            renk,
        })
    }

    /// Başlangıç iki boyutlu dönüşümünü değiştirir.
    #[must_use]
    pub const fn donusum(mut self, donusum: Donusum2B) -> Self {
        self.donusum2b = donusum;
        self
    }

    /// Başlangıç üç boyutlu dönüşümünü değiştirir.
    #[must_use]
    pub const fn donusum3b(mut self, donusum: Donusum3B) -> Self {
        self.donusum3b = donusum;
        self
    }

    /// glTF node veya başka kaynak tarafından sağlanan yerel matrisi değiştirir.
    #[must_use]
    pub const fn kaynak_matrisi(mut self, matris: Matris4) -> Self {
        self.kaynak_matrisi = matris;
        self
    }

    /// Varlığı kamera frustum kırpmasından muaf tutar.
    #[must_use]
    pub const fn her_zaman_ciz(mut self) -> Self {
        self.gorunurluk_kirpmasini_atla = true;
        self
    }

    /// İki boyutlu çizilebilir görünümü değiştirir.
    #[must_use]
    pub const fn gorunum(mut self, gorunum: Gorunum2B) -> Self {
        self.gorunum2b = Some(gorunum);
        self
    }

    /// Üç boyutlu çizilebilir görünümü değiştirir.
    #[must_use]
    pub const fn gorunum3b(mut self, gorunum: Gorunum3B) -> Self {
        self.gorunum3b = Some(gorunum);
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

    /// İki boyutlu dönüşümü döndürür.
    #[must_use]
    pub const fn donusumu(&self) -> &Donusum2B {
        &self.donusum2b
    }

    /// İki boyutlu dönüşümü değiştirilebilir olarak döndürür.
    #[must_use]
    pub const fn donusumu_mut(&mut self) -> &mut Donusum2B {
        &mut self.donusum2b
    }

    /// Üç boyutlu dönüşümü döndürür.
    #[must_use]
    pub const fn donusumu3b(&self) -> &Donusum3B {
        &self.donusum3b
    }

    /// Üç boyutlu dönüşümü değiştirilebilir olarak döndürür.
    #[must_use]
    pub const fn donusumu3b_mut(&mut self) -> &mut Donusum3B {
        &mut self.donusum3b
    }

    /// Kaynak yerel matrisini döndürür.
    #[must_use]
    pub const fn kaynak_matrisi_degeri(&self) -> Matris4 {
        self.kaynak_matrisi
    }

    /// Kullanıcı dönüşümüyle kaynak matrisini birleştirir.
    #[must_use]
    pub fn model_matrisi(&self) -> Matris4 {
        self.donusum3b.model_matrisi() * self.kaynak_matrisi
    }

    /// İki boyutlu görünümü döndürür.
    #[must_use]
    pub const fn gorunumu(&self) -> Option<Gorunum2B> {
        self.gorunum2b
    }

    /// Üç boyutlu görünümü döndürür.
    #[must_use]
    pub const fn gorunumu3b(&self) -> Option<Gorunum3B> {
        self.gorunum3b
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

    /// Varlığın frustum görünürlük kırpmasını atlayıp atlamadığını döndürür.
    #[must_use]
    pub const fn gorunurluk_kirpmasini_atlar_mi(&self) -> bool {
        self.gorunurluk_kirpmasini_atla
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

/// Perspektif üç boyutlu dünya kamerasını tanımlar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kamera3B {
    /// Kameranın dünya konumu.
    pub konum: Vektor3,
    /// Kameranın baktığı dünya noktası.
    pub hedef: Vektor3,
    /// Kameranın yukarı yönü.
    pub yukari: Vektor3,
    /// Dikey görüş açısı, radyan cinsinden.
    pub dikey_gorus_radyan: f32,
    /// Yakın kırpma düzlemi.
    pub yakin: f32,
    /// Uzak kırpma düzlemi.
    pub uzak: f32,
}

impl Kamera3B {
    /// Dengeli varsayılan değerlerle perspektif kamera oluşturur.
    #[must_use]
    pub const fn yeni() -> Self {
        Self {
            konum: Vektor3::yeni(0.0, 2.5, 7.0),
            hedef: Vektor3::SIFIR,
            yukari: Vektor3::YUKARI,
            dikey_gorus_radyan: std::f32::consts::FRAC_PI_3,
            yakin: 0.1,
            uzak: 200.0,
        }
    }

    /// Kamera konumunu değiştirir.
    #[must_use]
    pub const fn konum(mut self, konum: Vektor3) -> Self {
        self.konum = konum;
        self
    }

    /// Kameranın baktığı hedefi değiştirir.
    #[must_use]
    pub const fn hedef(mut self, hedef: Vektor3) -> Self {
        self.hedef = hedef;
        self
    }

    /// Dikey görüş açısını geçerliyse değiştirir.
    #[must_use]
    pub fn dikey_gorus_radyan(mut self, radyan: f32) -> Self {
        if radyan.is_finite() && radyan > 0.01 && radyan < std::f32::consts::PI - 0.01 {
            self.dikey_gorus_radyan = radyan;
        }
        self
    }

    /// Yakın ve uzak kırpma düzlemlerini geçerliyse değiştirir.
    #[must_use]
    pub fn kirpma(mut self, yakin: f32, uzak: f32) -> Self {
        if yakin.is_finite() && uzak.is_finite() && yakin > 0.0 && uzak > yakin {
            self.yakin = yakin;
            self.uzak = uzak;
        }
        self
    }

    /// Kamera görünüm ve perspektif matrislerinin birleşimini döndürür.
    #[must_use]
    pub fn gorunum_izdusum(self, en_boy_orani: f32) -> Matris4 {
        let guvenli_oran = guvenli_en_boy_orani(en_boy_orani);
        let gorunum = Matris4::bakis_sag_el(self.konum, self.hedef, self.yukari);
        let izdusum = Matris4::perspektif_sag_el(
            self.dikey_gorus_radyan,
            guvenli_oran,
            self.yakin,
            self.uzak,
        );
        izdusum * gorunum
    }

    /// Dünya uzayındaki bir sınır küresinin perspektif görüş hacminde olup olmadığını döndürür.
    #[must_use]
    pub fn kure_gorunur_mu(self, merkez: Vektor3, yaricap: f32, en_boy_orani: f32) -> bool {
        if !merkez.sonlu_mu() || !yaricap.is_finite() || yaricap < 0.0 {
            return true;
        }
        let ileri = (self.hedef - self.konum).birim();
        let sag = ileri.capraz(self.yukari).birim();
        let gercek_yukari = sag.capraz(ileri).birim();
        if ileri == Vektor3::SIFIR || sag == Vektor3::SIFIR || gercek_yukari == Vektor3::SIFIR {
            return true;
        }

        let kameradan = merkez - self.konum;
        let derinlik = kameradan.nokta(ileri);
        if derinlik + yaricap < self.yakin || derinlik - yaricap > self.uzak {
            return false;
        }

        let kesit_derinligi = derinlik.max(self.yakin);
        let yari_yukseklik = kesit_derinligi * (self.dikey_gorus_radyan * 0.5).tan();
        let yari_genislik = yari_yukseklik * guvenli_en_boy_orani(en_boy_orani);
        kameradan.nokta(sag).abs() <= yari_genislik + yaricap
            && kameradan.nokta(gercek_yukari).abs() <= yari_yukseklik + yaricap
    }
}

impl Default for Kamera3B {
    fn default() -> Self {
        Self::yeni()
    }
}

/// Varlıkları, geometriyi, dokuları, malzemeleri ve etkin kameraları saklar.
#[derive(Debug, Clone)]
pub struct Dunya {
    varliklar: Vec<Varlik>,
    meshler: Vec<MeshVerisi>,
    mesh_malzemeleri: Vec<MalzemeKimligi>,
    dokular: Vec<DokuVerisi>,
    malzemeler: Vec<MalzemeKaydi>,
    boyut: DunyaBoyutu,
    kamera2b: Kamera2B,
    kamera3b: Kamera3B,
}

impl Dunya {
    /// Boş bir iki boyutlu oyun dünyası oluşturur.
    #[must_use]
    pub fn yeni() -> Self {
        Self::default()
    }

    /// Boş bir perspektif üç boyutlu oyun dünyası oluşturur.
    #[must_use]
    pub fn yeni_3b() -> Self {
        Self {
            boyut: DunyaBoyutu::UcBoyut,
            ..Self::default()
        }
    }

    /// Dünyanın etkin grafik boyutunu döndürür.
    #[must_use]
    pub const fn boyut(&self) -> DunyaBoyutu {
        self.boyut
    }

    /// Dünyaya varlık ekler ve sabit kimliğini döndürür.
    pub fn varlik_ekle(&mut self, mut varlik: Varlik) -> VarlikKimligi {
        let kimlik = VarlikKimligi(self.varliklar.len());
        varlik.kimlik = Some(kimlik);
        self.varliklar.push(varlik);
        kimlik
    }

    /// Dünyaya doku ekler; eşit kaynak daha önce kaydedildiyse mevcut kimliği döndürür.
    pub fn doku_ekle(&mut self, doku: DokuVerisi) -> DokuKimligi {
        if let Some(indeks) = self.dokular.iter().position(|kayit| kayit == &doku) {
            return DokuKimligi(indeks);
        }
        let kimlik = DokuKimligi(self.dokular.len());
        self.dokular.push(doku);
        kimlik
    }

    /// Dünyaya malzeme ekler; eşit renk ve doku bileşimi mevcutsa onu tekrar kullanır.
    pub fn malzeme_ekle(&mut self, malzeme: MalzemeVerisi) -> MalzemeKimligi {
        let (temel_renk, temel_doku) = malzeme.parcalara_ayir();
        let temel_doku = temel_doku.map(|doku| self.doku_ekle(doku));
        let kayit = MalzemeKaydi::yeni(temel_renk, temel_doku);
        if let Some(indeks) = self
            .malzemeler
            .iter()
            .position(|mevcut| *mevcut == kayit)
        {
            return MalzemeKimligi(indeks);
        }
        let kimlik = MalzemeKimligi(self.malzemeler.len());
        self.malzemeler.push(kayit);
        kimlik
    }

    /// Dünyaya tek mesh ekler; içindeki malzeme ve doku bağımsız kayda ayrılır.
    pub fn mesh_ekle(&mut self, mesh: MeshVerisi) -> MeshKimligi {
        let (mesh, malzeme) = mesh.geometri_ve_malzemeye_ayir();
        let malzeme = self.malzeme_ekle(malzeme);
        let kimlik = MeshKimligi(self.meshler.len());
        self.meshler.push(mesh);
        self.mesh_malzemeleri.push(malzeme);
        kimlik
    }

    /// Modeldeki bütün mesh'leri dünyaya ekleyip kimliklerini döndürür.
    pub fn model_ekle(&mut self, model: ModelVerisi) -> Vec<MeshKimligi> {
        model
            .meshlere_ayir()
            .into_iter()
            .map(|mesh| self.mesh_ekle(mesh))
            .collect()
    }

    /// Modelin varsayılan glTF sahnesini node dönüşümleriyle varlıklara dönüştürür.
    ///
    /// # Errors
    ///
    /// Bir sahne örneği model içinde bulunmayan mesh sıra numarasına işaret ederse
    /// [`OyunHatasi`] döndürür.
    pub fn model_sahnesi_ekle(
        &mut self,
        ad_koku: impl Into<String>,
        model: ModelVerisi,
        renk: Renk,
    ) -> OyunSonucu<Vec<VarlikKimligi>> {
        let ad_koku = ad_koku.into();
        let (meshler, ornekler) = model.parcalara_ayir();
        let mesh_kimlikleri = meshler
            .into_iter()
            .map(|mesh| self.mesh_ekle(mesh))
            .collect::<Vec<_>>();
        let mut kimlikler = Vec::with_capacity(ornekler.len());

        for (sira, ornek) in ornekler.into_iter().enumerate() {
            let mesh = mesh_kimlikleri
                .get(ornek.mesh_indeksi())
                .copied()
                .ok_or_else(|| {
                    OyunHatasi::yeni("Model sahnesi bulunmayan bir mesh sıra numarası içeriyor.")
                })?;
            let varlik = Varlik::mesh(format!("{ad_koku} {}", sira + 1), mesh, renk)
                .kaynak_matrisi(ornek.dunya_matrisi());
            kimlikler.push(self.varlik_ekle(varlik));
        }
        Ok(kimlikler)
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

    /// Kimliği verilen mesh verisini döndürür.
    #[must_use]
    pub fn mesh(&self, kimlik: MeshKimligi) -> Option<&MeshVerisi> {
        self.meshler.get(kimlik.0)
    }

    /// Mesh'in içe aktarılmış varsayılan malzeme kimliğini döndürür.
    #[must_use]
    pub fn mesh_malzemesi(&self, kimlik: MeshKimligi) -> Option<MalzemeKimligi> {
        self.mesh_malzemeleri.get(kimlik.0).copied()
    }

    /// Kimliği verilen doku verisini döndürür.
    #[must_use]
    pub fn doku(&self, kimlik: DokuKimligi) -> Option<&DokuVerisi> {
        self.dokular.get(kimlik.0)
    }

    /// Kimliği verilen bağımsız malzeme kaydını döndürür.
    #[must_use]
    pub fn malzeme(&self, kimlik: MalzemeKimligi) -> Option<MalzemeKaydi> {
        self.malzemeler.get(kimlik.0).copied()
    }

    /// Dünyadaki bütün varlıkları eklenme sırasıyla döndürür.
    #[must_use]
    pub fn varliklar(&self) -> &[Varlik] {
        &self.varliklar
    }

    /// Dünyadaki bütün mesh kaynaklarını eklenme sırasıyla döndürür.
    #[must_use]
    pub fn meshler(&self) -> &[MeshVerisi] {
        &self.meshler
    }

    /// Mesh kayıtlarıyla aynı sıradaki varsayılan malzeme kimliklerini döndürür.
    #[must_use]
    pub fn mesh_malzemeleri(&self) -> &[MalzemeKimligi] {
        &self.mesh_malzemeleri
    }

    /// Tekilleştirilmiş bütün doku kaynaklarını döndürür.
    #[must_use]
    pub fn dokular(&self) -> &[DokuVerisi] {
        &self.dokular
    }

    /// Tekilleştirilmiş bütün malzeme kayıtlarını döndürür.
    #[must_use]
    pub fn malzemeler(&self) -> &[MalzemeKaydi] {
        &self.malzemeler
    }

    /// Etkin iki boyutlu kamerayı döndürür.
    #[must_use]
    pub const fn kamera(&self) -> &Kamera2B {
        &self.kamera2b
    }

    /// Etkin iki boyutlu kamerayı değiştirilebilir olarak döndürür.
    #[must_use]
    pub const fn kamera_mut(&mut self) -> &mut Kamera2B {
        &mut self.kamera2b
    }

    /// Etkin iki boyutlu kamerayı değiştirir.
    pub const fn kamerayi_ayarla(&mut self, kamera: Kamera2B) {
        self.kamera2b = kamera;
    }

    /// Etkin üç boyutlu kamerayı döndürür.
    #[must_use]
    pub const fn kamera3b(&self) -> &Kamera3B {
        &self.kamera3b
    }

    /// Etkin üç boyutlu kamerayı değiştirilebilir olarak döndürür.
    #[must_use]
    pub const fn kamera3b_mut(&mut self) -> &mut Kamera3B {
        &mut self.kamera3b
    }

    /// Etkin üç boyutlu kamerayı değiştirir.
    pub const fn kamera3b_ayarla(&mut self, kamera: Kamera3B) {
        self.kamera3b = kamera;
    }
}

impl Default for Dunya {
    fn default() -> Self {
        Self {
            varliklar: Vec::new(),
            meshler: Vec::new(),
            mesh_malzemeleri: Vec::new(),
            dokular: Vec::new(),
            malzemeler: Vec::new(),
            boyut: DunyaBoyutu::IkiBoyut,
            kamera2b: Kamera2B::default(),
            kamera3b: Kamera3B::default(),
        }
    }
}

fn guvenli_en_boy_orani(en_boy_orani: f32) -> f32 {
    if en_boy_orani.is_finite() && en_boy_orani > f32::EPSILON {
        en_boy_orani
    } else {
        1.0
    }
}

#[cfg(test)]
mod testler {
    use tgame_matematik::{Matris4, Renk, Vektor2, Vektor3};
    use tgame_model::{DokuVerisi, MalzemeVerisi, MeshVerisi, OrnekleyiciVerisi};

    use super::{Donusum2B, Donusum3B, Dunya, DunyaBoyutu, Kamera3B, Varlik};

    fn yakin(sol: f32, sag: f32) -> bool {
        (sol - sag).abs() < 0.000_01
    }

    fn test_mesh(malzeme: MalzemeVerisi) -> MeshVerisi {
        MeshVerisi::yeni_malzemeli(
            vec![Vektor3::SIFIR, Vektor3::SAG, Vektor3::YUKARI],
            vec![Vektor3::ILERI; 3],
            vec![Vektor2::SIFIR, Vektor2::SAG, Vektor2::YUKARI],
            vec![0, 1, 2],
            malzeme,
        )
        .expect("Test mesh'i geçerli olmalı.")
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

    #[test]
    fn ayni_doku_ve_malzeme_tek_kayit_olur() {
        let doku = DokuVerisi::yeni_rgba8(
            1,
            1,
            vec![255; 4],
            OrnekleyiciVerisi::default(),
        )
        .expect("Doku geçerli olmalı.");
        let malzeme = MalzemeVerisi::yeni(Renk::BEYAZ).temel_doku(doku);
        let mut dunya = Dunya::yeni_3b();
        let birinci = dunya.mesh_ekle(test_mesh(malzeme.clone()));
        let ikinci = dunya.mesh_ekle(test_mesh(malzeme));

        assert_eq!(dunya.dokular().len(), 1);
        assert_eq!(dunya.malzemeler().len(), 1);
        assert_eq!(dunya.mesh_malzemesi(birinci), dunya.mesh_malzemesi(ikinci));
    }

    #[test]
    fn kaynak_matrisi_kullanici_donusumunun_altinda_kalir() {
        let varlik = Varlik::yeni("Node")
            .donusum3b(Donusum3B::yeni().konum(Vektor3::new(2.0, 0.0, 0.0)))
            .kaynak_matrisi(Matris4::oteleme(Vektor3::yeni(0.0, 3.0, 0.0)));
        let konum = varlik.model_matrisi().noktayi_donustur(Vektor3::SIFIR);

        assert_eq!(konum, Vektor3::yeni(2.0, 3.0, 0.0));
    }

    #[test]
    fn kamera_frustum_disindaki_kureyi_eler() {
        let kamera = Kamera3B::yeni()
            .konum(Vektor3::SIFIR)
            .hedef(Vektor3::ILERI)
            .kirpma(0.1, 50.0);

        assert!(kamera.kure_gorunur_mu(Vektor3::yeni(0.0, 0.0, -5.0), 0.5, 16.0 / 9.0));
        assert!(!kamera.kure_gorunur_mu(Vektor3::yeni(100.0, 0.0, -5.0), 0.5, 16.0 / 9.0));
        assert!(!kamera.kure_gorunur_mu(Vektor3::yeni(0.0, 0.0, 5.0), 0.5, 16.0 / 9.0));
    }

    #[test]
    fn uc_boyutlu_dunya_mesh_kaynagini_paylasir() {
        let mut dunya = Dunya::yeni_3b();
        let mesh = dunya.mesh_ekle(test_mesh(MalzemeVerisi::default()));
        let kimlik = dunya.varlik_ekle(
            Varlik::mesh("Mesh", mesh, Renk::SARI)
                .donusum3b(Donusum3B::yeni().konum(Vektor3::yeni(0.0, 1.0, -2.0))),
        );

        dunya
            .varlik_mut(kimlik)
            .expect("Mesh varlığı bulunmalı.")
            .donusumu3b_mut()
            .tasi(Vektor3::SAG * 2.0);

        assert_eq!(dunya.boyut(), DunyaBoyutu::UcBoyut);
        assert_eq!(dunya.meshler().len(), 1);
        assert!(yakin(
            dunya
                .varlik(kimlik)
                .expect("Mesh varlığı bulunmalı.")
                .donusumu3b()
                .konum
                .x,
            2.0,
        ));
    }
}
