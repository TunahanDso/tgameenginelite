fn gecici_yol(yol: &Path) -> PathBuf {
    yol.with_extension("tgm.tmp")
}

fn yedek_yol(yol: &Path) -> PathBuf {
    yol.with_extension("tgm.bak")
}

fn kaydi_degistir(gecici: &Path, hedef: &Path) -> OyunSonucu {
    if !hedef.exists() {
        fs::rename(gecici, hedef)
            .map_err(|hata| OyunHatasi::yeni(format!("Kayıt dosyası yerine taşınamadı: {hata}")))?;
        return Ok(());
    }
    let yedek = yedek_yol(hedef);
    if yedek.exists() {
        fs::remove_file(&yedek)
            .map_err(|hata| OyunHatasi::yeni(format!("Eski kayıt yedeği silinemedi: {hata}")))?;
    }
    fs::rename(hedef, &yedek)
        .map_err(|hata| OyunHatasi::yeni(format!("Mevcut kayıt yedeklenemedi: {hata}")))?;
    if let Err(hata) = fs::rename(gecici, hedef) {
        drop(fs::rename(&yedek, hedef));
        return Err(OyunHatasi::yeni(format!("Yeni kayıt etkinleştirilemedi: {hata}")));
    }
    drop(fs::remove_file(yedek));
    Ok(())
}

fn kaydi_yaz(macera: &Macera) -> String {
    let mut satirlar = vec![format!("{KAYIT_IMZASI}\t{KAYIT_SURUMU}")];
    satirlar.push(format!("SURE\t{}", macera.oynama_suresi_milisaniye));
    if let Some(sahne) = &macera.etkin_sahne {
        satirlar.push(format!("SAHNE\t{}", metni_kodla(sahne.deger())));
    }
    for (ad, deger) in &macera.durum.bayraklar {
        satirlar.push(format!("BAYRAK\t{}\t{}", metni_kodla(ad), u8::from(*deger)));
    }
    for (ad, deger) in &macera.durum.sayaclar {
        satirlar.push(format!("SAYAC\t{}\t{deger}", metni_kodla(ad)));
    }
    for (ad, deger) in &macera.durum.metinler {
        satirlar.push(format!("METIN\t{}\t{}", metni_kodla(ad), metni_kodla(deger)));
    }
    for (esya, miktar) in &macera.envanter.miktarlar {
        satirlar.push(format!("ESYA\t{}\t{miktar}", metni_kodla(esya.deger())));
    }
    for (gorev, ilerleme) in &macera.gorevler {
        let hedefler = ilerleme
            .hedefler
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(",");
        satirlar.push(format!(
            "GOREV\t{}\t{}\t{}\t{hedefler}",
            metni_kodla(gorev.deger()),
            gorev_asamasi_kodu(ilerleme.asama),
            ilerleme.adim
        ));
    }
    kimlik_kumesini_yaz(&mut satirlar, "ETKILESIM", &macera.tuketilen_etkilesimler);
    kimlik_kumesini_yaz(&mut satirlar, "ALAN", &macera.tuketilen_alanlar);
    kimlik_kumesini_yaz(&mut satirlar, "KURAL", &macera.calismis_kurallar);
    if let Some(etkin) = &macera.etkin_diyalog {
        satirlar.push(format!(
            "DIYALOG\t{}\t{}",
            metni_kodla(etkin.diyalog.deger()),
            metni_kodla(&etkin.dugum)
        ));
    }
    if let Some(kontrol) = &macera.kontrol_noktasi {
        satirlar.push(format!(
            "KONTROL\t{}\t{}\t{}",
            metni_kodla(kontrol.kimlik.deger()),
            metni_kodla(kontrol.sahne.deger()),
            kontrol
                .giris_noktasi
                .as_deref()
                .map_or_else(String::new, metni_kodla)
        ));
    }
    satirlar.push("SON".to_owned());
    satirlar.join("\n")
}

fn kimlik_kumesini_yaz<T>(satirlar: &mut Vec<String>, etiket: &str, kume: &BTreeSet<T>)
where
    T: KimlikDegeri,
{
    satirlar.extend(
        kume.iter()
            .map(|kimlik| format!("{etiket}\t{}", metni_kodla(kimlik.kimlik_degeri()))),
    );
}

trait KimlikDegeri {
    fn kimlik_degeri(&self) -> &str;
}

macro_rules! kimlik_degeri_uygula {
    ($($ad:ty),+ $(,)?) => {
        $(
            impl KimlikDegeri for $ad {
                fn kimlik_degeri(&self) -> &str {
                    self.deger()
                }
            }
        )+
    };
}

kimlik_degeri_uygula!(EtkilesimKimligi, AlanKimligi, KuralKimligi);
