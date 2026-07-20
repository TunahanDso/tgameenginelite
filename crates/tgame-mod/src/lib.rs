//! Tgame Engine Lite modlama sözleşmeleri.

/// Bir modun motor tarafından okunabilen kimlik bilgileri.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModBilgisi {
    pub kimlik: String,
    pub ad: String,
    pub surum: String,
}

impl ModBilgisi {
    #[must_use]
    pub fn yeni(
        kimlik: impl Into<String>,
        ad: impl Into<String>,
        surum: impl Into<String>,
    ) -> Self {
        Self {
            kimlik: kimlik.into(),
            ad: ad.into(),
            surum: surum.into(),
        }
    }
}

/// Motorun yüklediği modların hafif ve güvenli kaydını tutar.
#[derive(Debug, Default)]
pub struct ModYoneticisi {
    yuklu_modlar: Vec<ModBilgisi>,
}

impl ModYoneticisi {
    #[must_use]
    pub fn yeni() -> Self {
        Self::default()
    }

    pub fn kaydet(&mut self, mod_bilgisi: ModBilgisi) {
        self.yuklu_modlar.push(mod_bilgisi);
    }

    #[must_use]
    pub fn yuklu_modlar(&self) -> &[ModBilgisi] {
        &self.yuklu_modlar
    }
}
