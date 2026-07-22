impl Macera {
    /// Oyuncunun konumuna göre alan giriş ve çıkışlarını günceller.
    ///
    /// # Errors
    ///
    /// Alan eylemlerinden biri geçersizse [`OyunHatasi`] döndürür.
    pub fn alanlari_guncelle(&mut self, oyuncu_konumu: Vektor3) -> OyunSonucu {
        let alanlar = self.alanlar.values().cloned().collect::<Vec<_>>();
        for tetikleyici in alanlar {
            self.tek_alani_guncelle(&tetikleyici, oyuncu_konumu)?;
        }
        self.olaylari_isle()
    }

    fn tek_alani_guncelle(
        &mut self,
        tetikleyici: &AlanTetikleyicisi,
        oyuncu_konumu: Vektor3,
    ) -> OyunSonucu {
        let iceride = tetikleyici.alan.icerir(oyuncu_konumu);
        let once_icerideydi = self.etkin_alanlar.contains(&tetikleyici.kimlik);
        let tuketildi = self.tuketilen_alanlar.contains(&tetikleyici.kimlik);
        if iceride && !once_icerideydi && !tuketildi {
            self.etkin_alanlar.insert(tetikleyici.kimlik.clone());
            if self.kosullar_saglanir(&tetikleyici.kosullar) {
                self.eylemleri_uygula_ic(&tetikleyici.giris_eylemleri)?;
                self.olay_kuyrugu.push_back(OyunOlayi::AlanaGirdi {
                    alan: tetikleyici.kimlik.clone(),
                });
            }
        } else if !iceride && once_icerideydi {
            self.etkin_alanlar.remove(&tetikleyici.kimlik);
            self.eylemleri_uygula_ic(&tetikleyici.cikis_eylemleri)?;
            self.olay_kuyrugu.push_back(OyunOlayi::AlandanCikti {
                alan: tetikleyici.kimlik.clone(),
            });
            if tetikleyici.tekrarlama == Tekrarlama::BirKez {
                self.tuketilen_alanlar.insert(tetikleyici.kimlik.clone());
            }
        }
        Ok(())
    }

    /// Etkin kontrol noktasını değiştirir ve olay yayınlar.
    ///
    /// # Errors
    ///
    /// Kontrol noktası olayının ürettiği eylemler geçersizse [`OyunHatasi`] döndürür.
    pub fn kontrol_noktasi_ayarla(&mut self, kontrol_noktasi: KontrolNoktasi) -> OyunSonucu {
        self.kontrol_noktasi_ayarla_ic(kontrol_noktasi);
        self.olaylari_isle()
    }

    fn kontrol_noktasi_ayarla_ic(&mut self, kontrol_noktasi: KontrolNoktasi) {
        self.olay_kuyrugu.push_back(OyunOlayi::KontrolNoktasi {
            kontrol_noktasi: kontrol_noktasi.kimlik.clone(),
        });
        self.kontrol_noktasi = Some(kontrol_noktasi);
    }

    /// Etkin kontrol noktasını döndürür.
    #[must_use]
    pub const fn kontrol_noktasi(&self) -> Option<&KontrolNoktasi> {
        self.kontrol_noktasi.as_ref()
    }

    /// Etkin kontrol noktasına sahne geçişi ister.
    ///
    /// # Errors
    ///
    /// Kontrol noktası yoksa veya hedef sahne geçersizse [`OyunHatasi`] döndürür.
    pub fn kontrol_noktasina_don(&mut self) -> OyunSonucu {
        let kontrol = self
            .kontrol_noktasi
            .clone()
            .ok_or_else(|| OyunHatasi::yeni("Etkin kontrol noktası bulunmuyor."))?;
        self.sahne_gecisi_iste(&kontrol.sahne, kontrol.giris_noktasi)
    }

    fn calisma_durumunu_sifirla(&mut self) {
        self.durum.temizle();
        self.envanter = Envanter::default();
        self.gorevler = self
            .gorev_tanimlari
            .keys()
            .cloned()
            .map(|kimlik| (kimlik, GorevIlerlemesi::default()))
            .collect();
        self.etkin_diyalog = None;
        self.etkin_sahne = None;
        self.bekleyen_gecis = None;
        self.tuketilen_etkilesimler.clear();
        self.etkin_alanlar.clear();
        self.tuketilen_alanlar.clear();
        self.calismis_kurallar.clear();
        self.olay_kuyrugu.clear();
        self.son_olaylar.clear();
        self.oynama_suresi_milisaniye = 0;
        self.kontrol_noktasi = None;
    }
}

fn hedefler_tamamlandi(adim: &GorevAdimi, ilerleme: &GorevIlerlemesi) -> bool {
    !adim.hedefler.is_empty()
        && adim
            .hedefler
            .iter()
            .zip(&ilerleme.hedefler)
            .all(|(hedef, miktar)| *miktar >= hedef.gereken)
}

fn uzaklik_karesi(sol: Vektor3, sag: Vektor3) -> f32 {
    (sol - sag).uzunluk_karesi()
}
