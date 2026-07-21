use std::path::PathBuf;

use tgame::onsoz::ModelVerisi;

#[test]
fn piramit_gltf_gercekten_mesh_uretir() {
    let yol = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/piramit.gltf");
    let model = ModelVerisi::gltf_yukle(yol).expect("Piramit glTF varlığı yüklenebilmeli.");
    let mesh = model
        .meshler()
        .first()
        .expect("Piramit modeli en az bir mesh üretmeli.");

    assert_eq!(mesh.konumlar().len(), 5);
    assert_eq!(mesh.normaller().len(), 5);
    assert_eq!(mesh.indeksler().len(), 18);
    assert!(mesh.normaller().iter().all(|normal| normal.uzunluk() > 0.9));
}
