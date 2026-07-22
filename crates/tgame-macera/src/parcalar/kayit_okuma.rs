fn kaydi_oku(icerik: &str, macera: &mut Macera) -> OyunSonucu {
    let onceki_sahne = macera.etkin_sahne.clone();
    let mut satirlar = icerik.lines();
    let baslik = satirlar
        .next()
        .ok_or_else(|| OyunHatasi::yeni("Kayıt dosyası boş."))?;
    let beklenen = format!("{KAYIT_IMZASI}\t{KAYIT_SURUMU}");
    if baslik != beklenen {
        return Err(OyunHatasi::yeni("Kayıt dosyası imzası veya sürümü desteklenmiyor."));
    }
    macera.calisma_durumunu_sifirla();
    let mut son_goruldu = false;
    for satir in satirlar {
        if satir == "SON" {
            son_goruldu = true;
            break;
        }
        kayit_satirini_uygula(satir, macera)?;
    }
    if !son_goruldu {
        return Err(OyunHatasi::yeni("Kayıt dosyası tamamlanma işareti içermiyor."));
    }
    if let Some(hedef) = macera.etkin_sahne.clone() {
        let giris_noktasi = macera
            .kontrol_noktasi
            .as_ref()
            .filter(|kontrol| kontrol.sahne == hedef)
            .and_then(|kontrol| kontrol.giris_noktasi.clone());
        macera.bekleyen_gecis = Some(SahneGecisi {
            onceki: onceki_sahne,
            hedef,
            giris_noktasi,
        });
    }
    Ok(())
}

fn kayit_satirini_uygula(satir: &str, macera: &mut Macera) -> OyunSonucu {
    let alanlar = satir.split('\t').collect::<Vec<_>>();
    let etiket = alanlar
        .first()
        .copied()
        .ok_or_else(|| OyunHatasi::yeni("Boş kayıt satırı bulundu."))?;
    match etiket {
        "SURE" => macera.oynama_suresi_milisaniye = sayi_oku(&alanlar, 1, "oynama süresi")?,
        "SAHNE" => macera.etkin_sahne = Some(SahneKimligi::yeni(metin_al(&alanlar, 1)?)),
        "BAYRAK" => {
            let ad = metin_al(&alanlar, 1)?;
            macera.durum.bayrak_ayarla(ad, alan_al(&alanlar, 2)? == "1");
        }
        "SAYAC" => {
            let ad = metin_al(&alanlar, 1)?;
            let deger = alan_al(&alanlar, 2)?.parse::<i64>().map_err(|hata| {
                OyunHatasi::yeni(format!("Kayıt sayacı çözülemedi: {hata}"))
            })?;
            macera.durum.sayac_ayarla(ad, deger);
        }
        "METIN" => macera
            .durum
            .metin_ayarla(metin_al(&alanlar, 1)?, metin_al(&alanlar, 2)?),
        "ESYA" => {
            let kimlik = EsyaKimligi::yeni(metin_al(&alanlar, 1)?);
            let miktar = u32_oku(&alanlar, 2, "eşya miktarı")?;
            macera.envanter.miktar_ayarla(kimlik, miktar);
        }
        "GOREV" => gorev_satirini_uygula(&alanlar, macera)?,
        "ETKILESIM" => {
            macera
                .tuketilen_etkilesimler
                .insert(EtkilesimKimligi::yeni(metin_al(&alanlar, 1)?));
        }
        "ALAN" => {
            macera
                .tuketilen_alanlar
                .insert(AlanKimligi::yeni(metin_al(&alanlar, 1)?));
        }
        "KURAL" => {
            macera
                .calismis_kurallar
                .insert(KuralKimligi::yeni(metin_al(&alanlar, 1)?));
        }
        "DIYALOG" => {
            macera.etkin_diyalog = Some(EtkinDiyalog {
                diyalog: DiyalogKimligi::yeni(metin_al(&alanlar, 1)?),
                dugum: metin_al(&alanlar, 2)?,
            });
        }
        "KONTROL" => kontrol_satirini_uygula(&alanlar, macera)?,
        _ => return Err(OyunHatasi::yeni(format!("Bilinmeyen kayıt etiketi: {etiket}"))),
    }
    Ok(())
}

fn gorev_satirini_uygula(alanlar: &[&str], macera: &mut Macera) -> OyunSonucu {
    let kimlik = GorevKimligi::yeni(metin_al(alanlar, 1)?);
    let asama = gorev_asamasi_oku(alan_al(alanlar, 2)?)?;
    let adim = usize_oku(alanlar, 3, "görev adımı")?;
    let hedefler = alan_al(alanlar, 4)?;
    let hedefler = if hedefler.is_empty() {
        Vec::new()
    } else {
        hedefler
            .split(',')
            .map(|deger| {
                deger.parse::<u32>().map_err(|hata| {
                    OyunHatasi::yeni(format!("Görev hedef ilerlemesi çözülemedi: {hata}"))
                })
            })
            .collect::<OyunSonucu<Vec<_>>>()?
    };
    macera.gorevler.insert(
        kimlik,
        GorevIlerlemesi {
            asama,
            adim,
            hedefler,
        },
    );
    Ok(())
}

