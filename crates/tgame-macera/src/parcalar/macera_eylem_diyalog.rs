impl Macera {
    /// Bir eylem listesini sırayla uygular ve üretilen olayları işler.
    ///
    /// # Errors
    ///
    /// Eylemlerden biri geçersiz içerik kimliği veya durum kullanırsa [`OyunHatasi`] döndürür.
    pub fn eylemleri_uygula(&mut self, eylemler: &[Eylem]) -> OyunSonucu {
        self.eylemleri_uygula_ic(eylemler)?;
        self.olaylari_isle()
    }

    fn eylemleri_uygula_ic(&mut self, eylemler: &[Eylem]) -> OyunSonucu {
        for eylem in eylemler {
            self.eylemi_uygula(eylem)?;
        }
        Ok(())
    }

    fn eylemi_uygula(&mut self, eylem: &Eylem) -> OyunSonucu {
        match eylem {
            Eylem::BayrakAyarla { ad, deger } => {
                self.durum.bayrak_ayarla(ad.clone(), *deger);
                Ok(())
            }
            Eylem::SayacDegistir { ad, fark } => {
                self.durum.sayac_degistir(ad.clone(), *fark)?;
                Ok(())
            }
            Eylem::MetinAyarla { ad, deger } => {
                self.durum.metin_ayarla(ad.clone(), deger.clone());
                Ok(())
            }
            Eylem::EsyaEkle { esya, miktar } => {
                self.esya_ekle_ic(esya, *miktar)?;
                Ok(())
            }
            Eylem::EsyaCikar { esya, miktar } => {
                self.esya_cikar_ic(esya, *miktar)?;
                Ok(())
            }
            Eylem::GorevBaslat(gorev) => self.gorev_baslat_ic(gorev),
            Eylem::GorevBasarisiz(gorev) => self.gorev_basarisiz(gorev),
            Eylem::DiyalogBaslat(diyalog) => self.diyalog_baslat_ic(diyalog),
            Eylem::SahneGecisiIste {
                sahne,
                giris_noktasi,
            } => self.sahne_gecisi_iste(sahne, giris_noktasi.clone()),
            Eylem::KontrolNoktasiAyarla {
                kimlik,
                sahne,
                giris_noktasi,
            } => {
                self.kontrol_noktasi_ayarla_ic(KontrolNoktasi {
                    kimlik: kimlik.clone(),
                    sahne: sahne.clone(),
                    giris_noktasi: giris_noktasi.clone(),
                });
                Ok(())
            }
            Eylem::OlayYayinla(olay) => {
                self.olay_kuyrugu.push_back((**olay).clone());
                Ok(())
            }
        }
    }

    /// Bir diyalog ağacını başlangıç düğümünden açar.
    ///
    /// # Errors
    ///
    /// Diyalog veya başlangıç düğümü tanımlı değilse [`OyunHatasi`] döndürür.
    pub fn diyalog_baslat(&mut self, kimlik: &DiyalogKimligi) -> OyunSonucu {
        self.diyalog_baslat_ic(kimlik)?;
        self.olaylari_isle()
    }

    fn diyalog_baslat_ic(&mut self, kimlik: &DiyalogKimligi) -> OyunSonucu {
        let tanim = self.diyalog_tanimlari.get(kimlik).ok_or_else(|| {
            OyunHatasi::yeni(format!("Tanımlanmamış diyalog başlatılamaz: {}", kimlik.deger()))
        })?;
        let baslangic = tanim.baslangic.clone();
        if !tanim.dugumler.contains_key(&baslangic) {
            return Err(OyunHatasi::yeni("Diyalog başlangıç düğümü bulunamadı."));
        }
        self.etkin_diyalog = Some(EtkinDiyalog {
            diyalog: kimlik.clone(),
            dugum: baslangic,
        });
        self.olay_kuyrugu.push_back(OyunOlayi::DiyalogBasladi {
            diyalog: kimlik.clone(),
        });
        self.etkin_dugum_eylemlerini_uygula()?;
        Ok(())
    }

    /// Etkin diyalog satırını ve kullanılabilir seçimleri döndürür.
    ///
    /// # Errors
    ///
    /// Etkin diyalog tanımı veya düğümü bulunamazsa [`OyunHatasi`] döndürür.
    pub fn diyalog_gorunumu(&self) -> OyunSonucu<Option<DiyalogGorunumu>> {
        let Some(etkin) = &self.etkin_diyalog else {
            return Ok(None);
        };
        let tanim = self
            .diyalog_tanimlari
            .get(&etkin.diyalog)
            .ok_or_else(|| OyunHatasi::yeni("Etkin diyalog tanımı bulunamadı."))?;
        let dugum = tanim
            .dugumler
            .get(&etkin.dugum)
            .ok_or_else(|| OyunHatasi::yeni("Etkin diyalog düğümü bulunamadı."))?;
        let secenekler = dugum
            .secenekler
            .iter()
            .filter(|secenek| self.kosullar_saglanir(&secenek.kosullar))
            .map(|secenek| DiyalogSecenegiGorunumu {
                kimlik: secenek.kimlik.clone(),
                metin: secenek.metin.clone(),
            })
            .collect();
        Ok(Some(DiyalogGorunumu {
            diyalog: etkin.diyalog.clone(),
            dugum: etkin.dugum.clone(),
            konusan: dugum.konusan.clone(),
            metin: dugum.metin.clone(),
            secenekler,
            ilerletilebilir: dugum.secenekler.is_empty(),
        }))
    }

    /// Seçeneksiz etkin diyalog satırını sonraki düğüme ilerletir.
    ///
    /// # Errors
    ///
    /// Diyalog etkin değilse, düğüm bulunamazsa veya düğüm seçim gerektiriyorsa
    /// [`OyunHatasi`] döndürür.
    pub fn diyalog_ilerlet(&mut self) -> OyunSonucu {
        let etkin = self
            .etkin_diyalog
            .clone()
            .ok_or_else(|| OyunHatasi::yeni("İlerletilecek etkin diyalog yok."))?;
        let dugum = self.diyalog_dugumu(&etkin)?.clone();
        if !dugum.secenekler.is_empty() {
            return Err(OyunHatasi::yeni("Bu diyalog düğümü bir seçim gerektiriyor."));
        }
        self.diyalog_sonrakiye_gec(etkin.diyalog, dugum.sonraki)?;
        self.olaylari_isle()
    }

    /// Etkin diyalogdaki kullanılabilir seçeneği uygular.
    ///
    /// # Errors
    ///
    /// Diyalog etkin değilse, seçenek bulunamazsa veya koşulları sağlanmıyorsa
    /// [`OyunHatasi`] döndürür.
    pub fn diyalog_sec(&mut self, secenek_kimligi: &str) -> OyunSonucu {
        let etkin = self
            .etkin_diyalog
            .clone()
            .ok_or_else(|| OyunHatasi::yeni("Seçim yapılacak etkin diyalog yok."))?;
        let dugum = self.diyalog_dugumu(&etkin)?;
        let secenek = dugum
            .secenekler
            .iter()
            .find(|secenek| secenek.kimlik == secenek_kimligi)
            .cloned()
            .ok_or_else(|| OyunHatasi::yeni("Diyalog seçeneği bulunamadı."))?;
        if !self.kosullar_saglanir(&secenek.kosullar) {
            return Err(OyunHatasi::yeni("Diyalog seçeneğinin koşulları sağlanmıyor."));
        }
        self.eylemleri_uygula_ic(&secenek.eylemler)?;
        self.olay_kuyrugu.push_back(OyunOlayi::DiyalogSecildi {
            diyalog: etkin.diyalog.clone(),
            secenek: secenek.kimlik,
        });
        self.diyalog_sonrakiye_gec(etkin.diyalog, secenek.sonraki)?;
        self.olaylari_isle()
    }

    fn diyalog_dugumu(&self, etkin: &EtkinDiyalog) -> OyunSonucu<&DiyalogDugumu> {
        self.diyalog_tanimlari
            .get(&etkin.diyalog)
            .and_then(|tanim| tanim.dugumler.get(&etkin.dugum))
            .ok_or_else(|| OyunHatasi::yeni("Etkin diyalog düğümü bulunamadı."))
    }

    fn diyalog_sonrakiye_gec(
        &mut self,
        diyalog: DiyalogKimligi,
        sonraki: Option<String>,
    ) -> OyunSonucu {
        if let Some(dugum) = sonraki {
            let tanim = self
                .diyalog_tanimlari
                .get(&diyalog)
                .ok_or_else(|| OyunHatasi::yeni("Diyalog tanımı bulunamadı."))?;
            if !tanim.dugumler.contains_key(&dugum) {
                return Err(OyunHatasi::yeni("Sonraki diyalog düğümü bulunamadı."));
            }
            self.etkin_diyalog = Some(EtkinDiyalog { diyalog, dugum });
            self.etkin_dugum_eylemlerini_uygula()?;
        } else {
            self.etkin_diyalog = None;
            self.olay_kuyrugu
                .push_back(OyunOlayi::DiyalogBitti { diyalog });
        }
        Ok(())
    }

    fn etkin_dugum_eylemlerini_uygula(&mut self) -> OyunSonucu {
        let etkin = self
            .etkin_diyalog
            .clone()
            .ok_or_else(|| OyunHatasi::yeni("Etkin diyalog bulunamadı."))?;
        let eylemler = self.diyalog_dugumu(&etkin)?.giris_eylemleri.clone();
        self.eylemleri_uygula_ic(&eylemler)
    }
}
