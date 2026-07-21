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

Model matrisi:

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

- `GovdeTuru::Statik`: hareket etmeyen engel
- `GovdeTuru::Dinamik`: hız, yerçekimi ve çarpışma çözümüne katılan gövde
- `Aabb3`: merkez ve pozitif yarı boyut

Dinamik hareket X, Y ve Z eksenlerinde ayrı uygulanıp çözülür. Bu yaklaşım oyuncunun duvara çarptığında diğer eksen boyunca kaymasını sağlar. Negatif Y yönündeki çözüm, gövdenin zeminde olduğunu işaretler ve zıplama yalnızca bu durumda kabul edilir.

AABB'ler varlık dönüşünü takip eder ancak Euler dönüşünü hacme uygulamaz. Karmaşık veya dönen görseller için fizik hacmi ayrı, görünmez bir varlıkta tutulabilir.

## Ham fare ve imleç

Pencere katmanı ham `DeviceEvent::MouseMotion` hareketini toplar ve işletim sistemi türlerini oyun API'sine sızdırmadan `FareHareketi` olarak `Girdi`ye aktarır.

- Pencere odaklandığında imleç önce `Locked`, desteklenmezse `Confined` modunda yakalanır.
- İmleç oyun sırasında gizlenir.
- Odak kaybolduğunda yakalama kaldırılır ve imleç gösterilir.
- Göreli hareket kare boyunca birikir ve kare sonunda sıfırlanır.

## 3B varlık ve mesh kimlikleri

Bir 3B varlık ortak olarak şunları taşır:

- `Donusum3B`: konum, Euler dönüşü ve ölçek
- `VarlikKimligi`: oyun nesnesi kimliği
- `Gorunum3B`: çizim kaynağı ve temel RGBA renk

`Gorunum3B` iki kaynak türünü destekler:

- `Gorunum3B::Kup`: motorun yerleşik küp mesh'i
- `Gorunum3B::Mesh`: dünyadaki bir `MeshKimligi`

`VarlikKimligi` ile `MeshKimligi` bilinçli biçimde ayrıdır. Bir mesh kimliği çok sayıda varlık tarafından paylaşılabilir; her varlığın dönüşümü ve rengi ayrı kalır.

## CPU mesh kayıt defteri

`Dunya`, değişmez ekleme sırasına sahip `Vec<MeshVerisi>` kayıt defteri taşır.

- `Dunya::mesh_ekle`: tek mesh kaydeder.
- `Dunya::model_ekle`: modeldeki bütün mesh'leri kaydeder.
- `Dunya::mesh`: kimlikle CPU mesh verisini döndürür.
- `Dunya::meshler`: bütün kayıtları eklenme sırasıyla döndürür.

Kimlik, vektördeki sabit sıra numarasıdır. Kaynak silme ve kimlik yeniden kullanımı henüz yoktur. Bu karar kimlikleri kararlı tutar ve CPU–GPU kayıtlarının aynı sırayla eşitlenmesini kolaylaştırır.

## Genel model verisi

`tgame-model`, grafik aygıtından bağımsız CPU mesh verisi üretir:

- `MeshVerisi`: konumlar, normaller ve `u32` indeksler
- `ModelVerisi`: bir veya daha fazla mesh
- `ModelVerisi::gltf_yukle`: `.gltf` ve `.glb` dosya yükleme

Yükleyici yalnızca üçgen primitive'leri kabul eder. İndeks bulunmazsa sıralı indeks üretir; normal bulunmazsa üçgen yüzlerinden yumuşatılmış tepe normalleri hesaplar.

GPU'ya geçmeden önce şu koşullar zorunludur:

- Mesh en az bir tepe içerir.
- Konum ve normal sayıları eşittir.
- İndeks listesi boş değildir ve üçün katıdır.
- Bütün indeksler tepe sınırları içindedir.

## GPU mesh kaydı

`UcBoyutGrafik`, dünya kayıt defterinin GPU karşılığını aynı sıra ile tutar.

- Yeni dünya mesh'leri ilk görüldükleri karede GPU'ya yüklenir.
- Konum ve normal, tepede art arda altı `f32` olarak saklanır.
- Kayıtlı mesh indeksleri `u32` ve `wgpu::IndexFormat::Uint32` kullanır.
- Yerleşik küp mevcut küçük `u16` indeks düzenini korur.
- CPU kayıt defterine yeni mesh eklendiğinde yalnızca eksik son kayıtlar GPU'ya aktarılır.

Bir mesh kaynağı için tepe ve indeks tamponları bir kez oluşturulur. Aynı mesh'i kullanan her varlık için geometri tekrar gönderilmez.

## Instance verisi ve draw batching

Her etkin 3B varlık GPU'ya 80 bayt instance verisi gönderir:

- 64 bayt model matrisi
- 16 bayt temel RGBA renk

Kare hazırlığında varlıklar `MeshAnahtari` ile sıralanır:

- Yerleşik küp grubu
- Her `MeshKimligi` için ayrı kayıtlı mesh grubu

Bütün instance verileri tek dinamik instance tamponuna ardışık yazılır. Her grup bu tampon içindeki kendi `Range<u32>` aralığını taşır. Render geçişinde pipeline, kamera grubu ve instance tamponu bir kez bağlanır; grup değiştikçe yalnızca tepe/indeks tamponları ve indeks biçimi değiştirilir.

Her mesh grubu için tek çağrı yapılır:

```text
draw_indexed(mesh indeksleri, grup instance aralığı)
```

Bu nedenle aynı glTF mesh'ini kullanan yüzlerce varlık tek draw call ile çizilebilir. Farklı mesh sayısı draw call sayısının temel belirleyicisidir.

## Yerleşik küp mesh'i

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

Gölgelendirici tek sabit yönsel ışık kullanır. Dünya normalinin ışık yönüyle nokta çarpımı, ortam payıyla birleştirilerek temel yaygın aydınlatma üretir.

Küp ve glTF mesh'leri aynı konum/normal vertex sözleşmesini ve aynı shader'ı kullanır.

## Güncel glTF sınırları

- glTF düğüm hiyerarşisi ve düğüm dönüşümleri uygulanmaz.
- Primitive malzemeleri, UV ve dokular çizime aktarılmaz.
- Bir varlık, seçilen tek `MeshKimligi` ve temel renk taşır.
- İskelet animasyonu, morph target ve skinning yoktur.
- Kaynak silme, sıcak yenileme ve GPU mesh boşaltma henüz yoktur.

## Geriye dönük uyumluluk

2B çizici ayrı modülde korunur. `ikiboyut-oyun` paketi workspace'e dahildir ve her kalite koşusunda derlenir. Yerleşik `Varlik::kup` API'si genel mesh sistemi içinde korunur.

## Sonraki 3B aşamalar

1. UV, sampler, doku ve malzeme kayıt defteri
2. glTF düğüm hiyerarşisi ve yerel/dünya dönüşümleri
3. Frustum culling ve görünür instance grupları
4. Hareketli–hareketli çarpışma ve geniş faz hızlandırması
5. Kapsül oyuncu çarpışması, basamak ve eğimli yüzeyler
6. Dünya ışıkları, gölge haritası ve normal matrisi
7. Animasyon, iskelet ve skinning
8. Kaynak sıcak yenileme ve yaşam döngüsü yönetimi