fn kontrol_satirini_uygula(alanlar: &[&str], macera: &mut Macera) -> OyunSonucu {
    let giris = alan_al(alanlar, 3)?;
    macera.kontrol_noktasi = Some(KontrolNoktasi {
        kimlik: KontrolNoktasiKimligi::yeni(metin_al(alanlar, 1)?),
        sahne: SahneKimligi::yeni(metin_al(alanlar, 2)?),
        giris_noktasi: if giris.is_empty() {
            None
        } else {
            Some(metni_coz(giris)?)
        },
    });
    Ok(())
}

fn alan_al<'a>(alanlar: &'a [&str], indeks: usize) -> OyunSonucu<&'a str> {
    alanlar
        .get(indeks)
        .copied()
        .ok_or_else(|| OyunHatasi::yeni("Kayıt satırında zorunlu alan eksik."))
}

fn metin_al(alanlar: &[&str], indeks: usize) -> OyunSonucu<String> {
    metni_coz(alan_al(alanlar, indeks)?)
}

fn sayi_oku(alanlar: &[&str], indeks: usize, ad: &str) -> OyunSonucu<u64> {
    alan_al(alanlar, indeks)?.parse::<u64>().map_err(|hata| {
        OyunHatasi::yeni(format!("Kayıt {ad} değeri çözülemedi: {hata}"))
    })
}

fn u32_oku(alanlar: &[&str], indeks: usize, ad: &str) -> OyunSonucu<u32> {
    alan_al(alanlar, indeks)?.parse::<u32>().map_err(|hata| {
        OyunHatasi::yeni(format!("Kayıt {ad} değeri çözülemedi: {hata}"))
    })
}

fn usize_oku(alanlar: &[&str], indeks: usize, ad: &str) -> OyunSonucu<usize> {
    alan_al(alanlar, indeks)?.parse::<usize>().map_err(|hata| {
        OyunHatasi::yeni(format!("Kayıt {ad} değeri çözülemedi: {hata}"))
    })
}

const fn gorev_asamasi_kodu(asama: GorevAsamasi) -> &'static str {
    match asama {
        GorevAsamasi::Kilitli => "K",
        GorevAsamasi::Etkin => "E",
        GorevAsamasi::Tamamlandi => "T",
        GorevAsamasi::Basarisiz => "B",
    }
}

fn gorev_asamasi_oku(kod: &str) -> OyunSonucu<GorevAsamasi> {
    match kod {
        "K" => Ok(GorevAsamasi::Kilitli),
        "E" => Ok(GorevAsamasi::Etkin),
        "T" => Ok(GorevAsamasi::Tamamlandi),
        "B" => Ok(GorevAsamasi::Basarisiz),
        _ => Err(OyunHatasi::yeni("Kayıt dosyasında bilinmeyen görev aşaması var.")),
    }
}

fn metni_kodla(metin: &str) -> String {
    const ONALTI: &[u8; 16] = b"0123456789abcdef";
    let mut sonuc = String::with_capacity(metin.len() * 2);
    for bayt in metin.as_bytes() {
        sonuc.push(char::from(ONALTI[usize::from(bayt >> 4)]));
        sonuc.push(char::from(ONALTI[usize::from(bayt & 0x0f)]));
    }
    sonuc
}

fn metni_coz(kod: &str) -> OyunSonucu<String> {
    if !kod.len().is_multiple_of(2) {
        return Err(OyunHatasi::yeni("Kayıt metni geçersiz onaltılık uzunluk içeriyor."));
    }
    let baytlar = kod
        .as_bytes()
        .chunks_exact(2)
        .map(|cift| {
            let ust = onaltilik_deger(cift[0])?;
            let alt = onaltilik_deger(cift[1])?;
            Ok((ust << 4) | alt)
        })
        .collect::<OyunSonucu<Vec<_>>>()?;
    String::from_utf8(baytlar)
        .map_err(|hata| OyunHatasi::yeni(format!("Kayıt metni UTF-8 değil: {hata}")))
}

fn onaltilik_deger(bayt: u8) -> OyunSonucu<u8> {
    match bayt {
        b'0'..=b'9' => Ok(bayt - b'0'),
        b'a'..=b'f' => Ok(bayt - b'a' + 10),
        b'A'..=b'F' => Ok(bayt - b'A' + 10),
        _ => Err(OyunHatasi::yeni("Kayıt metni geçersiz onaltılık karakter içeriyor.")),
    }
}
