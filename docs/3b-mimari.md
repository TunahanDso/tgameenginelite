# Tgame Engine Lite 3B Mimari Sözleşmesi

Bu belge, motorun üç boyutlu matematik, varlık, fizik, model, malzeme ve GPU çizim katmanlarında uyulacak temel sözleşmeleri tanımlar.

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
- `Gorunum3B`: çizim kaynağı ve varlık renk çarpanı

`Gorunum3B` iki kaynak türünü destekler:

- `Gorunum3B::Kup`: motorun yerleşik küp mesh'i
- `Gorunum3B::Mesh`: dünyadaki bir `MeshKimligi`

`VarlikKimligi` ile `MeshKimligi` bilinçli biçimde ayrıdır. Bir mesh kimliği çok sayıda varlık tarafından paylaşılabilir; her varlığın dönüşümü ve renk çarpanı ayrı kalır.

## CPU mesh kayıt defteri

`Dunya`, değişmez ekleme sırasına sahip `Vec<MeshVerisi>` kayıt defteri taşır.

- `Dunya::mesh_ekle`: tek mesh kaydeder.
- `Dunya::model_ekle`: modeldeki bütün mesh'leri kaydeder.
- `Dunya::mesh`: kimlikle CPU mesh verisini döndürür.
- `Dunya::meshler`: bütün kayıtları eklenme sırasıyla döndürür.

Kimlik, vektördeki sabit sıra numarasıdır. Kaynak silme ve kimlik yeniden kullanımı henüz yoktur. Bu karar kimlikleri kararlı tutar ve CPU–GPU kayıtlarının aynı sırayla eşitlenmesini kolaylaştırır.

## Genel model, UV ve malzeme verisi

`tgame-model`, grafik aygıtından bağımsız kaynak verisi üretir:

- `MeshVerisi`: konumlar, normaller, `Vektor2` UV'ler, `u32` indeksler ve içe aktarılmış malzeme
- `MalzemeVerisi`: taban renk çarpanı ve isteğe bağlı taban renk dokusu
- `DokuVerisi`: doğrulanmış genişlik, yükseklik, RGBA8 baytları ve sampler ayarları
- `OrnekleyiciVerisi`: büyütme/küçültme filtresi ve U/V sarma davranışı
- `ModelVerisi`: bir veya daha fazla mesh
- `ModelVerisi::gltf_yukle`: `.gltf` ve `.glb` dosya yükleme

Yükleyici yalnızca üçgen primitive'leri kabul eder. İndeks bulunmazsa sıralı indeks üretir; normal bulunmazsa üçgen yüzlerinden yumuşatılmış tepe normalleri hesaplar. `TEXCOORD_0` bulunmazsa her tepeye sıfır UV atanır.

GPU'ya geçmeden önce şu koşullar zorunludur:

- Mesh en az bir tepe içerir.
- Konum, normal ve UV sayıları eşittir.
- İndeks listesi boş değildir ve üçün katıdır.
- Bütün indeksler tepe sınırları içindedir.
- Doku genişliği ve yüksekliği sıfır değildir.
- RGBA8 bayt sayısı `genişlik × yükseklik × 4` değeridir.

## glTF malzeme içe aktarma

İlk malzeme aşamasında PBR metallic-roughness malzemesinden şunlar alınır:

- `baseColorFactor`
- `baseColorTexture`
- Taban renk dokusunun `texCoord` alanı; yalnızca `TEXCOORD_0` kabul edilir
- `magFilter` ve `minFilter`
- `wrapS` ve `wrapT`

Sampler dönüşümleri:

- `NEAREST` → `DokuFiltresi::EnYakin`
- `LINEAR` → `DokuFiltresi::Dogrusal`
- `REPEAT` → `DokuSarmasi::Tekrarla`
- `MIRRORED_REPEAT` → `DokuSarmasi::AynalayarakTekrarla`
- `CLAMP_TO_EDGE` → `DokuSarmasi::KenaraSabitle`

İçe aktarılan 8 bit resimler RGBA8'e dönüştürülür:

- R8 → gri RGB + tam alfa
- R8G8 → gri RGB + ikinci kanal alfa
- R8G8B8 → RGB + tam alfa
- R8G8B8A8 → doğrudan RGBA

16 bit ve kayan noktalı resim biçimleri bu aşamada Türkçe `OyunHatasi` ile reddedilir.

## GPU mesh kaydı

`UcBoyutGrafik`, dünya kayıt defterinin GPU karşılığını aynı sıra ile tutar.

- Yeni dünya mesh'leri ilk görüldükleri karede GPU'ya yüklenir.
- Kayıtlı mesh indeksleri `u32` ve `wgpu::IndexFormat::Uint32` kullanır.
- Yerleşik küp mevcut küçük `u16` indeks düzenini korur.
- CPU kayıt defterine yeni mesh eklendiğinde yalnızca eksik son kayıtlar GPU'ya aktarılır.

Her tepe 32 bayt taşır:

```text
konum.xyz   12 bayt   location 0
normal.xyz  12 bayt   location 1
uv.xy        8 bayt   location 7
```

Bir mesh kaynağı için tepe ve indeks tamponları bir kez oluşturulur. Aynı mesh'i kullanan her varlık için geometri tekrar gönderilmez.

## GPU malzeme kaydı

Her `GpuMesh`, mevcut aşamada kendi `GpuMalzeme` kaydını taşır. GPU malzemesi şu kaynakların ömrünü birlikte yönetir:

