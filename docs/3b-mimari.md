# Tgame Engine Lite 3B Mimari Sözleşmesi

Bu belge, motorun matematik, glTF sahne yükleme, dünya kaynakları, görünürlük, batching, fizik ve GPU çizim katmanlarında uyulacak güncel sözleşmeleri tanımlar.

## Koordinat ve matris sözleşmesi

Tgame Engine Lite sağ elli dünya koordinatı kullanır:

- Pozitif X: sağ
- Pozitif Y: yukarı
- Negatif Z: ileri

`Matris4`, GPU ile uyumlu sütun öncelikli 16 adet `f32` taşır.

Kullanıcı TRS modeli:

```text
Öteleme × Z dönüşü × Y dönüşü × X dönüşü × Ölçek
```

Bir glTF node örneğinin nihai varlık matrisi:

```text
Kullanıcı TRS matrisi × glTF birikmiş kaynak dünya matrisi
```

`Matris4::noktayi_donustur` homojen W bileşenini hesaba katar. `Matris4::en_buyuk_olcek`, sınır küresini eşit olmayan ölçek altında muhafazakâr biçimde büyütmek için ilk üç matris sütununun en büyük uzunluğunu döndürür.

## Dünya ve fizik

- `Dunya::yeni()`: ortografik 2B dünya
- `Dunya::yeni_3b()`: perspektif ve derinlik tamponlu 3B dünya

`FizikDunyasi`, render döngüsünden bağımsız yaklaşık 60 Hz sabit adım kullanır.

- Biriken süre sabit alt adımlara çevrilir.
- Tek kare en fazla 250 ms fizik süresi ekleyebilir.
- Tek karede en fazla sekiz fizik alt adımı çalıştırılır.
- Statik ve dinamik gövdeler `Aabb3` ile çözülür.
- X, Y ve Z eksenleri ayrı çözüldüğü için gövdeler duvar boyunca kayabilir.

AABB fizik hacmi görsel mesh dönüşünden bağımsız tutulabilir. Dönen veya karmaşık görseller için görünmez basit fizik varlığı kullanılması önerilir.

## CPU kaynak kimlikleri

Dünya dört ayrı kimlik alanı kullanır:

- `VarlikKimligi`: oyun nesnesi
- `MeshKimligi`: geometri
- `MalzemeKimligi`: renk ve doku seçimi
- `DokuKimligi`: RGBA8 piksel ve sampler kaynağı

Kimlikler ekleme sırasındaki sabit `usize` değerleridir. Bu aşamada silme ve kimlik yeniden kullanımı yoktur.

## Mesh kayıt defteri

`Dunya`, mesh geometrilerini eklenme sırasıyla saklar. Her mesh kaydının ayrıca varsayılan bir `MalzemeKimligi` vardır.

`Dunya::mesh_ekle` çağrısı:

1. `MeshVerisi` içindeki malzemeyi geometriden ayırır.
2. Malzemenin dokusunu bağımsız doku kayıt defterine gönderir.
3. Doku ve malzemeyi tekilleştirir.
4. Yalnızca geometriyi mesh kayıt defterine ekler.
5. Mesh ile varsayılan malzeme kimliğini aynı sıra numarasında ilişkilendirir.

Eski `MeshVerisi::yeni` API'si korunur. Dokusuz mesh sıfır UV ve beyaz varsayılan malzeme üretir.

## Doku tekilleştirme

`Dunya::doku_ekle`, yeni `DokuVerisi`ni mevcut kayıtlarla tam eşitlik üzerinden karşılaştırır:

- genişlik
- yükseklik
- RGBA8 baytları
- büyütme ve küçültme filtresi
- U ve V sarma davranışı

Eşit bir kayıt varsa yeni GPU dokusu oluşturulmaz; mevcut `DokuKimligi` döndürülür.

## Malzeme tekilleştirme

Dünya içindeki `MalzemeKaydi` yalnızca şunları taşır:

- doğrusal `Renk` taban çarpanı
- isteğe bağlı `DokuKimligi`

`Dunya::malzeme_ekle`, aynı renk ve aynı doku kimliği bileşimine sahip kaydı tekrar kullanır.

`Varlik::mesh`, mesh'in içe aktarılmış varsayılan malzemesini kullanır.

`Varlik::mesh_malzemeli`, aynı `MeshKimligi`ni başka bir `MalzemeKimligi` ile çizer. Bu kullanımda vertex ve indeks tamponları çoğaltılmaz.

## Model ve glTF sahne verisi

`tgame-model` şu türleri üretir:

