use tgame::onsoz::{
    Donusum3B, Dunya, FizikDunyasi, FizikGovdesi, Girdi, Kamera3B, MeshKimligi, ModelVerisi,
    Oyun, OyunAkisi, OyunHatasi, OyunSonucu, Renk, Sahne, Tus, Varlik, VarlikKimligi, Vektor3,
    Zaman,
};

const OYUNCU_HIZI: f32 = 4.8;
const ZIPLAMA_HIZI: f32 = 6.2;
const FARE_HASSASIYETI: f32 = 0.0025;
const KLAVYE_KAMERA_HIZI: f32 = 1.5;
const KAMERA_UZAKLIGI: f32 = 7.5;
const OYUNCU_OLCEGI: Vektor3 = Vektor3::yeni(0.75, 0.75, 0.75);

struct SahneKurulumu {
    dunya: Dunya,
    fizik: FizikDunyasi,
    oyuncu: VarlikKimligi,
    merkez: VarlikKimligi,
    piramitler: Vec<VarlikKimligi>,
}

struct OyunDurumu {
    fizik: FizikDunyasi,
    oyuncu: VarlikKimligi,
    merkez: VarlikKimligi,
    piramitler: Vec<VarlikKimligi>,
    kamera_yatay: f32,
    kamera_dikey: f32,
}

impl OyunDurumu {
    fn yeni(kurulum: SahneKurulumu) -> (Dunya, Self) {
        let SahneKurulumu {
            dunya,
            fizik,
            oyuncu,
            merkez,
            piramitler,
        } = kurulum;
        (
            dunya,
            Self {
                fizik,
                oyuncu,
                merkez,
                piramitler,
                kamera_yatay: 0.0,
                kamera_dikey: 0.35,
            },
        )
    }

    fn guncelle(&mut self, girdi: &Girdi, zaman: &Zaman, dunya: &mut Dunya) -> OyunAkisi {
        let kare_saniyesi = zaman.kare_saniyesi().min(0.05);
        self.kamera_acisini_guncelle(girdi, kare_saniyesi);
        let yon = self.hareket_yonu(girdi);
        let oyuncu_konumu = self.oyuncuyu_guncelle(girdi, zaman, dunya, yon);
        self.sahneyi_canlandir(dunya, kare_saniyesi);
        self.kamerayi_yerlestir(dunya, oyuncu_konumu);
        self.durum_yazdir(girdi, zaman, oyuncu_konumu);

        if girdi.bu_kare_basildi_mi(Tus::Kacis) {
            OyunAkisi::Kapat
        } else {
            OyunAkisi::DevamEt
        }
    }

    fn kamera_acisini_guncelle(&mut self, girdi: &Girdi, kare_saniyesi: f32) {
        let fare = girdi.fare_hareketi();
        self.kamera_yatay -= fare.x * FARE_HASSASIYETI;
        self.kamera_dikey -= fare.y * FARE_HASSASIYETI;

        if girdi.basili_mi(Tus::Sol) {
            self.kamera_yatay -= KLAVYE_KAMERA_HIZI * kare_saniyesi;
        }
        if girdi.basili_mi(Tus::Sag) {
            self.kamera_yatay += KLAVYE_KAMERA_HIZI * kare_saniyesi;
        }
        if girdi.basili_mi(Tus::Yukari) {
            self.kamera_dikey += KLAVYE_KAMERA_HIZI * kare_saniyesi;
        }
        if girdi.basili_mi(Tus::Asagi) {
            self.kamera_dikey -= KLAVYE_KAMERA_HIZI * kare_saniyesi;
        }
        self.kamera_dikey = self.kamera_dikey.clamp(-0.65, 1.05);
    }

    fn hareket_yonu(&self, girdi: &Girdi) -> Vektor3 {
        let ileri = Vektor3::yeni(-self.kamera_yatay.sin(), 0.0, -self.kamera_yatay.cos());
        let sag = Vektor3::yeni(self.kamera_yatay.cos(), 0.0, -self.kamera_yatay.sin());
        let mut yon = Vektor3::SIFIR;

        if girdi.basili_mi(Tus::W) {
            yon += ileri;
        }
        if girdi.basili_mi(Tus::S) {
            yon -= ileri;
        }
        if girdi.basili_mi(Tus::A) {
            yon -= sag;
        }
        if girdi.basili_mi(Tus::D) {
            yon += sag;
        }

        yon.birim()
    }

