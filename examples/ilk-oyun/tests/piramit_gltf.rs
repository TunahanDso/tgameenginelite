use std::path::PathBuf;

use tgame::onsoz::{DokuFiltresi, DokuSarmasi, Dunya, ModelVerisi, Renk, Vektor3};

fn yakin(sol: f32, sag: f32) -> bool {
    (sol - sag).abs() < 0.000_01
}

#[test]
fn piramit_gltf_doku_node_hiyerarsisi_ve_kaynaklar_uretir() {
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
    let sahne_ornegi = model
        .ornekler()
        .first()
        .copied()
        .expect("Varsayılan glTF sahnesi bir mesh örneği üretmeli.");
    let node_konumu = sahne_ornegi
        .dunya_matrisi()
        .noktayi_donustur(Vektor3::SIFIR);

    assert_eq!(mesh.konumlar().len(), 5);
    assert_eq!(mesh.normaller().len(), 5);
    assert_eq!(mesh.uvler().len(), 5);
    assert_eq!(mesh.indeksler().len(), 18);
    assert!(mesh.normaller().iter().all(|normal| normal.uzunluk() > 0.9));
    assert!(mesh.sinir_kuresi().yaricap() > 1.0);
    assert_eq!(doku.genislik(), 2);
    assert_eq!(doku.yukseklik(), 2);
    assert_eq!(doku.rgba8().len(), 16);
    assert_eq!(ornekleyici.buyutme, DokuFiltresi::Dogrusal);
    assert_eq!(ornekleyici.kucultme, DokuFiltresi::Dogrusal);
    assert_eq!(ornekleyici.sarma_u, DokuSarmasi::Tekrarla);
    assert_eq!(ornekleyici.sarma_v, DokuSarmasi::Tekrarla);
    assert_eq!(model.ornekler().len(), 1);
    assert_eq!(sahne_ornegi.mesh_indeksi(), 0);
    assert!(yakin(node_konumu.x, 1.25));
    assert!(yakin(node_konumu.y, 1.5));
    assert!(yakin(node_konumu.z, -0.75));
    assert!(yakin(sahne_ornegi.dunya_matrisi().en_buyuk_olcek(), 0.8));

    let mut dunya = Dunya::yeni_3b();
    let varliklar = dunya
        .model_sahnesi_ekle("İçe Aktarılan Piramit", model, Renk::BEYAZ)
        .expect("Model sahnesi dünyaya eklenebilmeli.");
    assert_eq!(varliklar.len(), 1);
    assert_eq!(dunya.meshler().len(), 1);
    assert_eq!(dunya.malzemeler().len(), 1);
    assert_eq!(dunya.dokular().len(), 1);
}
