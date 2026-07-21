# Tgame Engine Lite

**Tgame Engine Lite**, Rust ile geliştirilen; Türkçe, editörsüz, modüler ve performans odaklı bir 2B/3B oyun motoru kütüphanesidir.

Motor ayrı bir editör uygulaması açmaz. Oyun geliştiricisi `tgame` paketini Rust projesine ekler; dünyayı, varlıkları, kamerayı, fiziği ve oyun döngüsünü Türkçe API ile kodlar.

## Bugünkü durum

Motor aynı çekirdekte iki grafik yolu çalıştırır:

- Ortografik `Dunya::yeni()` ile 2B üçgen dünyası
- Perspektif ve derinlik tamponlu `Dunya::yeni_3b()` ile 3B küp dünyası

3B çekirdekte artık sabit zaman adımlı fizik, statik/dinamik AABB gövdeleri, yerçekimi, zeminde olma, zıplama, ham fare kamerası ve glTF/GLB dosyalarını genel mesh verisine çeviren model yükleme katmanı bulunur.

> `tgame-model` şu anda modeli CPU tarafında doğrulanmış `MeshVerisi` olarak yükler. Yüklenen glTF mesh'lerini genel GPU mesh kayıt sisteminde çizmek bir sonraki grafik aşamasıdır.

## Temel kararlar

- Programlama dili: Rust
- En düşük Rust sürümü: 1.87
- Kullanıcı API'si: Türkçe
- Motor türü: Editörsüz, kütüphane tabanlı
- Grafik: wgpu 30
- Fizik: Sabit yaklaşık 60 Hz adım, eksenlere hizalı 3B çarpışma
- Mimari: Bağımsız paketlere ayrılmış Cargo workspace
- Öncelik: Performans, kalite, anlaşılabilirlik ve geriye dönük uyumluluk
- Oyunlar: Baştan itibaren modlanabilir tasarlanacak
- Kalite kuralı: Uyarılar derleme hatası kabul edilir
- Güvenlik kuralı: Workspace içinde `unsafe` kod yasaktır

## Paketler

- `tgame`: Oyun geliştiricisinin kullandığı Türkçe üst API
- `tgame-cekirdek`: Ayarlar, çözünürlük, hata ve sonuç türleri
- `tgame-fizik`: Sabit zaman adımı, statik/dinamik gövdeler, AABB çarpışma, yerçekimi ve zıplama
- `tgame-girdi`: Türkçe fiziksel klavye tuşları ve karelik ham fare hareketi
- `tgame-grafik`: 2B/3B GPU pipeline'ları, instancing, indeksli mesh ve derinlik tamponu
- `tgame-matematik`: `Vektor2`, `Vektor3`, `Matris4` ve `Renk`
- `tgame-model`: Genel `MeshVerisi` ve glTF/GLB dosya yükleme altyapısı
- `tgame-pencere`: İşletim sistemi penceresi, ham aygıt olayları ve imleç yakalama
- `tgame-sahne`: Sahne tanımları
- `tgame-mod`: Modlama sözleşmeleri ve mod kayıt sistemi
- `tgame-varlik`: Kimlikli varlıklar, 2B/3B dönüşümler, görünümler, kameralar ve dünya
- `tgame-zaman`: Kare süresi, toplam çalışma süresi ve kare sayacı

## Sabit fizik örneği

```rust
use tgame::onsoz::{FizikDunyasi, FizikGovdesi, Varlik, Vektor3};

let oyuncu = dunya.varlik_ekle(Varlik::kup("Oyuncu", Renk::SARI));
let zemin = dunya.varlik_ekle(Varlik::kup("Zemin", Renk::YESIL));

let mut fizik = FizikDunyasi::yeni();
fizik.govde_ekle(FizikGovdesi::dinamik_kup(oyuncu, Vektor3::BIR));
fizik.govde_ekle(FizikGovdesi::statik_kup(
    zemin,
    Vektor3::yeni(10.0, 1.0, 10.0),
));

// Her render karesinde geçen gerçek süre sabit fizik adımlarına bölünür.
fizik.guncelle(&mut dunya, zaman.kare_suresi());
```

Fizik birikimi render hızından bağımsız sabit adımlarla işlenir. Uzun takılmalarda kare süresi ve alt adım sayısı sınırlandırılarak ölüm sarmalı engellenir. Dinamik gövdeler X, Y ve Z eksenlerinde ayrı çözülür; böylece duvara çarpan gövde diğer eksenlerde kaymaya devam eder.

## glTF/GLB yükleme

```rust
use tgame::onsoz::ModelVerisi;

let model = ModelVerisi::gltf_yukle("varliklar/karakter.glb")?;
println!("{} mesh yüklendi", model.meshler().len());
```

Yükleyici üçgen primitive'leri, konumları, normalleri ve indeksleri okur. İndeks yoksa sıralı indeks üretir; normal yoksa üçgenlerden yumuşatılmış tepe normalleri hesaplar. Bozuk veya sınırı aşan mesh verisi Türkçe `OyunHatasi` ile reddedilir.

## Örnekleri çalıştırma

### 3B fizik dünyası

```powershell
cargo run -p ilk-oyun
```

Kontroller:

- Fare: kamerayı oyuncunun çevresinde döndürür
- `WASD`: kamera yönüne göre oyuncuyu hareket ettirir
- `Boşluk`: oyuncu zemindeyse zıplatır
- Yön tuşları: fareye alternatif kamera kontrolü
- `Enter`: konum, hız ve zeminde olma durumunu yazdırır
- `Escape`: kontrollü kapanış

### 2B uyumluluk örneği

```powershell
cargo run -p ikiboyut-oyun
```

Bu örnek eski `Dunya::yeni()`, `Donusum2B`, `Kamera2B` ve üçgen instancing hattının 3B/fizik güncellemelerinden sonra da çalıştığını doğrular.

## Kalite denetimi

```powershell
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
```

Aynı denetimler her gönderimde GitHub Actions tarafından Windows üzerinde otomatik çalıştırılır.

> Vira bismillah. Her büyük güncelleme motoru daha geniş oyun dünyalarına taşıyacak.
