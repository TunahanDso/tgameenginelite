# Tgame Engine Lite Macera Mimarisi

Bu belge, birkaç saat veya daha uzun süren hikâyeli oyunların kalıcı durum, görev, diyalog, etkileşim, sahne geçişi ve kayıt ihtiyaçları için kullanılan sözleşmeleri tanımlar.

## Temel ilke

`Dunya` çizilebilir ve fiziksel oyun alanını, `Macera` ise hikâyenin kalıcı ve veri odaklı çalışma zamanını taşır.

Bir oyun karesinde önerilen sıra:

```text
girdi → oyuncu/fizik → alan denetimi → etkileşim → diyalog → olay zinciri → sahne geçişi → kayıt
```

`Oyun::her_kare_macera`, oyun koduna aynı karede `Girdi`, `Zaman`, `Dunya` ve `Macera` erişimi verir.

## Kalıcı kimlikler

Uzun oyun içeriği sıra numaralarına değil metin tabanlı kalıcı kimliklere dayanır:

- `EsyaKimligi`
- `GorevKimligi`
- `DiyalogKimligi`
- `SahneKimligi`
- `EtkilesimKimligi`
- `AlanKimligi`
- `KuralKimligi`
- `KontrolNoktasiKimligi`

Bu kimliklerin kayıt dosyası yayımlandıktan sonra değiştirilmemesi gerekir. Oyuncunun kaydı içerik tanımlarından bağımsız çalışma zamanı verisini bu kimliklerle geri bağlar.

## Hikâye kara tahtası

`OyunDurumu` üç tür kalıcı değişken saklar:

- Mantıksal bayrak
- `i64` sayaç
- Metin değeri

Örnek:

```rust
macera.durum_mut().bayrak_ayarla("kale_kapisi_acik", true);
macera.durum_mut().sayac_ayarla("bulunan_muhur", 2);
macera.durum_mut().metin_ayarla("oyuncu_unvani", "Kuzey Gözcüsü");
```

Koşul sistemi bu değerleri envanter, görev aşaması ve etkin sahneyle birleştirebilir. `Tumu`, `Herhangi` ve `Degil` düğümleriyle veri odaklı koşul ağaçları kurulabilir.

## Envanter

`EsyaTanimi` şunları belirler:

- Kalıcı eşya kimliği
- Oyuncuya gösterilen ad ve açıklama
- Yığın sınırı
- Görev eşyası koruması

Görev eşyası olarak işaretlenen eşya genel çıkarma işlemiyle kaybedilemez. Envanter değişiklikleri otomatik `OyunOlayi` üretir ve görev/kural sistemini ilerletir.

## Çok adımlı görevler

Bir `GorevTanimi`, sıralı `GorevAdimi` kayıtlarından oluşur. Her adım bir veya daha fazla `GorevHedefi` taşıyabilir.

Hedefler olay filtresi ve gereken miktarla tanımlanır. Örneğin:

```text
1. NPC ile konuş
2. Üç mühür parçası topla
3. Tapınak alanına gir
4. Muhafızı yen
```

Her hedef olaylardan otomatik ilerler. Adımdaki bütün hedefler tamamlanınca sonraki adıma geçilir; son adım tamamlanınca görev `Tamamlandi` durumuna gelir.

Görev çalışma zamanı:

- `Kilitli`
- `Etkin`
- `Tamamlandi`
- `Basarisiz`

Aynı görev başlatma isteğinin tekrar gelmesi ilerlemeyi sıfırlamaz.

## Dallanan diyaloglar

`DiyalogTanimi`, kimlikli düğümlerden oluşan bir diyalog grafiğidir. Bir düğüm:

- Konuşmacı
- Metin
- Görünürlük koşulları olan seçenekler
- Seçim sonrası eylemler
- Sonraki düğüm veya diyalog bitişi

barındırabilir.

Seçenek eylemleri bayrak değiştirebilir, görev başlatabilir, eşya verebilir, başka olay yayınlayabilir veya sahne geçişi isteyebilir. Böylece konuşmalar yalnızca metin değil, hikâye akışının veri odaklı karar noktalarıdır.

## Etkileşim noktaları

`EtkilesimNoktasi` dünya konumu, yarıçap, gösterilecek ileti, koşullar, eylemler ve tekrar davranışı taşır.

`Macera::yakin_etkilesim`, oyuncuya en yakın kullanılabilir noktayı seçer. `Macera::etkiles` menzili ve koşulları tekrar doğruladıktan sonra eylemleri çalıştırır.

Tek kullanımlı sandık, kapı veya NPC başlangıç olayı `Tekrarlama::BirKez`; sürekli kullanılabilen konuşma veya düzenek `Tekrarlama::HerZaman` kullanabilir.

