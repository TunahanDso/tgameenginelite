/// Sürümlemeli ve yuvalı macera kayıt dosyalarını yönetir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KayitYoneticisi {
    klasor: PathBuf,
    yuva_sayisi: u32,
}

impl KayitYoneticisi {
    /// Kayıt klasörü ve yuva sayısıyla yönetici oluşturur.
    #[must_use]
    pub fn yeni(klasor: impl Into<PathBuf>, yuva_sayisi: u32) -> Self {
        Self {
            klasor: klasor.into(),
            yuva_sayisi: yuva_sayisi.max(1),
        }
    }

    /// Macera çalışma zamanını belirtilen yuvaya güvenli geçici dosyayla kaydeder.
    ///
    /// # Errors
    ///
    /// Yuva geçersizse veya dosya sistemi işlemi başarısız olursa [`OyunHatasi`] döndürür.
    pub fn kaydet(&self, yuva: u32, macera: &Macera) -> OyunSonucu<PathBuf> {
        let yol = self.yuva_yolu(yuva)?;
        fs::create_dir_all(&self.klasor).map_err(|hata| {
            OyunHatasi::yeni(format!("Kayıt klasörü oluşturulamadı: {hata}"))
        })?;
        let gecici = gecici_yol(&yol);
        let icerik = kaydi_yaz(macera);
        let mut dosya = File::create(&gecici)
            .map_err(|hata| OyunHatasi::yeni(format!("Geçici kayıt dosyası açılamadı: {hata}")))?;
        dosya
            .write_all(icerik.as_bytes())
            .and_then(|()| dosya.sync_all())
            .map_err(|hata| OyunHatasi::yeni(format!("Kayıt verisi diske yazılamadı: {hata}")))?;
        drop(dosya);
        kaydi_degistir(&gecici, &yol)?;
        Ok(yol)
    }

    /// Kayıt yuvasını okuyup tanımları korunmuş macera çalışma zamanına uygular.
    ///
    /// # Errors
    ///
    /// Yuva geçersizse, dosya okunamazsa veya kayıt biçimi bozuksa [`OyunHatasi`] döndürür.
    pub fn yukle(&self, yuva: u32, macera: &mut Macera) -> OyunSonucu<PathBuf> {
        let yol = self.yuva_yolu(yuva)?;
        let icerik = fs::read_to_string(&yol)
            .map_err(|hata| OyunHatasi::yeni(format!("Kayıt dosyası okunamadı: {hata}")))?;
        kaydi_oku(&icerik, macera)?;
        Ok(yol)
    }

    /// Kayıt yuvasının var olup olmadığını döndürür.
    ///
    /// # Errors
    ///
    /// Yuva numarası geçersizse [`OyunHatasi`] döndürür.
    pub fn var_mi(&self, yuva: u32) -> OyunSonucu<bool> {
        Ok(self.yuva_yolu(yuva)?.is_file())
    }

    /// Kayıt yuvasını siler; dosya zaten yoksa başarılı sayılır.
    ///
    /// # Errors
    ///
    /// Yuva geçersizse veya dosya silinemezse [`OyunHatasi`] döndürür.
    pub fn sil(&self, yuva: u32) -> OyunSonucu {
        let yol = self.yuva_yolu(yuva)?;
        if yol.exists() {
            fs::remove_file(&yol)
                .map_err(|hata| OyunHatasi::yeni(format!("Kayıt dosyası silinemedi: {hata}")))?;
        }
        Ok(())
    }

    fn yuva_yolu(&self, yuva: u32) -> OyunSonucu<PathBuf> {
        if yuva >= self.yuva_sayisi {
            return Err(OyunHatasi::yeni(format!(
                "Kayıt yuvası {} sınırının dışında: {yuva}",
                self.yuva_sayisi
            )));
        }
        Ok(self.klasor.join(format!("yuva-{yuva}.tgm")))
    }
}
