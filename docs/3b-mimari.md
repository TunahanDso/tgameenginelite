# Tgame Engine Lite 3B Mimari Sözleşmesi

Bu belge, motorun üç boyutlu matematik, varlık, fizik, model ve GPU çizim katmanlarında uyulacak temel sözleşmeleri tanımlar.

## Koordinat sistemi

Tgame Engine Lite sağ elli bir dünya koordinat sistemi kullanır:

- Pozitif X: sağ
- Pozitif Y: yukarı
- Negatif Z: ileri

`Vektor3::ILERI`, `(0, 0, -1)` değeridir. 3B hareket, fizik ve kamera kodu bu sözleşmeye göre yazılır.

## Matris düzeni

`Matris4`, GPU ile uyumlu sütun öncelikli 16 adet `f32` taşır.

Model matrisi şu sırayla oluşturulur:

```text
Öteleme × Z dönüşü × Y dönüşü × X dönüşü × Ölçek
```

Kamera matrisi:

```text
Perspektif izdüşüm × sağ elli görünüm
```

Perspektif matrisi WGPU'nun 0–1 derinlik aralığına göre üretilir.

## Dünya seçimi

- `Dunya::yeni()`: `DunyaBoyutu::IkiBoyut`
- `Dunya::yeni_3b()`: `DunyaBoyutu::UcBoyut`

Ana grafik çekirdeği pencere, yüzey, aygıt ve kuyruğu ortak yönetir. Kare çiziminde dünyanın boyutuna bakarak `IkiBoyutGrafik` veya `UcBoyutGrafik` yolunu seçer.

## Sabit fizik adımı

`FizikDunyasi`, render döngüsünden bağımsız yaklaşık 60 Hz sabit adım kullanır.

- Render karesinde geçen süre bir birikimde tutulur.
- Birikim sabit adımı karşıladıkça fizik alt adımları çalıştırılır.
- Tek render karesi en fazla 250 ms fizik süresi ekleyebilir.
- Tek karede en fazla sekiz alt adım çalıştırılır.
- Sınır aşıldığında kalan birikim bırakılarak ölüm sarmalı engellenir.

Fizik sonucu doğrudan `Donusum3B::konum` alanına yazılır. Render, fizik tarafından tamamlanmış son dünya durumunu çizer.

## AABB çarpışma

İlk fizik aşaması eksenlere hizalı kutu hacimleri kullanır:

- `FizikGovdesi::Statik`: hareket etmeyen engel
- `FizikGovdesi::Dinamik`: hız, yerçekimi ve çarpışma çözümüne katılan gövde
- `Aabb3`: merkez ve pozitif yarı boyut

Dinamik hareket X, Y ve Z eksenlerinde ayrı uygulanıp çözülür. Bu yaklaşım oyuncunun duvara çarptığında diğer eksen boyunca kaymasını sağlar. Negatif Y yönündeki çözüm, gövdenin zeminde olduğunu işaretler ve zıplama yalnızca bu durumda kabul edilir.

İlk aşamadaki AABB'ler varlık dönüşünü takip eder ancak Euler dönüşünü hacme uygulamaz. Dönen görseller için çarpışma hacmi eksenlere hizalı yaklaşık kutu olarak kalır.

## Ham fare ve imleç

Pencere katmanı ham `DeviceEvent::MouseMotion` hareketini toplar ve işletim sistemi türlerini oyun API'sine sızdırmadan `FareHareketi` olarak `Girdi`ye aktarır.

- Pencere odaklandığında imleç önce `Locked`, desteklenmezse `Confined` modunda yakalanır.
- İmleç oyun sırasında gizlenir.
- Odak kaybolduğunda yakalama kaldırılır ve imleç gösterilir.
- Göreli hareket kare boyunca birikir ve kare sonunda sıfırlanır.

## 3B varlık verisi

Bir küp varlığı şunları taşır:

- `Donusum3B`: konum, Euler dönüşü ve ölçek
- `Gorunum3B::Kup`: temel RGBA renk
- Ortak `VarlikKimligi`

GPU'ya her küp için 80 bayt instance verisi gönderilir:

- 64 bayt model matrisi
- 16 bayt renk

Küpün tepe ve indeks verileri her varlık için tekrarlanmaz.

## Ortak küp mesh'i

Küp mesh'i:

- 24 tepe
- Her tepede konum ve yüzey normali
- 36 adet `u16` indeks
- Üçgen listesi topolojisi

Her yüzün ayrı normal taşıması için köşe konumları yüzler arasında paylaşılmaz. Bu, keskin küp kenarlarında doğru temel aydınlatma sağlar.

## Genel model verisi

`tgame-model`, grafik aygıtından bağımsız CPU mesh verisi üretir:

- `MeshVerisi`: konumlar, normaller ve `u32` indeksler
- `ModelVerisi`: bir veya daha fazla mesh
- `ModelVerisi::gltf_yukle`: `.gltf` ve `.glb` dosya yükleme

Yükleyici yalnızca üçgen primitive'leri kabul eder. İndeks bulunmazsa sıralı indeks üretir; normal bulunmazsa üçgen yüzlerinden yumuşatılmış tepe normalleri hesaplar. Bütün sınırlar ve indeksler GPU'ya geçmeden önce doğrulanır.

Bu katman henüz `UcBoyutGrafik` içinde genel mesh çizimine bağlanmamıştır. Sonraki grafik aşaması model mesh'lerini GPU kaynak kayıt defterine dönüştürüp mesh kimliğine göre toplu çizim yapacaktır.

## Derinlik

3B pipeline `Depth32Float` derinlik dokusu kullanır. Her karede derinlik 1.0 değerine temizlenir; daha yakın parçalar `Less` karşılaştırmasıyla görünür olur.

Pencere yeniden boyutlandırıldığında yüzey yapılandırmasıyla birlikte derinlik dokusu da yeni boyutta yeniden oluşturulur.

## Aydınlatma

İlk 3B aşamada gölgelendirici tek sabit yönsel ışık kullanır. Dünya normalinin ışık yönüyle nokta çarpımı, ortam payıyla birleştirilerek temel yaygın aydınlatma üretir.

Bu sistem geçicidir fakat normal, model matrisi ve mesh ayrımı gelecekte şu özelliklerin eklenmesine hazırdır:

- Dünya ışıkları
- Normal matrisi
- Malzemeler
- Dokular
- Gölge haritaları
- PBR

## Geriye dönük uyumluluk

2B çizici ayrı modülde korunur. `ikiboyut-oyun` paketi workspace'e dahildir ve her kalite koşusunda derlenir. 3B, fizik ve model geliştirmeleri eski Türkçe 2B API'yi bozamaz.

## Sonraki 3B aşamalar

1. Yüklenen glTF mesh'lerini GPU mesh kayıt defterine bağlama
2. Mesh kimliğine göre draw batching ve görünürlük
3. Hareketli–hareketli çarpışma ve geniş faz hızlandırması
4. Kapsül oyuncu çarpışması ve eğimli yüzeyler
5. Doku ve malzeme sistemi
6. Frustum culling ve görünürlük kümeleri
7. Işık bileşenleri ve gölge haritası
