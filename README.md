# Tgame Engine Lite

**Tgame Engine Lite**, Rust ile geliştirilen; Türkçe, sade, modüler ve performans odaklı bir 3B oyun motoru kütüphanesidir.

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
- `tgame-grafik`: wgpu tabanlı GPU yüzeyi, çizim hattı ve kare sunumu
- `tgame-pencere`: İşletim sistemi penceresi, olay döngüsü ve grafik olaylarının yönlendirilmesi
- `tgame-sahne`: Sahne tanımları
- `tgame-mod`: Modlama sözleşmeleri ve mod kayıt sistemi

## İlk oyun

```rust
use tgame::onsoz::{Oyun, OyunSonucu, Sahne};

fn main() -> OyunSonucu {
    Oyun::yeni("İlk Tgame Oyunum")
        .cozunurluk(800, 600)
        .mod_klasoru("modlar")
        .sahne_ekle(Sahne::yeni("Başlangıç"))
        .calistir()
}
```

Bu örnek gerçek bir 800×600 işletim sistemi penceresi oluşturur, yüksek performanslı GPU bağdaştırıcısını seçer ve WGSL gölgelendiricisiyle renkli bir üçgen çizer. Pencere yeniden boyutlandırıldığında GPU yüzeyi güncellenir; yüzey kaybı ve geçersizleşme durumları kontrollü biçimde ele alınır.

Çalıştırmak için:

```powershell
cargo run -p ilk-oyun
```

## Kalite denetimi

```powershell
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
```

Aynı denetimler her gönderimde GitHub Actions tarafından Windows üzerinde otomatik olarak çalıştırılır.

> Vira bismillah. Her güncelleme, Tgame Engine Lite ile daha ayrıntılı oyunlar yapılabilmesini sağlayacak.
