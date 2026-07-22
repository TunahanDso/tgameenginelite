from pathlib import Path


def oku(yol: str) -> str:
    return Path(yol).read_text(encoding="utf-8")


def yaz(yol: str, metin: str) -> None:
    Path(yol).write_text(metin, encoding="utf-8", newline="\n")


model = oku("crates/tgame-model/src/lib.rs")
if "pub mod uretim;" not in model:
    model = model.replace(
        "//! Tgame Engine Lite genel mesh, malzeme, doku ve glTF/GLB yükleme katmanı.\n",
        "//! Tgame Engine Lite genel mesh, malzeme, doku ve glTF/GLB yükleme katmanı.\n\npub mod uretim;\n",
        1,
    )
yaz("crates/tgame-model/src/lib.rs", model)

cargo = oku("crates/tgame/Cargo.toml")
if "tgame-oynanis" not in cargo:
    cargo = cargo.replace(
        'tgame-model = { path = "../tgame-model" }\n',
        'tgame-model = { path = "../tgame-model" }\ntgame-oynanis = { path = "../tgame-oynanis" }\n',
        1,
    )
yaz("crates/tgame/Cargo.toml", cargo)

tgame = oku("crates/tgame/src/lib.rs")
if "pub use tgame_model::uretim" not in tgame:
    tgame = tgame.replace(
        "    pub use tgame_model::{\n",
        "    pub use tgame_model::uretim::{AraziUreteci, koni, kure, silindir};\n    pub use tgame_model::{\n",
        1,
    )
if "pub use tgame_oynanis" not in tgame:
    tgame = tgame.replace(
        "    pub use tgame_pencere::OyunAkisi;\n",
        "    pub use tgame_oynanis::{AjanKarari, BasitAjan, Can, SavasDunyasi, SavasOlayi, Savasci, SesKuyrugu, SesOlayi, Takim};\n    pub use tgame_pencere::OyunAkisi;\n",
        1,
    )
yaz("crates/tgame/src/lib.rs", tgame)

sahne = oku("examples/ilk-oyun/src/sahne.rs")
sahne = sahne.replace("MalzemeVerisi::new(", "MalzemeVerisi::yeni(")
if "pub(crate) muhafiz: VarlikKimligi," not in sahne:
    sahne = sahne.replace(
        "    pub(crate) merkez: VarlikKimligi,\n",
        "    pub(crate) merkez: VarlikKimligi,\n    pub(crate) muhafiz: VarlikKimligi,\n",
        1,
    )
    sahne = sahne.replace(
        "    macera_isaretlerini_ekle(&mut dunya)?;",
        "    let muhafiz = macera_isaretlerini_ekle(&mut dunya)?;",
        1,
    )
    sahne = sahne.replace(
        "        merkez,\n        piramitler,",
        "        merkez,\n        muhafiz,\n        piramitler,",
        1,
    )
    sahne = sahne.replace(
        "fn macera_isaretlerini_ekle(dunya: &mut Dunya) -> OyunSonucu {",
        "fn macera_isaretlerini_ekle(dunya: &mut Dunya) -> OyunSonucu<VarlikKimligi> {",
        1,
    )
    sahne = sahne.replace(
        '    dunya.varlik_ekle(\n        Varlik::mesh("Taş Muhafız"',
        '    let muhafiz = dunya.varlik_ekle(\n        Varlik::mesh("Taş Muhafız"',
        1,
    )
    sahne = sahne.replace(
        "    );\n    Ok(())\n}\n\nfn piramit_meshini_yukle",
        "    );\n    Ok(muhafiz)\n}\n\nfn piramit_meshini_yukle",
        1,
    )
yaz("examples/ilk-oyun/src/sahne.rs", sahne)

oyun = oku("examples/ilk-oyun/src/oyun.rs")
if "mod ekran_arayuzu;" not in oyun:
    oyun = oyun.replace("mod macera_icerigi;\n", "mod ekran_arayuzu;\nmod macera_icerigi;\n", 1)
if "use ekran_arayuzu::arayuzu_guncelle;" not in oyun:
    oyun = oyun.replace(
        "use macera_icerigi",
        "use ekran_arayuzu::arayuzu_guncelle;\nuse macera_icerigi",
        1,
    )
