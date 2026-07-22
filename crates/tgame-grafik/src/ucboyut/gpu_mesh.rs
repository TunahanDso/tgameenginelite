use tgame_cekirdek::{OyunHatasi, OyunSonucu};
use tgame_model::MeshVerisi;

use super::mesh::{KUP_INDEKS_SAYISI, indeks_baytlari, tepe_baytlari};

pub(super) struct GpuMesh {
    pub(super) tepe: wgpu::Buffer,
    pub(super) indeks: wgpu::Buffer,
    pub(super) indeks_sayisi: u32,
    pub(super) indeks_bicimi: wgpu::IndexFormat,
}

impl GpuMesh {
    pub(super) fn kup(aygit: &wgpu::Device, kuyruk: &wgpu::Queue) -> Self {
        let tepe_baytlari = tepe_baytlari();
        let indeks_baytlari = indeks_baytlari();
        Self {
            tepe: tampon_olustur(
                aygit,
                kuyruk,
                "Tgame Yerleşik Küp Tepe Tamponu",
                &tepe_baytlari,
                wgpu::BufferUsages::VERTEX,
            ),
            indeks: tampon_olustur(
                aygit,
                kuyruk,
                "Tgame Yerleşik Küp İndeks Tamponu",
                &indeks_baytlari,
                wgpu::BufferUsages::INDEX,
            ),
            indeks_sayisi: KUP_INDEKS_SAYISI,
            indeks_bicimi: wgpu::IndexFormat::Uint16,
        }
    }

    pub(super) fn kayitli(
        aygit: &wgpu::Device,
        kuyruk: &wgpu::Queue,
        mesh: &MeshVerisi,
    ) -> OyunSonucu<Self> {
        let mut tepe_baytlari = Vec::with_capacity(mesh.konumlar().len().saturating_mul(32));
        for ((konum, normal), uv) in mesh
            .konumlar()
            .iter()
            .zip(mesh.normaller())
            .zip(mesh.uvler())
        {
            for deger in [
                konum.x, konum.y, konum.z, normal.x, normal.y, normal.z, uv.x, uv.y,
            ] {
                tepe_baytlari.extend_from_slice(&deger.to_le_bytes());
            }
        }

        let mut indeks_baytlari = Vec::with_capacity(mesh.indeksler().len().saturating_mul(4));
        for indeks in mesh.indeksler() {
            indeks_baytlari.extend_from_slice(&indeks.to_le_bytes());
        }
        let indeks_sayisi = u32::try_from(mesh.indeksler().len())
            .map_err(|_| OyunHatasi::yeni("GPU mesh'i desteklenenden fazla indeks içeriyor."))?;

        Ok(Self {
            tepe: tampon_olustur(
                aygit,
                kuyruk,
                "Tgame Kayıtlı Mesh Tepe Tamponu",
                &tepe_baytlari,
                wgpu::BufferUsages::VERTEX,
            ),
            indeks: tampon_olustur(
                aygit,
                kuyruk,
                "Tgame Kayıtlı Mesh İndeks Tamponu",
                &indeks_baytlari,
                wgpu::BufferUsages::INDEX,
            ),
            indeks_sayisi,
            indeks_bicimi: wgpu::IndexFormat::Uint32,
        })
    }
}

fn tampon_olustur(
    aygit: &wgpu::Device,
    kuyruk: &wgpu::Queue,
    etiket: &'static str,
    baytlar: &[u8],
    kullanim: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let boyut = u64::try_from(baytlar.len()).unwrap_or(u64::MAX).max(1);
    let tampon = aygit.create_buffer(&wgpu::BufferDescriptor {
        label: Some(etiket),
        size: boyut,
        usage: kullanim | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    if !baytlar.is_empty() {
        kuyruk.write_buffer(&tampon, 0, baytlar);
    }
    tampon
}

#[cfg(test)]
mod testler {
    use tgame_matematik::{Vektor2, Vektor3};
    use tgame_model::{MalzemeVerisi, MeshVerisi};

    #[test]
    fn genel_mesh_tepe_verisi_konum_normal_ve_uv_tasir() {
        let mesh = MeshVerisi::yeni_malzemeli(
            vec![Vektor3::SIFIR, Vektor3::SAG, Vektor3::YUKARI],
            vec![Vektor3::ILERI; 3],
            vec![Vektor2::SIFIR, Vektor2::SAG, Vektor2::YUKARI],
            vec![0, 1, 2],
            MalzemeVerisi::default(),
        )
        .expect("Test mesh'i geçerli olmalı.");

        assert_eq!(mesh.konumlar().len().saturating_mul(32), 96);
        assert_eq!(mesh.uvler()[1], Vektor2::SAG);
    }
}