    fn oyuncuyu_guncelle(
        &mut self,
        girdi: &Girdi,
        zaman: &Zaman,
        dunya: &mut Dunya,
        yon: Vektor3,
    ) -> Vektor3 {
        {
            let oyuncu_govdesi = self
                .fizik
                .govde_mut(self.oyuncu)
                .expect("Oyuncu fizik gövdesi oyun boyunca kalmalı.");
            oyuncu_govdesi.yatay_hizi_ayarla(yon * OYUNCU_HIZI);
            if girdi.bu_kare_basildi_mi(Tus::Bosluk) {
                oyuncu_govdesi.ziplat(ZIPLAMA_HIZI);
            }
        }

        self.fizik.guncelle(dunya, zaman.kare_suresi());
        let oyuncu = dunya
            .varlik_mut(self.oyuncu)
            .expect("Oyuncu varlığı oyun boyunca kalmalı.");
        oyuncu.donusumu3b_mut().donus_radyan.y = self.kamera_yatay;
        oyuncu.donusumu3b().konum
    }

    fn sahneyi_canlandir(&self, dunya: &mut Dunya, kare_saniyesi: f32) {
        dunya
            .varlik_mut(self.merkez)
            .expect("Merkez küp oyun boyunca kalmalı.")
            .donusumu3b_mut()
            .dondur(Vektor3::yeni(
                0.25 * kare_saniyesi,
                0.7 * kare_saniyesi,
                0.15 * kare_saniyesi,
            ));

        for (sira, kimlik) in self.piramitler.iter().copied().enumerate() {
            let yon = if sira & 1 == 0 { 1.0 } else { -1.0 };
            dunya
                .varlik_mut(kimlik)
                .expect("Piramit varlığı oyun boyunca kalmalı.")
                .donusumu3b_mut()
                .dondur(Vektor3::YUKARI * (yon * 0.45 * kare_saniyesi));
        }
    }

    fn kamerayi_yerlestir(&self, dunya: &mut Dunya, oyuncu_konumu: Vektor3) {
        let yatay_uzaklik = self.kamera_dikey.cos() * KAMERA_UZAKLIGI;
        let kamera_konumu = oyuncu_konumu
            + Vektor3::yeni(
                self.kamera_yatay.sin() * yatay_uzaklik,
                1.2 + self.kamera_dikey.sin() * KAMERA_UZAKLIGI,
                self.kamera_yatay.cos() * yatay_uzaklik,
            );
        let kamera = dunya.kamera3b_mut();
        kamera.konum = kamera_konumu;
        kamera.hedef = oyuncu_konumu + Vektor3::YUKARI * 0.3;
    }

    fn durum_yazdir(&self, girdi: &Girdi, zaman: &Zaman, oyuncu_konumu: Vektor3) {
        if !girdi.bu_kare_basildi_mi(Tus::Enter) {
            return;
        }

        let govde = self
            .fizik
            .govde(self.oyuncu)
            .expect("Oyuncu fizik gövdesi bulunmalı.");
        println!(
            "Oyuncu ({:.2}, {:.2}, {:.2}) — hız ({:.2}, {:.2}, {:.2}) — zeminde: {} — {}. kare",
            oyuncu_konumu.x,
            oyuncu_konumu.y,
            oyuncu_konumu.z,
            govde.hiz().x,
            govde.hiz().y,
            govde.hiz().z,
            govde.zeminde_mi(),
            zaman.kare_sayisi(),
        );
    }
}

fn main() -> OyunSonucu {
    let (dunya, mut durum) = OyunDurumu::yeni(sahneyi_olustur()?);

    Oyun::yeni("Tgame 3B Fizik ve glTF Dünyası")
        .cozunurluk(960, 640)
        .mod_klasoru("modlar")
        .sahne_ekle(Sahne::yeni("3B Mesh Başlangıcı"))
        .dunya(dunya)
        .her_kare(move |girdi, zaman, dunya| durum.guncelle(girdi, zaman, dunya))
        .calistir()
}

fn sahneyi_olustur() -> OyunSonucu<SahneKurulumu> {
    let mut dunya = Dunya::yeni_3b();
    let mut fizik = FizikDunyasi::yeni();
    dunya.kamera3b_ayarla(
        Kamera3B::yeni()
            .konum(Vektor3::yeni(0.0, 4.2, 8.0))
            .hedef(Vektor3::SIFIR)
            .kirpma(0.1, 250.0),
    );

    zemin_ekle(&mut dunya, &mut fizik);
    sutunlari_ekle(&mut dunya, &mut fizik);
    let piramit_mesh = piramit_meshini_yukle(&mut dunya)?;
    let piramitler = piramitleri_ekle(&mut dunya, &mut fizik, piramit_mesh);

    let oyuncu = dunya.varlik_ekle(
        Varlik::kup("Oyuncu", Renk::SARI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 2.5, 3.0))
                .olcek(OYUNCU_OLCEGI),
        ),
    );
    fizik.govde_ekle(FizikGovdesi::dinamik_kup(oyuncu, OYUNCU_OLCEGI));

    let merkez_olcegi = Vektor3::yeni(1.7, 1.7, 1.7);
    let merkez = dunya.varlik_ekle(
        Varlik::kup("Dönen Merkez", Renk::KIRMIZI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 1.35, 0.0))
                .olcek(merkez_olcegi)
                .donus(Vektor3::yeni(0.25, 0.35, 0.1)),
        ),
    );
    fizik.govde_ekle(FizikGovdesi::statik_kup(merkez, merkez_olcegi));

    Ok(SahneKurulumu {
        dunya,
        fizik,
        oyuncu,
        merkez,
        piramitler,
    })
}

