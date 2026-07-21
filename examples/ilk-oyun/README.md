# İlk Oyun: 3B Fizik ve Dokulu glTF Dünyası

Bu örnek, Tgame Engine Lite'ın Türkçe API'siyle perspektif kameralı, derinlik tamponlu, sabit fizik adımlı ve haricî glTF mesh'lerini gerçek dokularıyla GPU'da çizen oynanabilir bir 3B dünya oluşturur.

## Sahne

- 121 basık küpten oluşan dama desenli zemin
- Farklı yükseklik ve renklerde altı çarpışmalı sütun
- Yerçekimine tabi sarı dinamik oyuncu küpü
- Statik çarpışma gövdesine sahip dönen büyük kırmızı merkez küpü
- `assets/piramit.gltf` dosyasından yüklenen sekiz dokulu piramit
- Piramitlerin paylaştığı tek `MeshKimligi`, GPU mesh'i, sRGB doku ve sampler
- Oyuncuyu fareyle kontrol edilen yörüngeden takip eden `Kamera3B`
- Mesh kimliğine göre instance batching
- `Depth32Float` derinlik tamponu ve temel yönsel aydınlatma

Yerleşik küpler `u16`, glTF piramit `u32` indeks kullanır. Küpler otomatik 1×1 beyaz doku, piramitler glTF içindeki 2×2 renk dokusunu kullanır. İki kaynak aynı dokulu 3B shader ve pipeline içinde ayrı instance gruplarıyla çizilir.

## Fizik

- Simülasyon yaklaşık 60 Hz sabit adımla çalışır.
- Gerçek render süresi birikerek gereken fizik alt adımlarına çevrilir.
- Uzun kareler 250 ms ile, tek karedeki fizik adımları sekiz ile sınırlandırılır.
- Statik ve dinamik küpler `Aabb3` hacimleriyle çarpışır.
- X, Y ve Z hareketleri ayrı çözüldüğü için oyuncu duvarların boyunca kayabilir.
- Oyuncu yalnızca destekleyen bir yüzey üzerindeyken zıplayabilir.
- Piramitlerin görsel mesh'i ile basit statik AABB çarpışma varlığı birbirinden ayrıdır.

## Model, UV ve malzeme yükleme

Piramit glTF dosyası normal verisi taşımaz; `tgame-model`, üçgen indekslerinden tepe normallerini hesaplar. Dosya ayrıca şunları içerir:

- Beş adet `TEXCOORD_0` UV koordinatı
- Gömülü 2×2 PNG taban renk dokusu
- Doğrusal büyütme/küçültme filtresi
- U ve V eksenlerinde tekrar sarma davranışı
- PBR `baseColorFactor`

Yükleyici resmi RGBA8 veriye dönüştürür. Grafik katmanı `Rgba8UnormSrgb` GPU dokusu, texture view, sampler ve material bind group oluşturur. Sekiz piramit aynı kaynakları paylaşır ve tek `draw_indexed` grubunda sunulur.

CI testi dosyayı gerçek yolundan açar ve şunları doğrular:

- 5 tepe
- 5 hesaplanmış normal
- 5 UV
- 18 indeks
- 2×2 boyutunda 16 bayt RGBA8 doku
- Doğrusal filtre ve tekrar sarma ayarları

## Kontroller

- Fare: kamerayı yatay ve dikey döndürür
- `WASD`: kamera yönüne göre oyuncuyu X-Z düzleminde hareket ettirir
- `Boşluk`: oyuncu zemindeyse zıplatır
- Yön tuşları: fareye alternatif kamera kontrolü
- `Enter`: oyuncunun konumunu, hızını, zeminde olma durumunu ve kareyi yazdırır
- `Escape`: oyunu kontrollü kapatır

Pencere odaklandığında imleç kilitlenir ve gizlenir; odak kaybolduğunda serbest bırakılır. Hareket yönü birim uzunluğa getirildiği için çapraz hareket hız kazandırmaz.

## Çalıştırma

```powershell
cargo run -p ilk-oyun
```

Beklenen görüntü koyu arka plan üzerinde hacimli zemin, renkli sütunlar, küpler ve dokulu dönen glTF piramitleridir. Oyuncu başlangıçta zemine düşer, engellere çarpar ve `Boşluk` ile zıplar.
