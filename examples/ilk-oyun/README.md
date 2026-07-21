# İlk Oyun: Kayıp Mühür Macerası

Bu örnek, Tgame Engine Lite'ın grafik, fizik, model, kaynak yönetimi ve yeni `tgame-macera` katmanını aynı oynanabilir bölümde birleştirir.

Oyuncu Sisli Köy Meydanı'nda Gözcü Aras ile konuşur, **Kayıp Mühür** görevini kabul eder, dünyaya dağılmış üç mühür parçasını toplar, tapınak alanına girer, Taş Muhafız'ın mührünü çözer ve Kadim Mahzen'e geçer.

## Kullanılan motor sistemleri

- Perspektif 3B dünya ve derinlik tamponu
- Sabit yaklaşık 60 Hz fizik
- Dinamik oyuncu ve statik AABB çarpışmalar
- Fare kamerası, yürüme ve zıplama
- Dokulu glTF model yükleme
- Bağımsız mesh, malzeme ve doku kayıtları
- Aynı mesh'in farklı malzemelerle çizilmesi
- Instance batching ve frustum culling
- Kalıcı hikâye bayrakları, sayaçlar ve metinler
- Yığın sınırlı envanter ve korunan görev eşyası
- Üç adımlı görev zinciri
- Dallanan NPC diyaloğu
- Dünya konumlu etkileşim noktaları
- Koşullu alan tetikleyicisi
- Olay tabanlı görev ilerlemesi
- Görev tamamlanınca çalışan olay kuralı
- Güvenli sahne geçişi ve adlandırılmış giriş noktası
- Kontrol noktası
- F5/F9 kayıt ve yükleme
- Toplam oynama süresinin kaydedilmesi

## Hikâye akışı

1. Başlangıç noktasının yakınındaki mavi **Gözcü Aras** küpüne yaklaş ve `E` tuşuna bas.
2. `1` ile görevi kabul et; Sis Pusulası envantere eklenir ve görev başlar.
3. Yeşil, mavi ve sarı küçük mühür küplerini bulup `E` ile topla.
4. Üç parça tamamlanınca haritanın kuzeyindeki kırmızı Taş Muhafız'a yaklaş.
5. Tapınak alanına giriş ikinci görev adımını tamamlar.
6. Muhafızla etkileşim son hedefi tamamlar, kontrol noktası açar ve Kadim Mahzen'e sahne geçişi yapar.
7. Görev tamamlanınca oyuncunun kalıcı unvanı **Mührün Varisi** olur.

## Kontroller

- Fare: kamerayı yatay ve dikey döndürür
- `WASD`: kamera yönüne göre hareket
- Yön tuşları: fareye alternatif kamera kontrolü
- `Boşluk`: zemindeyken zıplama
- `E`: en yakın kullanılabilir etkileşim
- `1`, `2`, `3`: diyalog seçeneği
- `Enter`: seçeneksiz diyalog satırını ilerletme
- `Tab`: görev günlüğü, envanter, sahne ve oynama süresi
- `F5`: 0 numaralı yuvaya kaydetme
- `F9`: 0 numaralı yuvayı yükleme
- `R`: son kontrol noktasına dönme
- `Escape`: kontrollü kapanış

Diyalog açıkken oyuncu hareketi durur. Konuşma ve görev bilgileri mevcut yazı çizim katmanı tamamlanana kadar terminalde gösterilir.

## Grafik stres sahnesi

Macera içeriğinin yanında önceki teknik stres sahnesi korunur:

- 121 parçalı dama zemin
- Farklı yüksekliklerde çarpışmalı sütunlar
- Sekiz görünür dokulu/alternatif malzemeli glTF piramit
- Kamera görüşünün çok dışında 256 ek piramit
- Mesh + malzeme anahtarlı batching
- Sınır küresi tabanlı frustum culling

Uzak 256 piramit dünyada kayıtlıdır ancak görünür olmadıkları için GPU instance tamponuna yazılmaz.

## Otomatik bölüm testi

Örnek ikili kendi entegrasyon testini de taşır. Test:

- Gözcü diyaloğunu açar
- Görevi kabul eder
- Diyaloğu bitirir
- Üç mühür parçasını ekler
- Tapınak alanını tetikler
- Muhafız yenildi olayını yayınlar
- Görevin tamamlandığını
- Bölüm bayrağının açıldığını
- `Mührün Varisi` unvanının verildiğini

tek akışta doğrular.

## Çalıştırma

```powershell
cargo run -p ilk-oyun
```

Kayıt dosyaları çalışma klasöründeki `kayitlar/kayip-muhur` dizinine yazılır.