- `MeshVerisi`: konum, normal, UV, indeks, içe aktarılmış malzeme ve sınır küresi
- `DokuVerisi`: doğrulanmış RGBA8 veri ve sampler ayarları
- `MalzemeVerisi`: taban renk ve isteğe bağlı taban doku
- `SinirKuresi`: yerel merkez ve yarıçap
- `ModelOrnegi`: model içindeki mesh sıra numarası ve birikmiş glTF dünya matrisi
- `ModelVerisi`: benzersiz primitive mesh'leri ve sahne örnekleri

Yükleyici yalnızca üçgen primitive'leri kabul eder. İndeks yoksa sıralı indeks üretir. Normal yoksa indeksli üçgenlerden yumuşatılmış normal hesaplar. `TEXCOORD_0` yoksa sıfır UV üretir.

Sonlu olmayan konum, normal, UV, renk veya node matrisleri GPU'ya ulaşmadan Türkçe `OyunHatasi` ile reddedilir.

## glTF primitive eşleme

Her kabul edilen glTF primitive için şu eşleme tutulur:

```text
(glTF mesh indeksi, primitive sırası) → ModelVerisi mesh sıra numarası
```

Bu sayede bir glTF mesh birden fazla node tarafından kullanılsa bile geometri bir kez çözümlenir. Node'lar yalnızca aynı mesh sıra numarasına işaret eden ayrı `ModelOrnegi` kayıtları üretir.

## glTF node hiyerarşisi

Yükleyici varsayılan sahneyi, yoksa ilk sahneyi kullanır.

Her kök node için özyinelemeli dolaşım yapılır:

```text
node dünya matrisi = ebeveyn dünya matrisi × node yerel matrisi
```

Bir node mesh içeriyorsa desteklenen her primitive için bir `ModelOrnegi` oluşturulur. Çocuk node'lar aynı birikmiş dünya matrisiyle dolaşılır.

Sahne örneği bulunmazsa geriye dönük uyumluluk için her mesh'e birim matrisli örnek üretilir.

`Dunya::model_sahnesi_ekle`, bütün mesh kaynaklarını kaydeder ve `ModelOrnegi` kayıtlarını otomatik Tgame varlıklarına dönüştürür.

## glTF malzeme içe aktarma

İlk malzeme aşamasında PBR metallic-roughness malzemesinden şunlar alınır:

- `baseColorFactor`
- `baseColorTexture`
- `texCoord`; yalnızca `TEXCOORD_0`
- `magFilter`
- `minFilter`
- `wrapS`
- `wrapT`

Desteklenen resim dönüşümleri:

- R8 → gri RGB + tam alfa
- R8G8 → gri RGB + ikinci kanal alfa
- R8G8B8 → RGB + tam alfa
- R8G8B8A8 → doğrudan RGBA

16 bit ve kayan noktalı resim biçimleri açık hatayla reddedilir.

## Mesh sınır küresi

Her mesh'in yerel sınır küresi yükleme sırasında hesaplanır:

1. Konumların eksen bazlı en küçük ve en büyük değerleri bulunur.
2. Merkez, AABB orta noktası olarak seçilir.
3. Yarıçap, merkezden en uzak tepe mesafesidir.

Bu küre hızlı görünürlük testi için muhafazakârdır; mesh'i dışarıda bırakmaz.

Dünya küresi:

```text
merkez = model matrisi ile dönüştürülmüş yerel merkez
yarıçap = yerel yarıçap × model matrisinin en büyük ölçeği
```

## Kamera frustum culling

`Kamera3B::kure_gorunur_mu`, dünya sınır küresini şu sırayla sınar:

1. Yakın düzlem
2. Uzak düzlem
3. Kamera sağ eksenindeki yatay görüş sınırı
4. Kamera gerçek yukarı eksenindeki dikey görüş sınırı

Frustum dışında kalan varlık:

- instance baytlarına yazılmaz
- instance tamponunda yer kaplamaz
- çizim grubuna girmez
- draw çağrısına ulaşmaz

Sonlu olmayan veya güvenilir biçimde değerlendirilemeyen bir küre yanlışlıkla kaybolmaması için görünür kabul edilir.

`Varlik::her_zaman_ciz`, özel kullanıcı arayüzü veya debug nesneleri için culling'i atlar.

## GPU kaynak kayıtları

`UcBoyutGrafik` üç bağımsız GPU kayıt defteri tutar:

- `Vec<GpuDoku>`
- `Vec<GpuMalzeme>`
- `Vec<GpuMesh>`

Eşitleme sırası önemlidir:

```text
CPU dokuları → GPU dokuları
CPU malzemeleri → GPU malzemeleri
CPU mesh'leri → GPU mesh'leri
```

