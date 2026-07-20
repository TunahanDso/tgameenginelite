# Tgame Engine Lite

**Tgame Engine Lite**, Rust ile geliştirilen; Türkçe, sade, modüler ve performans odaklı bir oyun motoru kütüphanesidir. Uzun vadeli hedefi 3B oyunlar olsa da motorun temeli önce sağlam bir 2B varlık, kamera ve GPU çizim sistemiyle kurulmaktadır.

Motorun kendi editör uygulaması yoktur. Oyun geliştiricileri motoru bir Rust kütüphanesi olarak projelerine ekler ve oyunlarını Türkçe API ile kodlar.

## Temel kararlar

- Programlama dili: Rust
- En düşük Rust sürümü: 1.87
- Başlangıç çözünürlüğü: 800×600
- Kullanıcı API'si: Türkçe
- Mimari: Bağımsız paketlere ayrılmış modüler Cargo workspace
- Öncelik: Performans, kalite ve anlaşılabilirlik
- Oyunlar: Baştan itibaren modlanabilir tasarlanacak
- Motor türü: Editörsüz, kütüphane tabanlı
- Kalite kuralı: Uyarılar derleme hatası kabul edilir
- Güvenlik kuralı: Motor workspace'inde `unsafe` kod yasaktır

## Paketler

- `tgame`: Oyun geliştiricisinin kullandığı sade Türkçe üst API
- `tgame-cekirdek`: Ortak ayarlar, çözünürlük, hata ve sonuç türleri
- `tgame-girdi`: Türkçe fiziksel klavye tuşları ve karelik basma/bırakma durumları
- `tgame-grafik`: wgpu tabanlı yüzey, kamera uniform'u, GPU instancing ve kare sunumu
- `tgame-matematik`: `Vektor2`, yön sabitleri ve doğrusal RGBA `Renk` türü
- `tgame-pencere`: İşletim sistemi penceresi, olay döngüsü ve sistem olaylarının yönlendirilmesi
- `tgame-sahne`: Sahne tanımları
- `tgame-mod`: Modlama sözleşmeleri ve mod kayıt sistemi
- `tgame-varlik`: Kimlikli varlıklar, `Donusum2B`, görünüm, dünya ve `Kamera2B`
- `tgame-zaman`: Kare süresi, toplam çalışma süresi ve kare sayacı

## Oynanabilir ilk dünya

```rust
use tgame::onsoz::{
    Donusum2B, Dunya, Oyun, OyunAkisi, OyunSonucu, Renk, Tus, Varlik, Vektor2,
};

fn main() -> OyunSonucu {
    let mut dunya = Dunya::yeni();
    let oyuncu = dunya.varlik_ekle(
        Varlik::ucgen("Oyuncu", Renk::SARI)
            .donusum(Donusum2B::yeni().olcek(Vektor2::yeni(0.5, 0.5))),
    );

    Oyun::yeni("İlk Tgame Oyunum")
        .dunya(dunya)
        .her_kare(move |girdi, zaman, dunya| {
            let mut yon = Vektor2::SIFIR;

            if girdi.basili_mi(Tus::W) {
                yon += Vektor2::YUKARI;
            }
            if girdi.basili_mi(Tus::D) {
                yon += Vektor2::SAG;
            }

            if let Some(varlik) = dunya.varlik_mut(oyuncu) {
                varlik
                    .donusumu_mut()
                    .tasi(yon.birim() * 2.8 * zaman.kare_saniyesi());
            }

            if girdi.bu_kare_basildi_mi(Tus::Kacis) {
                OyunAkisi::Kapat
            } else {
                OyunAkisi::DevamEt
            }
        })
        .calistir()
}
```

Tam örnek; bir oyuncu ve sekiz dekor varlığı oluşturur. Oyuncu WASD veya yön tuşlarıyla kare hızından bağımsız hareket eder, hareket ederken döner ve kamera oyuncuyu takip eder. Bütün etkin üçgen varlıklar tek bir GPU instance buffer'ına yazılıp tek toplu çizim çağrısıyla çizilir.

## İlk oyunu çalıştırma

```powershell
cargo run -p ilk-oyun
```

Kontroller:

- `WASD` veya yön tuşları: hareket
- `Boşluk`: konum, kare ve süre bilgisini konsola yazdır
- `Escape`: kontrollü kapanış

## Kalite denetimi

```powershell
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
```

Aynı denetimler her gönderimde GitHub Actions tarafından Windows üzerinde otomatik olarak çalıştırılır.

> Vira bismillah. Her güncelleme, Tgame Engine Lite ile daha ayrıntılı oyunlar yapılabilmesini sağlayacak.
