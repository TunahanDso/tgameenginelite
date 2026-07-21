# Tgame Engine Lite

**Tgame Engine Lite**, Rust ile geliştirilen; Türkçe, editörsüz, modüler ve performans odaklı bir 2B/3B oyun motoru kütüphanesidir.

Motor ayrı bir editör uygulaması açmaz. Oyun geliştiricisi `tgame` paketini Rust projesine ekler; dünyayı, varlıkları, kamerayı, fiziği, modelleri ve oyun döngüsünü Türkçe API ile kodlar.

## Bugünkü durum

Motor aynı çekirdekte iki grafik yolu çalıştırır:

- Ortografik `Dunya::yeni()` ile 2B üçgen dünyası
- Perspektif ve derinlik tamponlu `Dunya::yeni_3b()` ile küp ve genel mesh dünyası

3B çekirdekte sabit zaman adımlı fizik, statik/dinamik AABB gövdeleri, yerçekimi, zıplama, ham fare kamerası, glTF/GLB yükleme, dünya mesh kayıt defteri ve mesh kimliğine göre GPU toplu çizimi bulunur.

Yüklenen bir mesh dünyada yalnızca bir kez saklanır ve ilk görüldüğü karede yalnızca bir kez GPU tepe/indeks tamponlarına aktarılır. Aynı `MeshKimligi`ni kullanan bütün varlıklar tek instance grubunda çizilir.

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
- `tgame-grafik`: 2B/3B GPU pipeline'ları, mesh kayıtları, instancing, batching ve derinlik tamponu
- `tgame-matematik`: `Vektor2`, `Vektor3`, `Matris4` ve `Renk`
- `tgame-model`: Doğrulanmış `MeshVerisi` ve glTF/GLB yükleme altyapısı
- `tgame-pencere`: İşletim sistemi penceresi, ham aygıt olayları ve imleç yakalama
- `tgame-sahne`: Sahne tanımları
- `tgame-mod`: Modlama sözleşmeleri ve mod kayıt sistemi
- `tgame-varlik`: Kimlikli varlıklar, mesh kaynakları, dönüşümler, görünümler, kameralar ve dünya
- `tgame-zaman`: Kare süresi, toplam çalışma süresi ve kare sayacı

## glTF modelini dünyaya ekleme

```rust
let model = ModelVerisi::gltf_yukle("varliklar/karakter.glb")?;
let meshler = dunya.model_ekle(model);
let govde_mesh = meshler[0];

dunya.varlik_ekle(
    Varlik::mesh("Karakter", govde_mesh, Renk::MAVI)
        .donusum3b(Donusum3B::yeni().konum(Vektor3::yeni(0.0, 0.0, -3.0))),
);
```

Yükleyici üçgen primitive'leri, konumları, normalleri ve indeksleri okur. İndeks yoksa sıralı indeks üretir; normal yoksa üçgenlerden yumuşatılmış tepe normalleri hesaplar. Bozuk veya sınırı aşan mesh verisi GPU'ya ulaşmadan Türkçe `OyunHatasi` ile reddedilir.

`Dunya::model_ekle`, modeldeki her mesh için kalıcı bir `MeshKimligi` üretir. `Varlik::mesh` aynı kimliği yüzlerce varlıkta paylaşabilir. Grafik katmanı varlıkları mesh kimliğine göre gruplayıp her mesh için tek `draw_indexed` çağrısı yapar. Yerleşik küp `u16`, glTF mesh'leri `u32` indeks kullanabilir ve aynı pipeline içinde çizilir.

Şimdiki sınırlar:

- glTF düğüm hiyerarşisi ve düğüm dönüşümleri henüz dünyaya aktarılmıyor.
- UV, doku ve glTF malzemeleri henüz çizilmiyor; varlık başına temel renk kullanılıyor.
- Animasyon, iskelet ve morph target desteği henüz yok.

## Örnekleri çalıştırma

### 3B fizik ve glTF mesh dünyası

```powershell
cargo run -p ilk-oyun
```

Sahne; küplerin yanında `assets/piramit.gltf` dosyasından yüklenen ve tek GPU mesh kaydını paylaşan sekiz dönen piramit içerir.

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

Bu örnek eski `Dunya::yeni()`, `Donusum2B`, `Kamera2B` ve üçgen instancing hattının yeni 3B özelliklerinden sonra da çalıştığını doğrular.

## Kalite denetimi

```powershell
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
```

Aynı denetimler her gönderimde GitHub Actions tarafından Windows üzerinde otomatik çalıştırılır. Örnek piramit glTF dosyası da CI içinde gerçekten yüklenir; tepe, indeks ve hesaplanan normal sayıları doğrulanır.

> Vira bismillah. Her büyük güncelleme motoru daha geniş oyun dünyalarına taşıyacak.
