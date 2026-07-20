//! Tgame Engine Lite sahne tanımları.

/// Bir oyundaki bağımsız bölümü veya ekranı temsil eder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sahne {
    ad: String,
}

impl Sahne {
    #[must_use]
    pub fn yeni(ad: impl Into<String>) -> Self {
        Self { ad: ad.into() }
    }

    #[must_use]
    pub fn ad(&self) -> &str {
        &self.ad
    }
}