fn piramit_meshini_yukle(dunya: &mut Dunya) -> OyunSonucu<MeshKimligi> {
    let model = ModelVerisi::gltf_yukle("assets/piramit.gltf")?;
    dunya
        .model_ekle(model)
        .first()
        .copied()
        .ok_or_else(|| OyunHatasi::yeni("Piramit modeli kayıtlı mesh üretmedi."))
}

fn piramitleri_ekle(
    dunya: &mut Dunya,
    fizik: &mut FizikDunyasi,
    mesh: MeshKimligi,
) -> Vec<VarlikKimligi> {
    let piramitler = [
        (Vektor3::yeni(-3.2, -0.58, -2.8), Renk::MAVI),
        (Vektor3::yeni(-1.6, -0.58, -4.2), Renk::YESIL),
        (Vektor3::yeni(1.6, -0.58, -4.2), Renk::KIRMIZI),
        (Vektor3::yeni(3.2, -0.58, -2.8), Renk::SARI),
        (Vektor3::yeni(-3.2, -0.58, 2.8), Renk::KIRMIZI),
        (Vektor3::yeni(-1.6, -0.58, 4.2), Renk::SARI),
        (Vektor3::yeni(1.6, -0.58, 4.2), Renk::MAVI),
        (Vektor3::yeni(3.2, -0.58, 2.8), Renk::YESIL),
    ];
    let goruntu_olcegi = Vektor3::yeni(0.65, 0.65, 0.65);
    let carpismа_olcegi = Vektor3::yeni(1.3, 1.17, 1.3);
    let mut kimlikler = Vec::with_capacity(piramitler.len());

    for (konum, renk) in piramitler {
        let kimlik = dunya.varlik_ekle(
            Varlik::mesh("glTF Piramit", mesh, renk).donusum3b(
                Donusum3B::yeni()
                    .konum(konum)
                    .olcek(goruntu_olcegi),
            ),
        );
        kimlikler.push(kimlik);

        let engel = dunya.varlik_ekle(
            Varlik::yeni("Piramit Çarpışması").donusum3b(
                Donusum3B::yeni().konum(konum + Vektor3::YUKARI * 0.585),
            ),
        );
        fizik.govde_ekle(FizikGovdesi::statik_kup(engel, carpismа_olcegi));
    }

    kimlikler
}

fn zemin_ekle(dunya: &mut Dunya, fizik: &mut FizikDunyasi) {
    let koordinatlar = [-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let olcek = Vektor3::yeni(0.96, 0.12, 0.96);
    let mut acik_renk = false;

    for z in koordinatlar {
        for x in koordinatlar {
            let renk = if acik_renk {
                Renk::yeni(0.18, 0.24, 0.34, 1.0)
            } else {
                Renk::yeni(0.10, 0.14, 0.22, 1.0)
            };
            let kimlik = dunya.varlik_ekle(
                Varlik::kup("Zemin", renk).donusum3b(
                    Donusum3B::yeni()
                        .konum(Vektor3::yeni(x, -0.65, z))
                        .olcek(olcek),
                ),
            );
            fizik.govde_ekle(FizikGovdesi::statik_kup(kimlik, olcek));
            acik_renk = !acik_renk;
        }
        acik_renk = !acik_renk;
    }
}

fn sutunlari_ekle(dunya: &mut Dunya, fizik: &mut FizikDunyasi) {
    let sutunlar = [
        (Vektor3::yeni(-4.0, 0.4, -4.0), Renk::MAVI, 2.0),
        (Vektor3::yeni(4.0, 0.9, -4.0), Renk::YESIL, 3.0),
        (Vektor3::yeni(-4.0, 1.4, 4.0), Renk::KIRMIZI, 4.0),
        (Vektor3::yeni(4.0, 0.65, 4.0), Renk::SARI, 2.5),
        (Vektor3::yeni(-2.5, 0.15, 0.0), Renk::YESIL, 1.5),
        (Vektor3::yeni(2.5, 0.15, 0.0), Renk::MAVI, 1.5),
    ];

    for (konum, renk, yukseklik) in sutunlar {
        let olcek = Vektor3::yeni(0.8, yukseklik, 0.8);
        let kimlik = dunya.varlik_ekle(
            Varlik::kup("Sütun", renk)
                .donusum3b(Donusum3B::yeni().konum(konum).olcek(olcek)),
        );
        fizik.govde_ekle(FizikGovdesi::statik_kup(kimlik, olcek));
    }
}
