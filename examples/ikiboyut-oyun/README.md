# 2B Uyumluluk Örneği

Bu küçük örnek, motorun 3B grafik hattı eklendikten sonra eski 2B API'nin kırılmadığını sürekli doğrular.

Sahnede sarı bir oyuncu üçgeni ve mavi bir dekor üçgeni vardır. `Dunya::yeni()`, `Donusum2B`, `Kamera2B` ve 2B GPU instancing hattı kullanılır.

## Kontroller

- `WASD`: oyuncuyu hareket ettirir
- `Escape`: oyunu kontrollü kapatır

## Çalıştırma

```powershell
cargo run -p ikiboyut-oyun
```

Bu paket workspace kalite denetimine dahildir; böylece sonraki 3B güncellemeleri 2B derleme uyumluluğunu fark edilmeden bozamaz.
