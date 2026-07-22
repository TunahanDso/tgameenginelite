//! Tgame Engine Lite ekran arayüzü veri katmanı.

use tgame_matematik::Renk;

/// Ekran metinlerinde kullanılan 8 bit sRGB rengidir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EkranRengi {
    /// Kırmızı kanal.
    pub kirmizi: u8,
    /// Yeşil kanal.
    pub yesil: u8,
    /// Mavi kanal.
    pub mavi: u8,
    /// Alfa kanalı.
    pub alfa: u8,
}

impl EkranRengi {
    /// Beyaz ekran rengi.
    pub const BEYAZ: Self = Self::yeni(255, 255, 255, 255);
    /// Açık sarı vurgu rengi.
    pub const SARI: Self = Self::yeni(255, 220, 105, 255);
    /// Açık mavi bilgi rengi.
    pub const MAVI: Self = Self::yeni(130, 195, 255, 255);
    /// Açık yeşil başarı rengi.
    pub const YESIL: Self = Self::yeni(130, 235, 160, 255);

    /// Yeni ekran rengi oluşturur.
    #[must_use]
    pub const fn yeni(kirmizi: u8, yesil: u8, mavi: u8, alfa: u8) -> Self {
        Self {
            kirmizi,
            yesil,
            mavi,
            alfa,
        }
    }
}

impl Default for EkranRengi {
    fn default() -> Self {
        Self::BEYAZ
    }
}

/// Ekran üzerindeki iki boyutlu dikdörtgen alanı piksel cinsinden tanımlar.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EkranDikdortgeni {
    /// Sol kenar.
    pub sol: f32,
    /// Üst kenar.
    pub ust: f32,
    /// Genişlik.
    pub genislik: f32,
    /// Yükseklik.
    pub yukseklik: f32,
}

impl EkranDikdortgeni {
    /// Yeni ekran dikdörtgeni oluşturur.
    #[must_use]
    pub const fn yeni(sol: f32, ust: f32, genislik: f32, yukseklik: f32) -> Self {
        Self {
            sol,
            ust,
            genislik,
            yukseklik,
        }
    }
}

/// Arayüz panelinin görünüşünü tanımlar.
#[derive(Debug, Clone, PartialEq)]
pub struct ArayuzPaneli {
    alan: EkranDikdortgeni,
    renk: Renk,
}

impl ArayuzPaneli {
    /// Alan ve renkle panel oluşturur.
    #[must_use]
    pub const fn yeni(alan: EkranDikdortgeni, renk: Renk) -> Self {
        Self { alan, renk }
    }

    /// Panel alanını döndürür.
    #[must_use]
    pub const fn alan(&self) -> EkranDikdortgeni {
        self.alan
    }

    /// Panel rengini döndürür.
    #[must_use]
    pub const fn renk(&self) -> Renk {
        self.renk
    }
}

/// Ekrana çizilecek şekillendirilmiş metin tanımıdır.
#[derive(Debug, Clone, PartialEq)]
pub struct EkranMetni {
    metin: String,
    alan: EkranDikdortgeni,
    punto: f32,
    satir_yuksekligi: f32,
    renk: EkranRengi,
}

impl EkranMetni {
    /// Metin, alan ve puntoyla ekran metni oluşturur.
    #[must_use]
    pub fn yeni(metin: impl Into<String>, alan: EkranDikdortgeni, punto: f32) -> Self {
        let punto = if punto.is_finite() && punto > 1.0 {
            punto
        } else {
            18.0
        };
        Self {
            metin: metin.into(),
            alan,
            punto,
            satir_yuksekligi: punto * 1.3,
            renk: EkranRengi::BEYAZ,
        }
    }

    /// Metin rengini değiştirir.
    #[must_use]
    pub const fn renk(mut self, renk: EkranRengi) -> Self {
        self.renk = renk;
        self
    }

    /// Satır yüksekliğini değiştirir.
    #[must_use]
    pub fn satir_yuksekligi(mut self, yukseklik: f32) -> Self {
        if yukseklik.is_finite() && yukseklik >= self.punto {
            self.satir_yuksekligi = yukseklik;
        }
        self
    }

    /// Metin içeriğini döndürür.
    #[must_use]
    pub fn metin(&self) -> &str {
        &self.metin
    }

    /// Metin alanını döndürür.
    #[must_use]
    pub const fn alan(&self) -> EkranDikdortgeni {
        self.alan
    }

    /// Puntoyu döndürür.
    #[must_use]
    pub const fn punto(&self) -> f32 {
        self.punto
    }

    /// Satır yüksekliğini döndürür.
    #[must_use]
    pub const fn satir_yuksekligi_degeri(&self) -> f32 {
        self.satir_yuksekligi
    }

    /// Metin rengini döndürür.
    #[must_use]
    pub const fn renk_degeri(&self) -> EkranRengi {
        self.renk
    }
}

/// Bir karede çizilecek ekran arayüzü öğelerini saklar.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Arayuz {
    paneller: Vec<ArayuzPaneli>,
    metinler: Vec<EkranMetni>,
}

impl Arayuz {
    /// Boş arayüz oluşturur.
    #[must_use]
    pub const fn yeni() -> Self {
        Self {
            paneller: Vec::new(),
            metinler: Vec::new(),
        }
    }

    /// Önceki karenin bütün öğelerini temizler.
    pub fn temizle(&mut self) {
        self.paneller.clear();
        self.metinler.clear();
    }

    /// Arayüze panel ekler.
    pub fn panel_ekle(&mut self, panel: ArayuzPaneli) {
        self.paneller.push(panel);
    }

    /// Arayüze metin ekler.
    pub fn metin_ekle(&mut self, metin: EkranMetni) {
        self.metinler.push(metin);
    }

    /// Panelleri eklenme sırasıyla döndürür.
    #[must_use]
    pub fn paneller(&self) -> &[ArayuzPaneli] {
        &self.paneller
    }

    /// Metinleri eklenme sırasıyla döndürür.
    #[must_use]
    pub fn metinler(&self) -> &[EkranMetni] {
        &self.metinler
    }
}

#[cfg(test)]
mod testler {
    use super::{Arayuz, ArayuzPaneli, EkranDikdortgeni, EkranMetni};
    use tgame_matematik::Renk;

    #[test]
    fn arayuz_temizligi_butun_ogeleri_siler() {
        let alan = EkranDikdortgeni::yeni(10.0, 10.0, 200.0, 80.0);
        let mut arayuz = Arayuz::yeni();
        arayuz.panel_ekle(ArayuzPaneli::yeni(
            alan,
            Renk::yeni(0.0, 0.0, 0.0, 0.75),
        ));
        arayuz.metin_ekle(EkranMetni::yeni("Görev", alan, 20.0));

        arayuz.temizle();

        assert!(arayuz.paneller().is_empty());
        assert!(arayuz.metinler().is_empty());
    }
}