- `Rgba8UnormSrgb` taban renk dokusu
- `TextureView`
- WGPU `Sampler`
- Malzeme `BindGroup`

Dokusuz mesh'ler 1×1 beyaz RGBA8 doku kullanır. Bu sayede shader'da koşullu dokulu/dokusuz dal bulunmaz; bütün mesh'ler aynı texture-sampling yolunu kullanır.

Pipeline bind group sözleşmesi:

```text
group 0: kamera uniform tamponu
group 1 binding 0: taban renk texture view
group 1 binding 1: filtering sampler
```

GPU dokuları tek mip seviyesine sahiptir. Sampler'ın mipmap filtresi eşlenir ancak bu aşamada mipmap zinciri üretilmez.

## Instance verisi ve draw batching

Her etkin 3B varlık GPU'ya 80 bayt instance verisi gönderir:

- 64 bayt model matrisi
- 16 bayt renk çarpanı

Kayıtlı glTF mesh'lerinde instance rengi, malzemenin `baseColorFactor` değeriyle CPU tarafında çarpılır. Fragment shader bu sonucu sRGB taban renk dokusundan örneklenen renkle tekrar çarpar.

Kare hazırlığında varlıklar `MeshAnahtari` ile sıralanır:

- Yerleşik küp grubu
- Her `MeshKimligi` için ayrı kayıtlı mesh grubu

Bütün instance verileri tek dinamik instance tamponuna ardışık yazılır. Her grup bu tampon içindeki kendi `Range<u32>` aralığını taşır. Render geçişinde pipeline, kamera grubu ve instance tamponu bir kez bağlanır; grup değiştikçe malzeme bind group'u, tepe/indeks tamponları ve indeks biçimi değiştirilir.

Her mesh grubu için tek çağrı yapılır:

```text
draw_indexed(mesh indeksleri, grup instance aralığı)
```

Bu nedenle aynı dokulu glTF mesh'ini kullanan yüzlerce varlık tek draw call ile çizilebilir. Farklı mesh sayısı draw call sayısının temel belirleyicisidir.

## Yerleşik küp mesh'i

Küp mesh'i:

- 24 tepe
- Her tepede konum, yüzey normali ve yüz UV'si
- 36 adet `u16` indeks
- Üçgen listesi topolojisi
- Otomatik 1×1 beyaz malzeme dokusu

Her yüzün ayrı normal ve UV taşıması için köşe konumları yüzler arasında paylaşılmaz. Bu, keskin küp kenarlarında doğru temel aydınlatma ve yüz başına tam UV alanı sağlar.

## Shader ve aydınlatma

Vertex shader model matrisiyle dünya konumunu ve normalini üretir; UV'yi fragment aşamasına aktarır.

Fragment shader:

1. `textureSample` ile taban renk dokusunu örnekler.
2. Doku rengini instance/malzeme renk çarpanıyla birleştirir.
3. Sabit yönsel ışık ve ortam payıyla temel yaygın aydınlatma uygular.

Bu aşama tam PBR değildir; ancak geometri, UV, doku, sampler ve renk çarpanı ayrımı sonraki PBR kaynakları için temel oluşturur.

## Derinlik

3B pipeline `Depth32Float` derinlik dokusu kullanır. Her karede derinlik 1.0 değerine temizlenir; daha yakın parçalar `Less` karşılaştırmasıyla görünür olur.

Pencere yeniden boyutlandırıldığında yüzey yapılandırmasıyla birlikte derinlik dokusu da yeni boyutta yeniden oluşturulur.

## Güncel sınırlar

- glTF düğüm hiyerarşisi ve düğüm dönüşümleri uygulanmaz.
- Yalnızca `TEXCOORD_0` desteklenir.
- Metalik/pürüzlülük, normal, emissive ve occlusion dokuları çizilmez.
- Mipmap zinciri oluşturulmaz.
- Malzeme mesh kaydının parçasıdır; bağımsız `MalzemeKimligi` ve aynı geometriyi farklı malzemelerle paylaşma henüz yoktur.
- Alfa modu, çift taraflılık ve alpha cutoff henüz uygulanmaz.
- İskelet animasyonu, morph target ve skinning yoktur.
- Kaynak silme, sıcak yenileme ve GPU kaynak boşaltma henüz yoktur.

## Geriye dönük uyumluluk

2B çizici ayrı modülde korunur. `ikiboyut-oyun` paketi workspace'e dahildir ve her kalite koşusunda derlenir. Yerleşik `Varlik::kup` API'si genel dokulu mesh sistemi içinde korunur.

## Sonraki 3B aşamalar

1. Bağımsız `MalzemeKimligi`, doku tekrar kullanımı ve malzeme bazlı batching
2. Mipmap üretimi ve anisotropic filtering
3. glTF düğüm hiyerarşisi ve yerel/dünya dönüşümleri
4. Frustum culling ve görünür instance grupları
5. Metalik/pürüzlülük, normal ve emissive haritalarıyla PBR
6. Dünya ışıkları ve gölge haritası
7. Hareketli–hareketli çarpışma ve geniş faz hızlandırması
8. Kapsül oyuncu çarpışması, basamak ve eğimli yüzeyler
9. Animasyon, iskelet ve skinning
10. Kaynak sıcak yenileme ve yaşam döngüsü yönetimi
