//! Tgame Engine Lite hikâye, görev, diyalog, etkileşim ve kayıt altyapısı.

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};

use tgame_cekirdek::{OyunHatasi, OyunSonucu};
use tgame_matematik::Vektor3;

const KAYIT_IMZASI: &str = "TGMSAVE";
const KAYIT_SURUMU: u32 = 1;
const AZAMI_ZINCIR_OLAYI: usize = 1_024;
const SON_OLAY_KAPASITESI: usize = 64;

macro_rules! kimlik_turu {
    ($ad:ident, $belge:literal) => {
        #[doc = $belge]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $ad(String);

        impl $ad {
            /// Metin değerinden kimlik oluşturur.
            #[must_use]
            pub fn yeni(deger: impl Into<String>) -> Self {
                Self(deger.into())
            }

            /// Kimliğin metin değerini döndürür.
            #[must_use]
            pub fn deger(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $ad {
            fn from(deger: &str) -> Self {
                Self::yeni(deger)
            }
        }

        impl From<String> for $ad {
            fn from(deger: String) -> Self {
                Self::yeni(deger)
            }
        }
    };
}

kimlik_turu!(EsyaKimligi, "Bir envanter eşyasının kalıcı kimliğidir.");
kimlik_turu!(GorevKimligi, "Bir hikâye görevinin kalıcı kimliğidir.");
kimlik_turu!(DiyalogKimligi, "Bir diyalog ağacının kalıcı kimliğidir.");
kimlik_turu!(SahneKimligi, "Bir oyun bölümünün veya sahnesinin kalıcı kimliğidir.");
kimlik_turu!(EtkilesimKimligi, "Dünyadaki bir etkileşim noktasının kalıcı kimliğidir.");
kimlik_turu!(AlanKimligi, "Dünyadaki bir tetik alanının kalıcı kimliğidir.");
kimlik_turu!(KuralKimligi, "Olay tabanlı bir oynanış kuralının kalıcı kimliğidir.");
kimlik_turu!(KontrolNoktasiKimligi, "Bir yeniden doğuş veya kayıt noktasının kalıcı kimliğidir.");

include!("parcalar/durum.rs");
include!("parcalar/olay_gorev.rs");
include!("parcalar/diyalog.rs");
include!("parcalar/dunya_icerik.rs");
include!("parcalar/eylem_yapi.rs");
include!("parcalar/macera.rs");
include!("parcalar/kayit.rs");
include!("parcalar/testler.rs");
