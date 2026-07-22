# Tgame Engine Lite

**Tgame Engine Lite**, Rust ile geliştirilen; Türkçe, editörsüz, modüler ve performans odaklı bir 2B/3B oyun motoru kütüphanesidir.

Motor ayrı bir editör uygulaması açmaz. Oyun geliştiricisi `tgame` paketini Rust projesine ekler; dünyayı, varlıkları, kamerayı, fiziği, modelleri, malzemeleri ve oyun döngüsünü Türkçe API ile kodlar.

## Bugünkü durum

Motor aynı çekirdekte iki grafik yolu çalıştırır:

- Ortografik `Dunya::yeni()` ile 2B üçgen dünyası
- Perspektif ve derinlik tamponlu `Dunya::yeni_3b()` ile dokulu küp ve genel mesh dünyası

3B çekirdekte şunlar çalışır:

- Sabit zaman adımlı fizik ve AABB çarpışma
- Ham fare kamerası, yürüme ve zıplama
- glTF/GLB geometri, UV, taban renk dokusu ve sampler yükleme
- glTF varsayılan sahnesi, ebeveyn–çocuk node hiyerarşisi ve birikmiş dünya matrisleri
- Bağımsız `MeshKimligi`, `MalzemeKimligi` ve `DokuKimligi`
- Eşit doku ve malzemelerin otomatik tekilleştirilmesi
- Aynı mesh'in farklı malzemelerle çizilebilmesi
- Mesh sınır küreleri ve kamera frustum culling
- Mesh + malzeme anahtarına göre instance batching
- Malzemeye göre sıralanmış çizim gruplarıyla daha az bind-group değişimi

Bir geometri GPU'ya bir kez, tekilleştirilmiş bir doku GPU'ya bir kez yüklenir. Aynı kaynakları kullanan yüzlerce varlık yalnızca dönüşüm ve renk instance verisi gönderir.

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
- `tgame-grafik`: 2B/3B GPU pipeline'ları, bağımsız GPU mesh/doku/malzeme kayıtları, instancing, batching, culling ve derinlik tamponu
- `tgame-matematik`: `Vektor2`, `Vektor3`, `Matris4` ve `Renk`
- `tgame-model`: Doğrulanmış mesh, sınır küresi, UV, malzeme, RGBA8 doku, glTF node ve sahne yükleme altyapısı
- `tgame-pencere`: İşletim sistemi penceresi, ham aygıt olayları ve imleç yakalama
- `tgame-sahne`: Sahne tanımları
- `tgame-mod`: Modlama sözleşmeleri ve mod kayıt sistemi
- `tgame-varlik`: Kimlikli varlıklar, kaynak kayıt defterleri, dönüşümler, kameralar ve dünya
- `tgame-zaman`: Kare süresi, toplam çalışma süresi ve kare sayacı

## glTF sahnesini doğrudan ekleme

```rust
let model = ModelVerisi::gltf_yukle("varliklar/fabrika.glb")?;
let varliklar = dunya.model_sahnesi_ekle(
    "Fabrika",
    model,
    Renk::BEYAZ,
)?;
```

`model_sahnesi_ekle`:

1. Modeldeki primitive mesh'lerini dünyaya kaydeder.
2. Doku ve malzemeleri bağımsız kayıt defterlerine ayırır.
3. Aynı doku veya malzeme zaten varsa mevcut kimliği tekrar kullanır.
4. Varsayılan glTF sahnesindeki kök node'lardan başlayarak ebeveyn × yerel matrisleri biriktirir.
5. Her mesh node örneğini, ortak geometri kimliğini paylaşan bir Tgame varlığına dönüştürür.

Kullanıcı dönüşümü, glTF'den gelen kaynak dünya matrisinin üzerinde uygulanır. Böylece bütün içe aktarılmış sahne sonradan taşınabilir, ölçeklenebilir veya döndürülebilir.

## Mesh'i farklı malzemelerle kullanma

```rust
let model = ModelVerisi::gltf_yukle("varliklar/kasa.glb")?;
let mesh = dunya.model_ekle(model)[0];

let turkuaz = dunya.malzeme_ekle(
    MalzemeVerisi::yeni(Renk::yeni(0.08, 0.85, 1.0, 1.0)),
);

dunya.varlik_ekle(Varlik::mesh(
    "Orijinal Dokulu Kasa",
    mesh,
    Renk::BEYAZ,
));

dunya.varlik_ekle(Varlik::mesh_malzemeli(
    "Turkuaz Kasa",
    mesh,
    turkuaz,
    Renk::BEYAZ,
));
```

