# İlk Oyun: 3B Dünya

Bu örnek, Tgame Engine Lite'ın Türkçe API'siyle perspektif kameralı, derinlik tamponlu ve oynanabilir bir 3B dünya oluşturur.

## Sahne

- 121 basık küpten oluşan dama desenli zemin
- Farklı yükseklik ve renklerde altı sütun
- WASD ile hareket eden sarı oyuncu küpü
- Sürekli dönen büyük kırmızı merkez küpü
- Oyuncuyu yörüngeden takip eden `Kamera3B`
- Bütün küplerin paylaştığı indeksli ortak mesh
- Model matrisi ve rengi instance verisinde taşıyan toplu GPU çizimi
- `Depth32Float` derinlik tamponu
- Yüzey normallerine dayalı temel yönsel aydınlatma

## Kontroller

- `WASD`: oyuncuyu X-Z düzleminde hareket ettirir
- Sol/sağ yön tuşları: kamerayı oyuncunun çevresinde döndürür
- Yukarı/aşağı yön tuşları: kamera yüksekliğini değiştirir
- `Boşluk`: oyuncunun 3B konumunu, kare sayısını ve toplam süreyi yazdırır
- `Escape`: oyunu kontrollü kapatır

Hareket kare süresinden bağımsızdır. Çapraz yönde hız artmaması için hareket vektörü birim uzunluğa getirilir. Kamera her karede oyuncunun güncel konumuna bakar.

## Çalıştırma

```powershell
cargo run -p ilk-oyun
```

Beklenen görüntü koyu arka plan üzerinde hacimli bir zemin, renkli sütunlar, dönen merkez küpü ve hareketli sarı oyuncudur. Yakındaki yüzeyler uzaktakileri derinlik testi sayesinde doğru biçimde örter.
