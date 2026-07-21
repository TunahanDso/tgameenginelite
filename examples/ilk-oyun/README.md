# İlk Oyun: 3B Kaynak Sistemi ve Culling Stres Sahnesi

Bu örnek, Tgame Engine Lite'ın Türkçe API'siyle fizik, dokulu glTF, bağımsız malzemeler, node hiyerarşisi, kaynak tekilleştirme, instance batching ve frustum culling kullanan oynanabilir bir 3B dünya oluşturur.

## Sahne

- 121 basık küpten oluşan dama desenli zemin
- Farklı yükseklik ve renklerde altı çarpışmalı sütun
- Yerçekimine tabi sarı dinamik oyuncu küpü
- Statik çarpışma gövdesine sahip dönen merkez küpü
- `assets/piramit.gltf` dosyasından yüklenen sekiz görünür piramit
- Aynı `MeshKimligi`ni kullanan dokulu ve turkuaz malzemeli piramitler
- Kamera görüşünün çok dışında bulunan 256 ek piramit
- Mesh + malzeme anahtarına göre instance batching
- Sınır küresi tabanlı frustum culling
- `Depth32Float` derinlik tamponu ve temel yönsel aydınlatma

256 stres varlığı dünyada gerçekten kayıtlıdır. Ancak kamera frustum'ının dışında oldukları için GPU instance tamponuna yazılmaz ve draw çağrılarına katılmaz.

## Fizik

- Simülasyon yaklaşık 60 Hz sabit adımla çalışır.
- Render süresi birikerek sabit fizik alt adımlarına çevrilir.
- Uzun kareler 250 ms ile, tek karedeki fizik adımları sekiz ile sınırlandırılır.
- Statik ve dinamik küpler `Aabb3` hacimleriyle çarpışır.
- X, Y ve Z hareketleri ayrı çözüldüğü için oyuncu duvarların boyunca kayabilir.
- Oyuncu yalnızca destekleyen bir yüzey üzerindeyken zıplayabilir.
- Piramit görselleri ile basit statik AABB çarpışma varlıkları ayrıdır.

## glTF geometri ve node hiyerarşisi

Piramit glTF dosyası normal verisi taşımaz. `tgame-model`, üçgen indekslerinden tepe normallerini hesaplar.

Dosyada ayrıca şunlar vardır:

- Beş `TEXCOORD_0` UV koordinatı
- Gömülü 2×2 PNG taban renk dokusu
- Doğrusal büyütme ve küçültme filtresi
- U/V tekrar sarma davranışı
- PBR `baseColorFactor`
- Ötelenmiş kök node
- Ötelenmiş, döndürülmüş ve 0.8 ölçekli çocuk mesh node'u

Yükleyici ebeveyn ve çocuk matrislerini biriktirir. Testte mesh yerel orijininin dünya konumu `(1.25, 1.5, -0.75)` ve birikmiş en büyük ölçeğin `0.8` olduğu doğrulanır.

## Bağımsız kaynak kayıtları

Model dünyaya eklenirken:

- geometri `MeshKimligi`
- glTF malzemesi `MalzemeKimligi`
- gömülü PNG `DokuKimligi`

olarak ayrı kayıt edilir.

Eşit doku ve malzemeler tekrar eklenirse mevcut kimlikleri kullanılır. GPU tarafında texture, sampler, material bind group ve mesh tamponları da ayrı yaşam döngülerine sahiptir.

## Aynı mesh, farklı malzeme

Görünür piramitlerin bir bölümü glTF'nin dokulu varsayılan malzemesini kullanır. Her üçüncü piramit ise bağımsız turkuaz malzeme ile çizilir:

```rust
Varlik::mesh_malzemeli(
    "Alternatif Malzemeli Piramit",
    piramit_mesh,
    turkuaz_malzeme,
    renk,
)
```

Bu işlem geometriyi çoğaltmaz. Aynı vertex ve indeks tamponları farklı material bind group'larla kullanılır.

## Frustum culling

Her mesh otomatik hesaplanan bir `SinirKuresi` taşır. Çizim hazırlığında küre varlığın nihai model matrisiyle dünya uzayına dönüştürülür.

Kamera:

- yakın düzlem
- uzak düzlem
- yatay görüş sınırı
- dikey görüş sınırı

üzerinden küreyi sınar. Görünmeyen varlıklar instance tamponuna ulaşmadan elenir.

## CI doğrulaması

Gerçek `assets/piramit.gltf` dosyası diskten açılır ve şunlar doğrulanır:

- 5 tepe
- 5 hesaplanmış normal
- 5 UV
- 18 indeks
- geçerli mesh sınır küresi
- 2×2 boyutunda 16 bayt RGBA8 doku
- doğrusal filtre ve tekrar sarma ayarları
- 1 sahne örneği
- doğru ebeveyn–çocuk dünya matrisi
- `Dunya::model_sahnesi_ekle` sonucu 1 mesh, 1 malzeme ve 1 doku kaydı

Ayrıca dünya katmanı testleri eşit doku/malzeme tekilleştirmesini ve kamera dışındaki kürenin elenmesini sınar.

## Kontroller

- Fare: kamerayı yatay ve dikey döndürür
- `WASD`: kamera yönüne göre oyuncuyu X-Z düzleminde hareket ettirir
- `Boşluk`: oyuncu zemindeyse zıplatır
- Yön tuşları: fareye alternatif kamera kontrolü
- `Enter`: oyuncunun konumunu, hızını, zeminde olma durumunu ve kareyi yazdırır
- `Escape`: oyunu kontrollü kapatır

Pencere odaklandığında imleç kilitlenir ve gizlenir; odak kaybolduğunda serbest bırakılır. Çapraz hareket birimlenir ve hız kazandırmaz.

## Çalıştırma

```powershell
cargo run -p ilk-oyun
```

Beklenen görüntü koyu arka plan üzerinde zemin, sütunlar, küpler, dokulu piramitler ve turkuaz alternatif malzemeli piramitlerdir. Uzakta oluşturulan 256 stres piramidi kamera dışında olduğu için görünmez ve çizim yükü oluşturmaz.