if "    muhafiz: VarlikKimligi," not in oyun:
    oyun = oyun.replace(
        "    Dunya, FizikDunyasi, Girdi, GorevAsamasi, GorevIlerlemesi, KayitYoneticisi, Macera, Oyun,\n    OyunAkisi, OyunSonucu, Sahne, SahneKimligi, Tus, VarlikKimligi, Vektor3, Zaman,\n",
        "    AjanKarari, BasitAjan, Dunya, FizikDunyasi, Girdi, GorevAsamasi, GorevIlerlemesi,\n    KayitYoneticisi, Macera, Oyun, OyunAkisi, OyunOlayi, OyunSonucu, Sahne, SahneKimligi,\n    SavasDunyasi, SavasOlayi, Savasci, SesKuyrugu, SesOlayi, Takim, Tus, VarlikKimligi,\n    Vektor3, Zaman,\n",
        1,
    )
    oyun = oyun.replace(
        "    merkez: VarlikKimligi,\n",
        "    merkez: VarlikKimligi,\n    muhafiz: VarlikKimligi,\n",
        1,
    )
    oyun = oyun.replace(
        "    son_gorev_asamasi: GorevAsamasi,\n",
        "    son_gorev_asamasi: GorevAsamasi,\n    savas: SavasDunyasi,\n    muhafiz_ajani: BasitAjan,\n    sesler: SesKuyrugu,\n    gunluk_acik: bool,\n    bildirim: String,\n",
        1,
    )
    oyun = oyun.replace(
        "            merkez,\n            piramitler,",
        "            merkez,\n            muhafiz,\n            piramitler,",
        1,
    )
    oyun = oyun.replace(
        "        let durum = Self {\n",
        "        let mut savas = SavasDunyasi::yeni();\n        savas.savasci_ekle(oyuncu, Savasci::yeni(Takim::Oyuncu, 100.0, 24.0, 2.1, std::time::Duration::from_millis(520)));\n        savas.savasci_ekle(muhafiz, Savasci::yeni(Takim::Dusman, 120.0, 11.0, 1.8, std::time::Duration::from_millis(900)));\n        let durum = Self {\n",
        1,
    )
    oyun = oyun.replace(
        "            merkez,\n            piramitler,",
        "            merkez,\n            muhafiz,\n            piramitler,",
        1,
    )
    oyun = oyun.replace(
        "            son_gorev_asamasi: GorevAsamasi::Kilitli,\n",
        "            son_gorev_asamasi: GorevAsamasi::Kilitli,\n            savas,\n            muhafiz_ajani: BasitAjan::yeni(),\n            sesler: SesKuyrugu::yeni(),\n            gunluk_acik: false,\n            bildirim: \"Gözcü Aras'ı bul.\".to_owned(),\n",
        1,
    )
    oyun = oyun.replace(
        "        self.macera_girdisini_isle(girdi, dunya, macera, oyuncu_konumu);\n",
        "        self.macera_girdisini_isle(girdi, dunya, macera, oyuncu_konumu);\n        self.savasi_guncelle(girdi, zaman, dunya, macera, oyuncu_konumu);\n",
        1,
    )
    oyun = oyun.replace(
        "        self.kamerayi_yerlestir(dunya);\n",
        "        self.kamerayi_yerlestir(dunya);\n        let oyuncu_can = self.savas.savasci(self.oyuncu).map_or(0.0, |s| s.can.oran());\n        let muhafiz_can = self.savas.savasci(self.muhafiz).map_or(0.0, |s| s.can.oran());\n        arayuzu_guncelle(dunya, macera, &self.kimlikler, oyuncu_konumu, self.gunluk_acik, oyuncu_can, muhafiz_can, &self.bildirim);\n",
        1,
    )
    oyun = oyun.replace(
        "        if girdi.bu_kare_basildi_mi(Tus::Sekme) {\n            self.durum_yazdir(macera);\n        }\n",
        "        if girdi.bu_kare_basildi_mi(Tus::Sekme) {\n            self.gunluk_acik = !self.gunluk_acik;\n            self.durum_yazdir(macera);\n        }\n",
        1,
    )
    metod = '''    fn savasi_guncelle(&mut self, girdi: &Girdi, zaman: &Zaman, dunya: &mut Dunya, macera: &mut Macera, oyuncu_konumu: Vektor3) {
        self.savas.guncelle(zaman.kare_suresi());
        let muhafiz_canli = self.savas.savasci(self.muhafiz).is_some_and(|s| s.can.canli_mi());
        let muhafiz_konumu = dunya.varlik(self.muhafiz).map_or(Vektor3::SIFIR, |v| v.donusumu3b().konum);
        let mesafe = (muhafiz_konumu - oyuncu_konumu).uzunluk();
        if girdi.bu_kare_basildi_mi(Tus::F) && muhafiz_canli {
            if self.savas.saldir(self.oyuncu, self.muhafiz, mesafe) {
                self.bildirim = "Taş Muhafız'a saldırdın.".to_owned();
                self.sesler.ekle(SesOlayi::dunya("kilic-vurus", muhafiz_konumu, 0.9));
            } else {
                self.bildirim = "Saldırı için hedefe yaklaş.".to_owned();
            }
        }
        match self.muhafiz_ajani.karar(muhafiz_konumu, oyuncu_konumu, muhafiz_canli) {
            AjanKarari::Bekle => {}
            AjanKarari::TakipEt { yon, hiz } => if let Some(v) = dunya.varlik_mut(self.muhafiz) { v.donusumu3b_mut().tasi(yon * hiz * zaman.kare_saniyesi()); },
            AjanKarari::Saldir => if self.savas.saldir(self.muhafiz, self.oyuncu, mesafe) { self.bildirim = "Taş Muhafız sana vurdu!".to_owned(); self.sesler.ekle(SesOlayi::dunya("tas-vurus", oyuncu_konumu, 0.8)); },
        }
        for olay in self.savas.olaylari_al() {
            if let SavasOlayi::Yenildi { varlik, .. } = olay {
                if varlik == self.muhafiz {
                    if let Some(v) = dunya.varlik_mut(self.muhafiz) { v.etkinlestir(false); }
                    let _ = macera.olay_yayinla(OyunOlayi::DusmanYenildi { dusman: "Taş Muhafız".to_owned() });
                    self.bildirim = "Taş Muhafız yenildi. Kadim mühür çözüldü!".to_owned();
                    self.sesler.ekle(SesOlayi::ekran("muhafiz-yenildi", 1.0));
                } else if varlik == self.oyuncu {
                    self.bildirim = "Yenildin. R ile kontrol noktasına dön.".to_owned();
                }
            }
        }
        for ses in self.sesler.olaylari_al() {
            println!("Ses olayı: {} / şiddet {:.2}", ses.ses, ses.siddet);
        }
    }

'''
    oyun = oyun.replace("    fn kamera_acisini_guncelle", metod + "    fn kamera_acisini_guncelle", 1)
