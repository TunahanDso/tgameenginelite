use std::path::PathBuf;

use tgame::onsoz::{DokuFiltresi, DokuSarmasi, ModelVerisi};

#[test]
fn piramit_gltf_gercekten_dokulu_mesh_uretir() {
    let yol = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/piramit.gltf");
    let model = ModelVerisi::gltf_yukle(yol).expect("Piramit glTF varlığı yüklenebilmeli.");
    let mesh = model
        .meshler()
        .first()
        .expect("Piramit modeli en az bir mesh üretmeli.");
    let doku = mesh
        .malzeme()
        .temel_dokusu()
        .expect("Piramit malzemesi taban renk dokusu taşımalı.");
    let ornekleyici = doku.ornekleyici();

    assert_eq!(mesh.konumlar().len(), 5);
    assert_eq!(mesh.normaller().len(), 5);
    assert_eq!(mesh.uvler().len(), 5);
    assert_eq!(mesh.indeksler().len(), 18);
    assert!(mesh.normaller().iter().all(|normal| normal.uzunluk() > 0.9));
    assert_eq!(doku.genislik(), 2);
    assert_eq!(doku.yukseklik(), 2);
    assert_eq!(doku.rgba8().len(), 16);
    assert_eq!(ornekleyici.buyutme, DokuFiltresi::Dogrusal);
    assert_eq!(ornekleyici.kucultme, DokuFiltresi::Dogrusal);
    assert_eq!(ornekleyici.sarma_u, DokuSarmasi::Tekrarla);
    assert_eq!(ornekleyici.sarma_v, DokuSarmasi::Tekrarla);
}
