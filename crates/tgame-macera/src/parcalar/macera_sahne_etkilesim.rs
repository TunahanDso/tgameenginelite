impl Macera {
    /// İlk etkin sahneyi geçiş oluşturmadan ayarlar.
    ///
    /// # Errors
    ///
    /// Sahne tanımlı değilse [`OyunHatasi`] döndürür.
    pub fn baslangic_sahnesi_ayarla(&mut self, sahne: &SahneKimligi) -> OyunSonucu {
        self.sahne_var_mi(sahne)?;
        self.etkin_sahne = Some(sahne.clone());
        Ok(())
    }

    /// Güncel etkin sahne kimliğini döndürür.
    #[must_use]
    pub const fn etkin_sahne(&self) -> Option<&SahneKimligi> {
        self.etkin_sahne.as_ref()
    }

    /// Sahne geçişini bir sonraki güvenli oyun güncellemesi için sıraya alır.
    ///
    /// # Errors
    ///
    /// Hedef sahne veya giriş noktası tanımlı değilse [`OyunHatasi`] döndürür.
    pub fn sahne_gecisi_iste(
        &mut self,
        sahne: &SahneKimligi,
        giris_noktasi: Option<String>,
    ) -> OyunSonucu {
        let tanim = self.sahne_var_mi(sahne)?;
        if let Some(giris) = &giris_noktasi {
            if tanim.giris_konumu(giris).is_none() {
                return Err(OyunHatasi::yeni(format!(
                    "'{}' sahnesinde '{}' giriş noktası bulunamadı.",
                    tanim.ad, giris
                )));
            }
        }
        self.bekleyen_gecis = Some(SahneGecisi {
            onceki: self.etkin_sahne.clone(),
            hedef: sahne.clone(),
            giris_noktasi,
        });
        Ok(())
    }

    fn sahne_var_mi(&self, sahne: &SahneKimligi) -> OyunSonucu<&SahneTanimi> {
        self.sahne_tanimlari.get(sahne).ok_or_else(|| {
            OyunHatasi::yeni(format!("Tanımlanmamış sahne kullanılamaz: {}", sahne.deger()))
        })
    }

    /// Bekleyen geçişi uygular ve oyun kodunun dünyayı yeniden kurması için döndürür.
    ///
    /// # Errors
    ///
    /// Bekleyen geçişin hedef sahnesi artık tanımlı değilse [`OyunHatasi`] döndürür.
    pub fn sahne_gecisini_al(&mut self) -> OyunSonucu<Option<SahneGecisi>> {
        let Some(gecis) = self.bekleyen_gecis.take() else {
            return Ok(None);
        };
        self.sahne_var_mi(&gecis.hedef)?;
        self.etkin_sahne = Some(gecis.hedef.clone());
        self.olay_kuyrugu.push_back(OyunOlayi::SahneDegisti {
            onceki: gecis.onceki.clone(),
            yeni: gecis.hedef.clone(),
        });
        self.olaylari_isle()?;
        Ok(Some(gecis))
    }

    /// Etkin sahnedeki giriş noktasının dünya konumunu döndürür.
    #[must_use]
    pub fn etkin_giris_konumu(&self, ad: &str) -> Option<Vektor3> {
        self.etkin_sahne
            .as_ref()
            .and_then(|kimlik| self.sahne_tanimlari.get(kimlik))
            .and_then(|sahne| sahne.giris_konumu(ad))
    }

    /// Oyuncu konumuna en yakın kullanılabilir etkileşimi döndürür.
    #[must_use]
    pub fn yakin_etkilesim(&self, oyuncu_konumu: Vektor3) -> Option<EtkilesimGorunumu> {
        self.etkilesimler
            .values()
            .filter(|etkilesim| self.etkilesim_kullanilabilir(etkilesim, oyuncu_konumu))
            .min_by(|sol, sag| {
                uzaklik_karesi(sol.konum, oyuncu_konumu)
                    .total_cmp(&uzaklik_karesi(sag.konum, oyuncu_konumu))
            })
            .map(|etkilesim| EtkilesimGorunumu {
                kimlik: etkilesim.kimlik.clone(),
                ileti: etkilesim.ileti.clone(),
            })
    }

    fn etkilesim_kullanilabilir(
        &self,
        etkilesim: &EtkilesimNoktasi,
        oyuncu_konumu: Vektor3,
    ) -> bool {
        (etkilesim.tekrarlama == Tekrarlama::HerZaman
            || !self.tuketilen_etkilesimler.contains(&etkilesim.kimlik))
            && self.kosullar_saglanir(&etkilesim.kosullar)
            && uzaklik_karesi(etkilesim.konum, oyuncu_konumu)
                <= etkilesim.yaricap * etkilesim.yaricap
    }

    /// Etkileşim noktasının eylemlerini uygular.
    ///
    /// # Errors
    ///
    /// Etkileşim bulunamazsa, menzil/koşul sağlanmazsa veya eylem geçersizse
    /// [`OyunHatasi`] döndürür.
    pub fn etkiles(
        &mut self,
        kimlik: &EtkilesimKimligi,
        oyuncu_konumu: Vektor3,
    ) -> OyunSonucu {
        let etkilesim = self
            .etkilesimler
            .get(kimlik)
            .cloned()
            .ok_or_else(|| OyunHatasi::yeni("Etkileşim noktası bulunamadı."))?;
        if !self.etkilesim_kullanilabilir(&etkilesim, oyuncu_konumu) {
            return Err(OyunHatasi::yeni("Etkileşim menzil veya hikâye koşullarını sağlamıyor."));
        }
        if etkilesim.tekrarlama == Tekrarlama::BirKez {
            self.tuketilen_etkilesimler.insert(kimlik.clone());
        }
        self.eylemleri_uygula_ic(&etkilesim.eylemler)?;
        self.olay_kuyrugu
            .push_back(OyunOlayi::Etkilesim { etkilesim: kimlik.clone() });
        self.olaylari_isle()
    }
}