yaz("examples/ilk-oyun/src/oyun.rs", oyun)

ui = oku("examples/ilk-oyun/src/ekran_arayuzu.rs")
if "oyuncu_can: f32" not in ui:
    ui = ui.replace(
        "    gunluk_acik: bool,\n) {",
        "    gunluk_acik: bool,\n    oyuncu_can: f32,\n    muhafiz_can: f32,\n    bildirim: &str,\n) {",
        1,
    )
    ui = ui.replace(
        "    if gunluk_acik {\n",
        "    let can_alani = EkranDikdortgeni::yeni(18.0, 140.0, 280.0, 54.0);\n    arayuz.panel_ekle(ArayuzPaneli::yeni(can_alani, KARANLIK_PANEL));\n    arayuz.metin_ekle(EkranMetni::yeni(format!(\"CAN {:>3.0}%   MUHAFIZ {:>3.0}%\", oyuncu_can * 100.0, muhafiz_can * 100.0), EkranDikdortgeni::yeni(34.0, 155.0, 250.0, 28.0), 16.0));\n    if !bildirim.is_empty() { arayuz.metin_ekle(EkranMetni::yeni(bildirim, EkranDikdortgeni::yeni(320.0, 20.0, 600.0, 34.0), 16.0).renk(EkranRengi::SARI)); }\n\n    if gunluk_acik {\n",
        1,
    )
    ui = ui.replace(
        "WASD Hareket  •  E Etkileşim  •  Tab Günlük  •  F5/F9 Kayıt",
        "WASD Hareket  •  E Etkileşim  •  F Saldırı  •  Tab Günlük  •  F5/F9 Kayıt",
    )
yaz("examples/ilk-oyun/src/ekran_arayuzu.rs", ui)