## Alan tetikleyicileri

`AlanTetikleyicisi`, eksenlere hizalı `KutuAlan` içinde oyuncunun giriş ve çıkışını izler.

Alanlar şu işler için kullanılabilir:

- Bölüm başlangıcı
- Sinematik veya müzik tetikleme olayı
- Pusu başlatma
- Görev hedefi ilerletme
- Zehirli bölgeye giriş/çıkış
- Kontrol noktası etkinleştirme

Tetikleyicinin önceki içeride/dışarıda durumu saklandığı için her kare aynı giriş olayını üretmez.

## Olay kuralları

`OlayKurali`, bir `OlayFiltresi`, koşullar, eylemler ve tekrar davranışından oluşur.

Bu katman, oyun koduna özel `if` zincirleri yazmadan hikâye bağlantıları kurar:

```text
"anahtar alındı" + "mahzen görevi etkin" → mahzen kapısını aç
"muhafız yenildi" → bölüm sonu görevini tamamla
"köprü sahnesine geçildi" → ilk giriş konuşmasını başlat
```

Bir olayın ürettiği eylem yeni olaylar doğurabilir. Sonsuz içerik döngülerine karşı tek işlem zinciri en fazla 1.024 olayla sınırlandırılır. Son 64 olay hata ayıklama ve oyun içi günlük için saklanır.

## Sahne geçişleri

`SahneTanimi`, sahne kimliği ve adlandırılmış giriş noktaları taşır.

Geçiş iki aşamalıdır:

1. `sahne_gecisi_iste` güvenli geçiş isteğini sıraya alır.
2. `sahne_gecisini_al` geçişi uygular ve oyun koduna dünyayı yeniden kurması için `SahneGecisi` döndürür.

Bu ayrım, fizik veya render koleksiyonları üzerinde dolaşılırken dünyanın ortada değiştirilmesini engeller.

## Kontrol noktaları

Kontrol noktası kimliği, sahne ve giriş noktasıyla birlikte kalıcı çalışma zamanında tutulur. Oyuncu öldüğünde oyun kodu son kontrol noktasını okuyup doğru sahneyi ve doğma konumunu kurabilir.

## Kayıt ve yükleme

`KayitYoneticisi` numaralı kayıt yuvaları kullanır.

Kayıt güvenliği:

1. İçerik geçici dosyaya yazılır.
2. Dosya `sync_all` ile diske zorlanır.
3. Mevcut kayıt değiştirilirken yedekleme/değiştirme uygulanır.
4. Kayıt imzası ve sürümü yüklemede doğrulanır.

Kayıt dosyası içerik tanımlarını değil çalışma zamanını saklar. Yükleme öncesinde oyun aynı eşya, görev, diyalog ve sahne tanımlarını yeniden kaydetmelidir; sonra kayıt bu tanımların üzerine uygulanır.

Kalıcı veriler arasında şunlar bulunur:

- Hikâye bayrakları, sayaçları ve metinleri
- Envanter
- Görev ilerlemeleri
- Etkin sahne ve bekleyen geçiş
- Diyalog çalışma zamanı
- Tüketilmiş etkileşim, alan ve kurallar
- Kontrol noktası
- Toplam oynama süresi

Entegrasyon testi, iki saatten uzun oynama süresiyle birlikte görev, görev eşyası, etkin sahne ve kara tahta değerlerinin gerçek bir yuva dosyasına yazılıp yeni çalışma zamanına geri yüklenmesini doğrular.

## Uzun oyun içerik düzeni

Önerilen oyun projesi yapısı:

```text
oyun/
  hikaye/
    esyalar.rs
    gorevler.rs
    diyaloglar.rs
    kurallar.rs
  sahneler/
    koy.rs
    orman.rs
    kale.rs
  sistemler/
    oyuncu.rs
    savas.rs
    kayit.rs
```

Motor içerik tanımlarını Rust kodunda tutar; çalışma zamanı bunları kalıcı kimliklerle bağlar. Daha sonraki veri dosyası veya editör katmanı aynı API üzerine kurulabilir.

## Güncel kapsam ve sonraki ihtiyaçlar

Bu paket, birkaç saatlik hikâyeli bir maceranın mantık ve kalıcılık omurgasını sağlar. Görsel kullanıcı arayüzü, ses/müzik, animasyon, sinematik kamera, karakter denetleyicisi, savaş sistemi, yapay zekâ/navmesh ve büyük dünya akışı ayrı motor katmanları olarak eklenecektir.

Kalite kapısı:

```powershell
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
```