Malzeme oluşturulurken işaret ettiği `DokuKimligi` GPU kayıt defterinde hazır olmak zorundadır.

Dokusuz malzemeler motorun tek 1×1 beyaz fallback dokusunu paylaşır.

## GPU doku kaydı

`GpuDoku` şu kaynakların ömrünü birlikte yönetir:

- `Rgba8UnormSrgb` texture
- `TextureView`
- WGPU sampler

Doku `COPY_DST | TEXTURE_BINDING` kullanımıyla oluşturulur ve `write_texture` ile yüklenir.

## GPU malzeme kaydı

`GpuMalzeme` geometri taşımaz. Yalnızca seçilen `GpuDoku` görünümü ve sampler'ından material bind group oluşturur.

Bind group sözleşmesi:

```text
group 0: kamera uniform tamponu
group 1 binding 0: taban renk texture view
group 1 binding 1: filtering sampler
```

## GPU mesh kaydı

`GpuMesh` yalnızca şunları taşır:

- vertex tamponu
- indeks tamponu
- indeks sayısı
- indeks biçimi

Her genel mesh tepesi 32 bayttır:

```text
konum.xyz   12 bayt   location 0
normal.xyz  12 bayt   location 1
uv.xy        8 bayt   location 7
```

Yerleşik küp `Uint16`, kayıtlı glTF mesh'leri `Uint32` indeks kullanır.

## Instance verisi

Her görünür 3B varlık 80 bayt instance verisi taşır:

- 64 bayt nihai model matrisi
- 16 bayt renk çarpanı

Nihai renk çarpanı:

```text
varlık rengi × seçilen malzemenin baseColorFactor değeri
```

Fragment shader bu değeri sRGB dokudan örneklenen renkle çarpar ve temel yönsel ışığı uygular.

## Mesh + malzeme batching

Çizim anahtarı:

```text
CizimAnahtari {
    malzeme: MalzemeAnahtari,
    mesh: MeshAnahtari,
}
```

BTreeMap sıralaması nedeniyle gruplar önce malzemeye, sonra mesh'e göre sıralanır.

Render geçişinde:

- pipeline bir kez bağlanır
- kamera bind group'u bir kez bağlanır
- ortak instance tamponu bir kez bağlanır
- malzeme değişmedikçe group 1 yeniden bağlanmaz
- grup başına mesh vertex/index tamponları bağlanır
- grup başına bir `draw_indexed` çağrısı yapılır

Aynı mesh + malzeme bileşimini kullanan yüzlerce görünür varlık tek draw çağrısında çizilir.

## Örnek stres sahnesi

`ilk-oyun` örneği aynı piramit mesh'ini:

- glTF'nin dokulu varsayılan malzemesiyle
- bağımsız turkuaz malzemeyle

çizer.

Ayrıca kamera görüşünün çok dışında 256 ek piramit oluşturur. Bu varlıklar dünya kayıtlarında bulunur fakat frustum culling nedeniyle instance tamponuna ve draw gruplarına ulaşmaz.

## Geriye dönük uyumluluk

- `Varlik::kup` korunur.
- `Varlik::mesh` korunur.
- `Dunya::model_ekle` yalnızca mesh kimlikleri isteyen kod için korunur.
- 2B çizici ayrı modülde kalır.
- `ikiboyut-oyun` her kalite koşusunda derlenir ve test edilir.

## Güncel sınırlar

- Yalnızca `TEXCOORD_0` desteklenir.
- Yalnızca taban renk dokusu çizilir.
- Mipmap zinciri üretilmez.
- Anisotropic filtering yoktur.
- Metalik/pürüzlülük, normal, emissive ve occlusion haritaları çizilmez.
- Mesh geometrileri henüz içerik eşitliği veya hash ile tekilleştirilmez.
- Alfa modu, alpha cutoff ve çift taraflılık uygulanmaz.
- Animasyon, skinning ve morph target desteği yoktur.
- Kaynak silme, sıcak yenileme ve GPU kaynak boşaltma henüz yoktur.

## Sonraki büyük aşamalar

1. Mesh içerik hash'i ve geometri tekilleştirme
2. Mipmap üretimi ve anisotropic filtering
3. Frustum sonuçlarının kareler arası önbelleği ve mekânsal bölümleme
4. Metalik/pürüzlülük ve normal haritalı PBR
5. Dünya ışıkları ve gölge haritası
6. glTF animasyon, iskelet ve skinning
7. Kaynak sıcak yenileme ve yaşam döngüsü
8. Hareketli–hareketli çarpışma için broad phase
