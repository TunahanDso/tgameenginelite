impl Macera {
    /// Boş bir macera çalışma zamanı oluşturur.
    #[must_use]
    pub fn yeni() -> Self {
        Self::default()
    }

    /// Tanımlı içerik sayılarını döndürür.
    #[must_use]
    pub fn rapor(&self) -> MaceraRaporu {
        MaceraRaporu {
            esya: self.esya_tanimlari.len(),
            gorev: self.gorev_tanimlari.len(),
            diyalog: self.diyalog_tanimlari.len(),
            sahne: self.sahne_tanimlari.len(),
            etkilesim: self.etkilesimler.len(),
            alan: self.alanlar.len(),
            kural: self.kurallar.len(),
        }
    }

    /// Hikâye kara tahtasını döndürür.
    #[must_use]
    pub const fn durum(&self) -> &OyunDurumu {
        &self.durum
    }

    /// Hikâye kara tahtasını değiştirilebilir döndürür.
    #[must_use]
    pub const fn durum_mut(&mut self) -> &mut OyunDurumu {
        &mut self.durum
    }

    /// Oyuncu envanterini döndürür.
    #[must_use]
    pub const fn envanter(&self) -> &Envanter {
        &self.envanter
    }

    /// Toplam oynama süresini döndürür.
    #[must_use]
    pub const fn oynama_suresi_milisaniye(&self) -> u64 {
        self.oynama_suresi_milisaniye
    }

    /// Kare süresini taşma denetimiyle toplam oynama süresine ekler.
    pub fn sure_ekle(&mut self, sure: Duration) {
        let milisaniye = u64::try_from(sure.as_millis()).unwrap_or(u64::MAX);
        self.oynama_suresi_milisaniye = self
            .oynama_suresi_milisaniye
            .saturating_add(milisaniye);
    }

    /// Son işlenen olayların sınırlı geçmişini döndürür.
    #[must_use]
    pub fn son_olaylar(&self) -> &VecDeque<OyunOlayi> {
        &self.son_olaylar
    }

    /// Eşya tanımı ekler veya aynı kimlikteki tanımı değiştirir.
    pub fn esya_tanimla(&mut self, tanim: EsyaTanimi) {
        self.esya_tanimlari.insert(tanim.kimlik.clone(), tanim);
    }

    /// Görev tanımı ekler veya aynı kimlikteki tanımı değiştirir.
    pub fn gorev_tanimla(&mut self, tanim: GorevTanimi) {
        let kimlik = tanim.kimlik.clone();
        self.gorev_tanimlari.insert(kimlik.clone(), tanim);
        self.gorevler.entry(kimlik).or_default();
    }

    /// Diyalog tanımı ekler veya aynı kimlikteki tanımı değiştirir.
    pub fn diyalog_tanimla(&mut self, tanim: DiyalogTanimi) {
        self.diyalog_tanimlari
            .insert(tanim.kimlik.clone(), tanim);
    }

    /// Sahne tanımı ekler veya aynı kimlikteki tanımı değiştirir.
    pub fn sahne_tanimla(&mut self, tanim: SahneTanimi) {
        self.sahne_tanimlari.insert(tanim.kimlik.clone(), tanim);
    }

    /// Etkileşim noktası ekler veya aynı kimlikteki noktayı değiştirir.
    pub fn etkilesim_tanimla(&mut self, tanim: EtkilesimNoktasi) {
        self.etkilesimler.insert(tanim.kimlik.clone(), tanim);
    }

    /// Alan tetikleyicisi ekler veya aynı kimlikteki alanı değiştirir.
    pub fn alan_tanimla(&mut self, tanim: AlanTetikleyicisi) {
        self.alanlar.insert(tanim.kimlik.clone(), tanim);
    }

    /// Olay kuralı ekler veya aynı kimlikteki kuralı değiştirir.
    pub fn kural_tanimla(&mut self, tanim: OlayKurali) {
        self.kurallar.insert(tanim.kimlik.clone(), tanim);
    }

    /// Koşul ağacının güncel macera durumunda sağlanıp sağlanmadığını döndürür.
    #[must_use]
    pub fn kosul_saglanir(&self, kosul: &Kosul) -> bool {
        match kosul {
            Kosul::HerZaman => true,
            Kosul::Bayrak { ad, deger } => self.durum.bayrak(ad) == *deger,
            Kosul::SayacEnAz { ad, deger } => self.durum.sayac(ad) >= *deger,
            Kosul::SayacEnFazla { ad, deger } => self.durum.sayac(ad) <= *deger,
            Kosul::MetinEsit { ad, deger } => self.durum.metin(ad) == Some(deger),
            Kosul::EsyaEnAz { esya, miktar } => self.envanter.var_mi(esya, *miktar),
            Kosul::GorevAsamasi { gorev, asama } => self.gorev_asamasi(gorev) == *asama,
            Kosul::Sahne(sahne) => self.etkin_sahne.as_ref() == Some(sahne),
            Kosul::Tumu(kosullar) => kosullar.iter().all(|alt| self.kosul_saglanir(alt)),
            Kosul::Herhangi(kosullar) => kosullar.iter().any(|alt| self.kosul_saglanir(alt)),
            Kosul::Degil(alt) => !self.kosul_saglanir(alt),
        }
    }

    fn kosullar_saglanir(&self, kosullar: &[Kosul]) -> bool {
        kosullar.iter().all(|kosul| self.kosul_saglanir(kosul))
    }

    /// Envantere tanımlı eşya ekler ve olay yayınlar.
    ///
    /// # Errors
    ///
    /// Eşya tanımlı değilse veya miktar yığın sınırını aşarsa [`OyunHatasi`] döndürür.
    pub fn esya_ekle(&mut self, kimlik: &EsyaKimligi, miktar: u32) -> OyunSonucu<u32> {
        let yeni = self.esya_ekle_ic(kimlik, miktar)?;
        self.olaylari_isle()?;
        Ok(yeni)
    }

    fn esya_ekle_ic(&mut self, kimlik: &EsyaKimligi, miktar: u32) -> OyunSonucu<u32> {
        let tanim = self.esya_tanimlari.get(kimlik).ok_or_else(|| {
            OyunHatasi::yeni(format!("Tanımlanmamış eşya envantere eklenemez: {}", kimlik.deger()))
        })?;
        let mevcut = self.envanter.miktar(kimlik);
        let yeni = mevcut
            .checked_add(miktar)
            .ok_or_else(|| OyunHatasi::yeni("Envanter eşya miktarı sayı sınırını aştı."))?;
        if yeni > tanim.yigin_siniri {
            return Err(OyunHatasi::yeni(format!(
                "'{}' eşyası {} yığın sınırını aşamaz.",
                tanim.ad, tanim.yigin_siniri
            )));
        }
        self.envanter.miktar_ayarla(kimlik.clone(), yeni);
        if miktar > 0 {
            self.olay_kuyrugu.push_back(OyunOlayi::EsyaEklendi {
                esya: kimlik.clone(),
                miktar,
            });
        }
        Ok(yeni)
    }

    /// Envanterden eşya çıkarır ve olay yayınlar.
    ///
    /// # Errors
    ///
    /// Eşya miktarı yetersizse veya korunan görev eşyası çıkarılmak istenirse
    /// [`OyunHatasi`] döndürür.
    pub fn esya_cikar(&mut self, kimlik: &EsyaKimligi, miktar: u32) -> OyunSonucu<u32> {
        let yeni = self.esya_cikar_ic(kimlik, miktar)?;
        self.olaylari_isle()?;
        Ok(yeni)
    }

    fn esya_cikar_ic(&mut self, kimlik: &EsyaKimligi, miktar: u32) -> OyunSonucu<u32> {
        let tanim = self.esya_tanimlari.get(kimlik).ok_or_else(|| {
            OyunHatasi::yeni(format!("Tanımlanmamış eşya envanterden çıkarılamaz: {}", kimlik.deger()))
        })?;
        if tanim.gorev_esyasi && miktar > 0 {
            return Err(OyunHatasi::yeni(format!(
                "'{}' korunan bir görev eşyasıdır.", tanim.ad
            )));
        }
        let mevcut = self.envanter.miktar(kimlik);
        let yeni = mevcut.checked_sub(miktar).ok_or_else(|| {
            OyunHatasi::yeni(format!("'{}' eşyası envanterde yeterli miktarda yok.", tanim.ad))
        })?;
        self.envanter.miktar_ayarla(kimlik.clone(), yeni);
        if miktar > 0 {
            self.olay_kuyrugu.push_back(OyunOlayi::EsyaCikarildi {
                esya: kimlik.clone(),
                miktar,
            });
        }
        Ok(yeni)
    }

    /// Görevin güncel aşamasını döndürür.
    #[must_use]
    pub fn gorev_asamasi(&self, kimlik: &GorevKimligi) -> GorevAsamasi {
        self.gorevler
            .get(kimlik)
            .map_or(GorevAsamasi::Kilitli, GorevIlerlemesi::asama)
    }

    /// Görevin çalışma zamanı ilerlemesini döndürür.
    #[must_use]
    pub fn gorev_ilerlemesi(&self, kimlik: &GorevKimligi) -> Option<&GorevIlerlemesi> {
        self.gorevler.get(kimlik)
    }

    /// Görevi ilk adımından etkinleştirir.
    ///
    /// # Errors
    ///
    /// Görev tanımlı değilse veya hiç adım içermiyorsa [`OyunHatasi`] döndürür.
    pub fn gorev_baslat(&mut self, kimlik: &GorevKimligi) -> OyunSonucu {
        self.gorev_baslat_ic(kimlik)?;
        self.olaylari_isle()
    }

    fn gorev_baslat_ic(&mut self, kimlik: &GorevKimligi) -> OyunSonucu {
        let tanim = self.gorev_tanimlari.get(kimlik).ok_or_else(|| {
            OyunHatasi::yeni(format!("Tanımlanmamış görev başlatılamaz: {}", kimlik.deger()))
        })?;
        let ilk_adim = tanim
            .adimlar
            .first()
            .ok_or_else(|| OyunHatasi::yeni("Görev en az bir adım içermelidir."))?;
        let ilerleme = self.gorevler.entry(kimlik.clone()).or_default();
        if ilerleme.asama != GorevAsamasi::Kilitli {
            return Ok(());
        }
        ilerleme.asama = GorevAsamasi::Etkin;
        ilerleme.adim = 0;
        ilerleme.hedefler = vec![0; ilk_adim.hedefler.len()];
        self.olay_kuyrugu
            .push_back(OyunOlayi::GorevBasladi { gorev: kimlik.clone() });
        Ok(())
    }

    /// Etkin görevi başarısız duruma geçirir.
    ///
    /// # Errors
    ///
    /// Görev tanımlı değilse [`OyunHatasi`] döndürür.
    pub fn gorev_basarisiz(&mut self, kimlik: &GorevKimligi) -> OyunSonucu {
        if !self.gorev_tanimlari.contains_key(kimlik) {
            return Err(OyunHatasi::yeni(format!(
                "Tanımlanmamış görev başarısız yapılamaz: {}",
                kimlik.deger()
            )));
        }
        let ilerleme = self.gorevler.entry(kimlik.clone()).or_default();
        if ilerleme.asama == GorevAsamasi::Etkin {
            ilerleme.asama = GorevAsamasi::Basarisiz;
        }
        Ok(())
    }

    /// Adlandırılmış oyun olayını görev ve kural sistemine yayınlar.
    ///
    /// # Errors
    ///
    /// Olayların ürettiği eylemler geçersizse veya olay zinciri güvenlik sınırını aşarsa
    /// [`OyunHatasi`] döndürür.
    pub fn olay_yayinla(&mut self, olay: OyunOlayi) -> OyunSonucu {
        self.olay_kuyrugu.push_back(olay);
        self.olaylari_isle()
    }

    fn olaylari_isle(&mut self) -> OyunSonucu {
        let mut islenen = 0;
        while let Some(olay) = self.olay_kuyrugu.pop_front() {
            islenen += 1;
            if islenen > AZAMI_ZINCIR_OLAYI {
                self.olay_kuyrugu.clear();
                return Err(OyunHatasi::yeni(
                    "Oyun olayı zinciri güvenlik sınırını aştı; döngüsel kural olabilir.",
                ));
            }
            self.olay_gecmisine_ekle(olay.clone());
            for yeni_olay in self.gorev_olayini_isle(&olay)? {
                self.olay_kuyrugu.push_back(yeni_olay);
            }
            let eylemler = self.kural_eylemlerini_topla(&olay);
            self.eylemleri_uygula_ic(&eylemler)?;
        }
        Ok(())
    }

    fn olay_gecmisine_ekle(&mut self, olay: OyunOlayi) {
        if self.son_olaylar.len() == SON_OLAY_KAPASITESI {
            self.son_olaylar.pop_front();
        }
        self.son_olaylar.push_back(olay);
    }

    fn gorev_olayini_isle(&mut self, olay: &OyunOlayi) -> OyunSonucu<Vec<OyunOlayi>> {
        let etkinler = self
            .gorevler
            .iter()
            .filter(|(_, ilerleme)| ilerleme.asama == GorevAsamasi::Etkin)
            .map(|(kimlik, _)| kimlik.clone())
            .collect::<Vec<_>>();
        let mut uretilenler = Vec::new();
        for kimlik in etkinler {
            uretilenler.extend(self.tek_gorev_olayini_isle(&kimlik, olay)?);
        }
        Ok(uretilenler)
    }

    fn tek_gorev_olayini_isle(
        &mut self,
        kimlik: &GorevKimligi,
        olay: &OyunOlayi,
    ) -> OyunSonucu<Vec<OyunOlayi>> {
        let tanim = self
            .gorev_tanimlari
            .get(kimlik)
            .ok_or_else(|| OyunHatasi::yeni("Etkin görevin tanımı bulunamadı."))?;
        let ilerleme = self
            .gorevler
            .get_mut(kimlik)
            .ok_or_else(|| OyunHatasi::yeni("Etkin görevin ilerlemesi bulunamadı."))?;
        let adim = tanim
            .adimlar
            .get(ilerleme.adim)
            .ok_or_else(|| OyunHatasi::yeni("Görev ilerlemesi bulunmayan bir adıma işaret ediyor."))?;
        if ilerleme.hedefler.len() != adim.hedefler.len() {
            ilerleme.hedefler.resize(adim.hedefler.len(), 0);
        }
        for (hedef, miktar) in adim.hedefler.iter().zip(&mut ilerleme.hedefler) {
            if hedef.filtre.eslesir(olay) {
                *miktar = miktar.saturating_add(olay.miktar()).min(hedef.gereken);
            }
        }
        if !hedefler_tamamlandi(adim, ilerleme) {
            return Ok(Vec::new());
        }
        let tamamlanan_adim = ilerleme.adim;
        let mut olaylar = vec![OyunOlayi::GorevAdimiTamamlandi {
            gorev: kimlik.clone(),
            adim: tamamlanan_adim,
        }];
        ilerleme.adim += 1;
        if let Some(yeni_adim) = tanim.adimlar.get(ilerleme.adim) {
            ilerleme.hedefler = vec![0; yeni_adim.hedefler.len()];
        } else {
            ilerleme.asama = GorevAsamasi::Tamamlandi;
            ilerleme.hedefler.clear();
            olaylar.push(OyunOlayi::GorevTamamlandi { gorev: kimlik.clone() });
        }
        Ok(olaylar)
    }

    fn kural_eylemlerini_topla(&mut self, olay: &OyunOlayi) -> Vec<Eylem> {
        let kurallar = self
            .kurallar
            .values()
            .filter(|kural| {
                (kural.tekrarlama == Tekrarlama::HerZaman
                    || !self.calismis_kurallar.contains(&kural.kimlik))
                    && kural.filtre.eslesir(olay)
                    && self.kosullar_saglanir(&kural.kosullar)
            })
            .cloned()
            .collect::<Vec<_>>();
        let mut eylemler = Vec::new();
        for kural in kurallar {
            if kural.tekrarlama == Tekrarlama::BirKez {
                self.calismis_kurallar.insert(kural.kimlik);
            }
            eylemler.extend(kural.eylemler);
        }
        eylemler
    }
}
