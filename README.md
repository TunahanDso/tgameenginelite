# Tgame Engine Lite

**Tgame Engine Lite**, Rust ile geliştirilen; Türkçe, editörsüz, modüler ve performans odaklı bir 2B/3B oyun motoru kütüphanesidir.

Motor ayrı bir editör uygulaması açmaz. Oyun geliştiricisi `tgame` paketini Rust projesine ekler; dünyayı, varlıkları, kamerayı ve oyun döngüsünü Türkçe API ile kodlar.

## Bugünkü durum

Motor aynı çekirdekte iki grafik yolu çalıştırır:

- Ortografik `Dunya::yeni()` ile 2B üçgen dünyası
- Perspektif ve derinlik tamponlu `Dunya::yeni_3b()` ile 3B küp dünyası

2B ve 3B varlıklar aynı `VarlikKimligi`, `Dunya`, girdi, zaman, pencere ve oyun döngüsü altyapısını paylaşır. Grafik katmanı dünyanın boyutuna göre doğru GPU pipeline'ını seçer.

## Temel kararlar

- Programlama dili: Rust
- En düşük Rust sürümü: 1.87
- Kullanıcı API'si: Türkçe
- Motor türü: Editörsüz, kütüphane tabanlı
- Grafik: wgpu 30
- Mimari: Bağımsız paketlere ayrılmış Cargo workspace
- Öncelik: Performans, kalite, anlaşılabilirlik ve geriye dönük uyumluluk
- Oyunlar: Baştan itibaren modlanabilir tasarlanacak
- Kalite kuralı: Uyarılar derleme hatası kabul edilir
- Güvenlik kuralı: Workspace içinde `unsafe` kod yasaktır

## Paketler

- `tgame`: Oyun geliştiricisinin kullandığı Türkçe üst API
- `tgame-cekirdek`: Ayarlar, çözünürlük, hata ve sonuç türleri
- `tgame-girdi`: Türkçe fiziksel klavye tuşları ve karelik durumlar
- `tgame-grafik`: 2B/3B GPU pipeline'ları, instancing, indeksli mesh ve derinlik tamponu
- `tgame-matematik`: `Vektor2`, `Vektor3`, `Matris4` ve `Renk`
- `tgame-pencere`: İşletim sistemi penceresi ve olay döngüsü
- `tgame-sahne`: Sahne tanımları
- `tgame-mod`: Modlama sözleşmeleri ve mod kayıt sistemi
- `tgame-varlik`: Kimlikli varlıklar, 2B/3B dönüşümler, görünümler, kameralar ve dünya
- `tgame-zaman`: Kare süresi, toplam çalışma süresi ve kare sayacı

## İlk 3B dünya

```rust
use tgame::onsoz::{Donusum3B, Dunya, Oyun, OyunAkisi, Renk, Varlik, Vektor3};

let mut dunya = Dunya::yeni_3b();
let oyuncu = dunya.varlik_ekle(
    Varlik::kup("Oyuncu", Renk::SARI).donusum3b(
        Donusum3B::yeni()
            .konum(Vektor3::yeni(0.0, 0.0, 3.0))
            .olcek(Vektor3::yeni(0.75, 0.75, 0.75)),
    ),
);

Oyun::yeni("3B Oyunum")
    .dunya(dunya)
    .her_kare(move |girdi, zaman, dunya| {
        if let Some(varlik) = dunya.varlik_mut(oyuncu) {
            varlik
                .donusumu3b_mut()
                .dondur(Vektor3::YUKARI * zaman.kare_saniyesi());
        }
        let _ = girdi;
        OyunAkisi::DevamEt
    });
```

3B çizici şunları birlikte kullanır:

- Sağ elli dünya koordinatları
- Perspektif `Kamera3B`
- 4×4 model, görünüm ve izdüşüm matrisleri
- 24 normalli tepe ve 36 indeksli ortak küp mesh'i
- Her küp için model matrisi ve renk taşıyan GPU instance verisi
- `Depth32Float` derinlik dokusu ve depth test
- Yüzey normallerine dayalı temel yönsel aydınlatma

## Örnekleri çalıştırma

### 3B dünya

```powershell
cargo run -p ilk-oyun
```

Kontroller:

- `WASD`: oyuncuyu X-Z düzleminde hareket ettirir
- Sol/sağ yön tuşları: kamerayı oyuncunun çevresinde döndürür
- Yukarı/aşağı yön tuşları: kamera yüksekliğini değiştirir
- `Boşluk`: oyuncu konumu, kare ve süre bilgisini yazdırır
- `Escape`: kontrollü kapanış

### 2B uyumluluk örneği

```powershell
cargo run -p ikiboyut-oyun
```

Bu örnek eski `Dunya::yeni()`, `Donusum2B`, `Kamera2B` ve üçgen instancing hattının 3B güncellemelerinden sonra da çalıştığını doğrular.

## Kalite denetimi

```powershell
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
```

Aynı denetimler her gönderimde GitHub Actions tarafından Windows üzerinde otomatik çalıştırılır.

> Vira bismillah. Her büyük güncelleme motoru daha geniş oyun dünyalarına taşıyacak.
