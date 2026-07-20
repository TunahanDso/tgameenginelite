# Tgame Engine Lite 3B Mimari Sözleşmesi

Bu belge, motorun üç boyutlu matematik, varlık ve GPU çizim katmanlarında uyulacak temel sözleşmeleri tanımlar.

## Koordinat sistemi

Tgame Engine Lite sağ elli bir dünya koordinat sistemi kullanır:

- Pozitif X: sağ
- Pozitif Y: yukarı
- Negatif Z: ileri

`Vektor3::ILERI`, `(0, 0, -1)` değeridir. 3B hareket ve kamera kodu bu sözleşmeye göre yazılır.

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

2B çizici ayrı modülde korunur. `ikiboyut-oyun` paketi workspace'e dahildir ve her kalite koşusunda derlenir. 3B geliştirmeleri eski Türkçe 2B API'yi bozamaz.

## Sonraki 3B aşamalar

1. Sabit zaman adımı ve 3B AABB çarpışma
2. Serbest kamera ve fare girdisi
3. Genel `Mesh` soyutlaması
4. OBJ veya glTF model yükleme
5. Doku ve malzeme sistemi
6. Frustum culling ve görünürlük kümeleri
7. Işık bileşenleri ve gölge haritası
