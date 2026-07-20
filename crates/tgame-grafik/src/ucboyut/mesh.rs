pub(super) const TEPE_ADIMI_GPU: u64 = 24;
pub(super) const KUP_TEPE_TAMPON_BOYUTU: u64 = 576;
pub(super) const KUP_INDEKS_TAMPON_BOYUTU: u64 = 72;
pub(super) const KUP_INDEKS_SAYISI: u32 = 36;
pub(super) const TEPE_NITELIKLERI: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

const KUP_TEPELERI: [[f32; 6]; 24] = [
    [-0.5, -0.5, 0.5, 0.0, 0.0, 1.0],
    [0.5, -0.5, 0.5, 0.0, 0.0, 1.0],
    [0.5, 0.5, 0.5, 0.0, 0.0, 1.0],
    [-0.5, 0.5, 0.5, 0.0, 0.0, 1.0],
    [0.5, -0.5, -0.5, 0.0, 0.0, -1.0],
    [-0.5, -0.5, -0.5, 0.0, 0.0, -1.0],
    [-0.5, 0.5, -0.5, 0.0, 0.0, -1.0],
    [0.5, 0.5, -0.5, 0.0, 0.0, -1.0],
    [0.5, -0.5, 0.5, 1.0, 0.0, 0.0],
    [0.5, -0.5, -0.5, 1.0, 0.0, 0.0],
    [0.5, 0.5, -0.5, 1.0, 0.0, 0.0],
    [0.5, 0.5, 0.5, 1.0, 0.0, 0.0],
    [-0.5, -0.5, -0.5, -1.0, 0.0, 0.0],
    [-0.5, -0.5, 0.5, -1.0, 0.0, 0.0],
    [-0.5, 0.5, 0.5, -1.0, 0.0, 0.0],
    [-0.5, 0.5, -0.5, -1.0, 0.0, 0.0],
    [-0.5, 0.5, 0.5, 0.0, 1.0, 0.0],
    [0.5, 0.5, 0.5, 0.0, 1.0, 0.0],
    [0.5, 0.5, -0.5, 0.0, 1.0, 0.0],
    [-0.5, 0.5, -0.5, 0.0, 1.0, 0.0],
    [-0.5, -0.5, -0.5, 0.0, -1.0, 0.0],
    [0.5, -0.5, -0.5, 0.0, -1.0, 0.0],
    [0.5, -0.5, 0.5, 0.0, -1.0, 0.0],
    [-0.5, -0.5, 0.5, 0.0, -1.0, 0.0],
];

const KUP_INDEKSLERI: [u16; 36] = [
    0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14,
    15, 16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
];

pub(super) fn tepe_baytlari() -> Vec<u8> {
    let mut baytlar = Vec::with_capacity(576);
    for tepe in KUP_TEPELERI {
        for deger in tepe {
            baytlar.extend_from_slice(&deger.to_le_bytes());
        }
    }
    baytlar
}

pub(super) fn indeks_baytlari() -> Vec<u8> {
    let mut baytlar = Vec::with_capacity(72);
    for indeks in KUP_INDEKSLERI {
        baytlar.extend_from_slice(&indeks.to_le_bytes());
    }
    baytlar
}

#[cfg(test)]
mod testler {
    use super::{indeks_baytlari, tepe_baytlari, KUP_INDEKS_SAYISI};

    #[test]
    fn kup_mesh_boyutlari_dogrudur() {
        assert_eq!(tepe_baytlari().len(), 576);
        assert_eq!(indeks_baytlari().len(), 72);
        assert_eq!(KUP_INDEKS_SAYISI, 36);
    }
}
