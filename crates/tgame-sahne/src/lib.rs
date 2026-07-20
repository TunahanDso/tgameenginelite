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

#[cfg(test)]
mod testler {
    use super::Sahne;

    #[test]
    fn sahne_adini_korur() {
        let sahne = Sahne::yeni("Başlangıç");

        assert_eq!(sahne.ad(), "Başlangıç");
    }
}
