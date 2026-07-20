# İlk Oyun Örneği

Bu örnek, Tgame Engine Lite'ın Türkçe kullanıcı API'sini kullanarak gerçek bir 800×600 pencere açar ve GPU üzerinde renkli bir üçgen çizer.

Ayrıca:

- Boşluk tuşuna ilk basıldığı kareyi ve toplam oyun süresini konsola yazar.
- Escape tuşuyla oyun döngüsünü kontrollü biçimde kapatır.
- Pencere yeniden boyutlandırıldığında GPU yüzeyini günceller.

Çalıştırmak için:

```powershell
cargo run -p ilk-oyun
```

Beklenen görüntü koyu arka plan üzerinde kırmızı, yeşil ve mavi köşelere sahip bir üçgendir.
