# İlk Oyun: 3B Fizik ve glTF Mesh Dünyası

Bu örnek, Tgame Engine Lite'ın Türkçe API'siyle perspektif kameralı, derinlik tamponlu, sabit fizik adımlı ve haricî glTF mesh'leri GPU'da çizen oynanabilir bir 3B dünya oluşturur.

## Sahne

- 121 basık küpten oluşan dama desenli zemin
- Farklı yükseklik ve renklerde altı çarpışmalı sütun
- Yerçekimine tabi sarı dinamik oyuncu küpü
- Statik çarpışma gövdesine sahip dönen büyük kırmızı merkez küpü
- `assets/piramit.gltf` dosyasından yüklenen sekiz renkli piramit
- Piramitlerin paylaştığı tek `MeshKimligi` ve tek GPU mesh kaydı
- Oyuncuyu fareyle kontrol edilen yörüngeden takip eden `Kamera3B`
- Mesh kimliğine göre instance batching
- `Depth32Float` derinlik tamponu ve temel yönsel aydınlatma

Yerleşik küpler `u16`, glTF piramit `u32` indeks kullanır. İki mesh türü aynı genel 3B pipeline içinde ayrı instance gruplarıyla çizilir.

## Fizik

- Simülasyon yaklaşık 60 Hz sabit adımla çalışır.
- Gerçek render süresi birikerek gereken fizik alt adımlarına çevrilir.
- Uzun kareler 250 ms ile, tek karedeki fizik adımları sekiz ile sınırlandırılır.
- Statik ve dinamik küpler `Aabb3` hacimleriyle çarpışır.
- X, Y ve Z hareketleri ayrı çözüldüğü için oyuncu duvarların boyunca kayabilir.
- Oyuncu yalnızca destekleyen bir yüzey üzerindeyken zıplayabilir.
- Piramitlerin görsel mesh'i ile basit statik AABB çarpışma varlığı birbirinden ayrıdır.

## Model yükleme

Piramit glTF dosyası normal verisi taşımaz. `tgame-model`, üçgen indekslerinden tepe normallerini hesaplar; dünya mesh kayıt defteri modeli kalıcı `MeshKimligi` ile saklar. Grafik katmanı bu kaynağı ilk kullanımda GPU tamponlarına yükler ve sekiz piramidi tek `draw_indexed` grubunda sunar.

CI testi dosyayı gerçek yolundan açar ve şu değerleri doğrular:

- 5 tepe
- 5 hesaplanmış normal
- 18 indeks

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

Beklenen görüntü koyu arka plan üzerinde hacimli zemin, renkli sütunlar, küpler ve dönen glTF piramitleridir. Oyuncu başlangıçta zemine düşer, engellere çarpar ve `Boşluk` ile zıplar.
