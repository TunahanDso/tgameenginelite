# İlk Oyun: 3B Fizik Dünyası

Bu örnek, Tgame Engine Lite'ın Türkçe API'siyle perspektif kameralı, derinlik tamponlu, sabit fizik adımlı ve oynanabilir bir 3B dünya oluşturur.

## Sahne

- 121 basık küpten oluşan dama desenli zemin
- Farklı yükseklik ve renklerde altı çarpışmalı sütun
- Yerçekimine tabi sarı dinamik oyuncu küpü
- Statik çarpışma gövdesine sahip dönen büyük kırmızı merkez küpü
- Oyuncuyu fareyle kontrol edilen yörüngeden takip eden `Kamera3B`
- Bütün küplerin paylaştığı indeksli ortak mesh
- Model matrisi ve rengi instance verisinde taşıyan toplu GPU çizimi
- `Depth32Float` derinlik tamponu ve temel yönsel aydınlatma

## Fizik

- Simülasyon yaklaşık 60 Hz sabit adımla çalışır.
- Gerçek render süresi birikerek gereken fizik alt adımlarına çevrilir.
- Uzun kareler 250 ms ile, tek karedeki fizik adımları sekiz ile sınırlandırılır.
- Statik ve dinamik küpler `Aabb3` hacimleriyle çarpışır.
- X, Y ve Z hareketleri ayrı çözüldüğü için oyuncu duvarların boyunca kayabilir.
- Oyuncu yalnızca destekleyen bir yüzey üzerindeyken zıplayabilir.

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

Beklenen görüntü koyu arka plan üzerinde hacimli bir zemin, renkli sütunlar, dönen merkez küpü ve hareketli sarı oyuncudur. Oyuncu başlangıçta zemine düşer, engellere çarpar ve `Boşluk` ile zıplar.
