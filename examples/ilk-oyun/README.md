# İlk Oyun Örneği

Bu örnek, Tgame Engine Lite'ın Türkçe API'siyle gerçek bir 800×600 pencere ve oynanabilir bir 2B dünya oluşturur.

Sahnede:

- Sarı bir oyuncu üçgeni
- Sekiz renkli dekor üçgeni
- Oyuncuyu takip eden bir `Kamera2B`
- Konum, ölçek, dönüş ve renk taşıyan kimlikli varlıklar
- Bütün üçgenleri tek çizim çağrısında sunan GPU instancing sistemi bulunur

## Kontroller

- `WASD` veya yön tuşları: oyuncuyu hareket ettirir
- Hareket sırasında oyuncu döner
- `Boşluk`: oyuncu konumunu, kare sayısını ve toplam süreyi konsola yazar
- `Escape`: oyunu kontrollü biçimde kapatır

Hareket kare süresinden bağımsızdır ve çapraz yönde hız artmaması için yön vektörü birim uzunluğa getirilir.

## Çalıştırma

```powershell
cargo run -p ilk-oyun
```

Beklenen görüntü koyu arka plan üzerinde sarı oyuncu ve farklı konum, ölçek, dönüş ve renklere sahip sekiz dekor üçgenidir. Oyuncu hareket ettikçe kamera onunla birlikte ilerler.