`Varlik::mesh` modelin varsayılan malzemesini kullanır. `Varlik::mesh_malzemeli` aynı geometriyi başka bir `MalzemeKimligi` ile çizer. Geometri tekrar yüklenmez.

## Doku ve malzeme tekilleştirme

`Dunya::doku_ekle` aynı boyut, RGBA8 baytları ve sampler ayarlarına sahip dokuları tek kayıt olarak tutar.

`Dunya::malzeme_ekle` aynı temel renk ve aynı `DokuKimligi` bileşimini tek kayıt olarak tutar.

Başlangıç raporu artık ayrı kaynak sayılarını gösterir. Güncel `ilk-oyun` stres sahnesi şu özeti üretir:

```text
... 401 varlık — 1 mesh — 2 malzeme — 1 doku ...
```

## Sınır küresi ve frustum culling

Her `MeshVerisi`, konumlarından otomatik hesaplanan yerel bir `SinirKuresi` taşır.

Çizim öncesinde:

1. Varlığın kullanıcı dönüşümü ile glTF kaynak matrisi birleştirilir.
2. Yerel sınır küresi dünya uzayına taşınır.
3. Kamera yakın/uzak düzlemleri ile yatay/dikey görüş sınırları sınanır.
4. Görünmeyen varlık instance tamponuna hiç yazılmaz.

Özel durumlarda `Varlik::her_zaman_ciz()` culling'i atlar.

## GPU batching

Çizim anahtarı artık yalnızca mesh değildir:

```text
CizimAnahtari = MalzemeKimligi + MeshKimligi
```

Gruplar önce malzemeye, sonra mesh'e göre sıralanır. Böylece aynı malzemeyi paylaşan ardışık mesh gruplarında material bind group yeniden bağlanmaz.

Yerleşik küpler beyaz fallback doku kullanır. Kayıtlı dokulu ve dokusuz mesh'ler aynı shader/pipeline yolunu paylaşır.

## glTF yükleyicisinin doğruladıkları

Yükleyici:

- Yalnızca üçgen primitive'leri kabul eder.
- İndeks yoksa sıralı indeks üretir.
- Normal yoksa üçgenlerden yumuşatılmış normal hesaplar.
- `TEXCOORD_0` yoksa sıfır UV üretir.
- Sonlu olmayan konum, normal, UV, renk ve node matrisi değerlerini reddeder.
- 8 bit gri, gri-alfa, RGB ve RGBA resimleri RGBA8'e çevirir.
- `baseColorFactor`, `baseColorTexture`, filtre ve sarma ayarlarını içe aktarır.
- Her mesh için muhafazakâr sınır küresi hesaplar.

## Örnekleri çalıştırma

### 3B fizik, kaynak kayıtları ve culling stres sahnesi

```powershell
cargo run -p ilk-oyun
```

Sahne şunları içerir:

- Dokulu glTF piramitler
- Aynı mesh'i kullanan alternatif turkuaz malzemeli piramitler
- Tek mesh ve tek dokuyu paylaşan görünür instance grupları
- Kamera menzilinin çok dışında yer alan 256 ek piramit
- Frustum dışında oldukları için GPU instance tamponuna yazılmayan stres varlıkları
- Fizik, zıplama, sütunlar ve çarpışmalı zemin

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

Eski `Dunya::yeni()`, `Donusum2B`, `Kamera2B` ve üçgen instancing hattı korunur.

## Güncel sınırlar

- Yalnızca `TEXCOORD_0` ve taban renk dokusu çizilir.
- Metalik/pürüzlülük, normal, emissive ve occlusion haritaları henüz kullanılmaz.
- Mipmap zinciri ve anisotropic filtering henüz yoktur.
- Mesh kaynakları henüz içerik hash'iyle tekilleştirilmez; doku ve malzemeler tekilleştirilir.
- Alfa modu, çift taraflılık ve alpha cutoff uygulanmaz.
- Animasyon, iskelet, skinning ve morph target desteği henüz yoktur.
- Kaynak silme, sıcak yenileme ve GPU kaynak boşaltma henüz yoktur.

## Kalite denetimi

```powershell
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
```

Aynı denetimler her gönderimde GitHub Actions tarafından Windows üzerinde otomatik çalıştırılır. Workspace genelinde `unsafe_code = "forbid"`, `warnings = "deny"` ve pedantik Clippy kuralları aktiftir.

> Vira bismillah. Her büyük güncelleme motoru daha geniş oyun dünyalarına taşıyacak.
