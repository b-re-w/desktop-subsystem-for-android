# Uygulamalar ve pencereler

[English](../apps.md) · [한국어](../ko/apps.md) · [日本語](../ja/apps.md) · [简体中文](../zh-CN/apps.md) · [繁體中文](../zh-TW/apps.md) · [Español](../es/apps.md) · [Français](../fr/apps.md) · [Deutsch](../de/apps.md) · [Português (Brasil)](../pt-BR/apps.md) · [Русский](../ru/apps.md) · [Italiano](../it/apps.md) · [Tiếng Việt](../vi/apps.md) · [Bahasa Indonesia](../id/apps.md) · **Türkçe**

Her Android uygulama görevi, kendi görev çubuğu düğmesiyle kendine ait bir masaüstü penceresi
olarak açılır. Bir telefon ekranının çevresinde öykünücü çerçevesi yoktur. Bu sayfa Windows'taki
Desktop Subsystem for Android™ (DSA) yazılımını anlatır.

## Bir uygulama açma

- **Başlat menüsü.** Yüklü her uygulamanın kendi adı ve simgesiyle bir Başlat menüsü girdisi
  vardır, bu yüzden Windows araması onu bulur. Android çalışmıyorsa onu açmak önce Android'i
  başlatır.
- **DSA Settings.** **Apps** (uygulamalar) sayfasında uygulamanın yanındaki **Open** düğmesini
  seçin.
- **Komut satırı.** `dsa apps list` yüklü uygulamaları paketleriyle birlikte listeler.
  `dsa launch <package>` birini açar, örneğin `dsa launch com.android.settings`.

Başka bir programın Başlat menüsü girdisi uygulamanın adını zaten taşıyorsa DSA ada paketi ekler,
örneğin `Clock (com.example.clock)`. DSA yalnızca kendi oluşturduğu girdileri değiştirir veya
kaldırır.

## Masaüstü kısayolları ve atlama listeleri

Uygulama simgeleri bir Android ana ekranındaki gibi değil, Windows uygulamalarınınki gibi çalışır.

- **Masaüstü kısayolu.** DSA Settings'in **Apps** sayfasında uygulamanın yanındaki **Create
  desktop shortcut** düğmesini seçin. Uygulamanın görev çubuğu düğmesine sağ tıklayıp **Create
  desktop shortcut**'ı da seçebilirsiniz. Komut satırından: `dsa apps desktop-shortcut <package>`.
  Kısayol, uygulamayı Başlat menüsü girdisinin açtığı gibi açar.
- **Uygulama kısayolları.** Bir Android uygulamasının sunduğu kısayollar (örneğin bir mesajlaşma
  uygulamasının yeni sohbeti), uygulamanın görev çubuğu düğmesine veya Başlat menüsü girdisine
  sağ tıkladığınızda görünür. Birini seçmek uygulamanın o ekranını bir pencerede açar; Android
  çalışmıyorsa önce onu başlatır. Uygulama kısayollarını değiştirdiğinde liste de değişir. Komut
  satırından: `dsa apps shortcut <package> <id>`; kimlikleri `dsa apps info <package> --json`
  listeler.
- **Bir uygulamanın istediği kısayollar.** Bir uygulama kısayollarından birini masaüstünüze
  eklemek istediğinde DSA önce size sorar ve yalnızca kabul ederseniz ekler.

Bir uygulamanın adı değiştiğinde (örneğin bir güncellemeden sonra), Başlat menüsü girdisi, masaüstü
kısayolları ve görev çubuğundaki sabitlemesi yeni adı izler ve çalışmaya devam eder. Sildiğiniz bir
masaüstü kısayolu yeniden oluşturulmaz. Bir uygulamayı kaldırmak masaüstü kısayollarını da kaldırır.

## Uygulama pencereleriyle çalışma

- Her görevin, uygulamanın adı ve simgesiyle kendi penceresi, görev çubuğu düğmesi ve Alt+Tab
  girdisi vardır. Görev Yöneticisi her uygulamayı kendi adıyla listeler.
- Başlık çubuğu uygulamanın parçasıdır: uygulamanın renkleri pencerenin üst kenarına kadar uzanır
  ve Windows 11 başlık çubuğu düğmeleri bunun üzerinde durur. Uygulamanın simgesi ve adı soldadır.
  Uygulama geri gidebildiğinde simge bir geri okuna dönüşür; geri gitmek için onu seçin.
- Ek düzenleri (Snap layouts; ekranı kapla düğmesinin üzerine gelin), başlık çubuğunu sürükleme,
  ekranı kaplamak için ona çift tıklama ve canlı yeniden boyutlandırma çalışır. Yeniden
  boyutlandırma uygulamayı yeniden başlatmaz. Ekranı kaplayan bir pencere monitörün çalışma alanını
  doldurur ve görev çubuğunu görünür bırakır.
- Android gezinti çubuğu yoktur. Bir uygulamanın alt çubuğu, altında boş bir şerit olmadan
  pencerenin alt kenarında durur.
- Fare (üzerine gelme dahil), dokunma, kalem (basınç, silgi ve yan düğme) ve klavyenin hepsi
  çalışır. Metin, Korece ve diğer diller dahil Windows giriş yönteminizden gelir; oluşturma Android
  düzenleyicisinde satır içinde görünür ve aday penceresi imlecin yanında durur. Android klavye
  uygulaması yoktur.
- Dosyaları Dosya Gezgini'nden bir uygulamanın penceresine sürükleyin veya bir uygulamadan Dosya
  Gezgini'ne sürükleyin.
- Ekran Okuyucusu (Narrator) ve NVDA gibi ekran okuyucular uygulamayı UI Automation üzerinden
  okur.
- Bir pencereyi kapatmak, Android'den o görevi sonlandırmasını ister.

## Pencere menüsü

Başlık çubuğuna sağ tıklayın, uygulamanın simgesini seçin veya Alt+Space tuşlarına basın. Olağan
Windows öğelerinden sonra menüde şunlar bulunur:

- **Full screen** (tam ekran): F11 gibi tüm ekranı doldurur. Çıkmak için yeniden seçin veya F11'e
  basın.
- **New window** (yeni pencere): birden çok pencereyi destekleyen uygulamalar için uygulamanın
  başka bir penceresini açar.
- **Screenshot** (ekran görüntüsü): uygulamanın ekranını başlık çubuğu olmadan panoya kopyalar ve
  Resimler klasörünüzdeki Ekran Görüntüleri klasörüne kaydeder.
- **Aspect ratio** (en boy oranı): büyük bir pencerede gerilmiş görünen uygulamalar için,
  Android'in uygulamaya ait en boy oranı ayarını açar.
- **Open by default** (varsayılan olarak aç): uygulamanın desteklediği bağlantıların uygulamada mı
  yoksa tarayıcınızda mı açılacağını seçer.
- **App info** (uygulama bilgisi): uygulamanın DSA Settings'in **Apps** sayfasındaki girdisini
  açar.

## Tam ekran

Bir uygulama tam ekran istediğinde penceresi bunu iki yoldan biriyle gösterir:

- **Tüm ekranda**: pencere monitörü doldurur ve başlık çubuğu gizlenir. Onu göstermek için
  işaretçiyle ekranın üst kenarına dokunun; simge durumuna küçültme ve kapatma ile birlikte görünür.
  Ortadaki düğme tam ekrandan çıkar.
- **Pencerenin içinde**: pencere boyutunu ve başlık çubuğunu korur, uygulama onu doldurur.

Uygulamalar iki tür yolla ister ve her türün kendi varsayılanı vardır:

| İstek türü | Örnek | Varsayılan |
|---|---|---|
| Açık bir tam ekran isteği | Bir video oynatıcının tam ekran düğmesi | Tüm ekranda |
| Yalnızca çubukları gizleme | Bir kısa video akışı | Pencerenin içinde |

Varsayılanları DSA Settings'in **Display & GPU** (ekran ve GPU) sayfasındaki **App full screen** ve
**Apps that hide their bars** altında, tek bir uygulama için ise **Apps** sayfasındaki **Full
screen request** ve **When only the bars hide** altında değiştirin. Uygulamanın kendi seçimi önce gelir. F11 ve pencere
menüsündeki **Full screen**, ayar ne olursa olsun tüm ekranda tam ekranı her zaman açıp kapatır.

Komut satırından:

```
dsa settings set app-fullscreen screen|window           # açık istekler için varsayılan
dsa settings set app-immersive screen|window            # gizlenen çubuklar için varsayılan
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## Bir uygulamanın pencere boyutunu ayarlama

DSA Settings'in **Apps** sayfasında:

- Uygulamanın pencerelerini açıldıkları boyutta tutmak için **Resizable** (yeniden
  boyutlandırılabilir) ayarını kapatın.
- Uygulamanın pencerelerinin açılacağı boyutu seçin: Android'in seçtiği boyut veya dikey ya da
  yatay bir telefon veya tablet boyutu.

Değişiklik, uygulamanın açtığı bir sonraki pencereye uygulanır. Komut satırından:

```
dsa app-window <package> --fixed-size on        # yeniden boyutlandırılamaz, ekranı kaplayamaz
dsa app-window <package> --start-size 1280x800  # bu boyutta açılır; auto seçimi Android'e bırakır
dsa app-window <package> --dpi-scaling system   # başka bir monitörde nasıl ölçeklendiği; aşağıya bakın
dsa app-window <package> reset                  # varsayılanlara döner
```

## Bir uygulamayı başlık çubuğunun altından başlatma

Bir uygulama, penceresinin en üstüne, başlık çubuğunun altına kadar çizer. Bu yüzden bazı
uygulamaların üst çubuğu sıkışır ya da bir kısmı başlık çubuğunun altında kalır. Böyle bir
uygulama için DSA Settings'in **Apps** sayfasında **Start below the title bar** (başlık çubuğunun
altından başlat) seçeneğini açın. Uygulamanın içeriği o zaman başlık çubuğunun altından başlar ve başlık
çubuğu, uygulamanın orada gösterdiği rengi alır. Varsayılan olarak kapalıdır ve uygulamanın
penceresi bir sonraki açılışında geçerli olur. Komut satırından:

```
dsa app-window <package> --below-title-bar on   # off varsayılana döner
```

## Bir uygulamayı başka ölçekteki bir monitöre taşıma

Android ana monitörünüzün ölçeğiyle başlar, bu yüzden uygulamalar orada doğru boyutta açılır.
Bir pencereyi başka ölçekteki bir monitöre
taşıdığınızda uygulama ekrandaki boyutunu korur. Her uygulama bunu aşağıdaki yollardan biriyle
yapar; bunu **Apps** sayfasındaki **Scaling** altında veya
`dsa app-window <package> --dpi-scaling` ile seçersiniz:

- **Automatic** (`auto`, varsayılan): DSA'nın, çalışırken ölçeği değişince bozulan uygulamaları
  içeren yerleşik listesindeki uygulamalar için **System**, diğer tüm uygulamalar için **App**.
  Listede YouTube bulunur: başladığı ölçeğe göre düzenini korur, bu yüzden taşındıktan sonra alt
  sekmeleri ve mini oynatıcısı yeniden başlayana kadar bozuk kalırdı. Ayarlar, böyle bir uygulamanın
  varsayılanını **Automatic (system, recommended for this app)** olarak gösterir. Uygulama için kendi
  seçiminiz önce gelir.
- **App** (`app`): pencereyi sürüklerken DSA onu yeni monitöre göre ölçekler. Bıraktığınızda
  uygulama yeni monitörün ölçeğinde yeniden çizilir ve metni keskin kalır: Android ekranını bir kez
  yeniden oluşturur ve durumunu (kaydırma konumu, yazılan metin) geri yükler. Bu yüzden her
  taşımadan sonra, "Uygulama" ölçeklemesi kullanan bir Windows programında olduğu gibi kısa bir süre
  yeniden çizilir. Bu, ölçek değişikliğini kendisinin işlediğini söyleyen bir uygulama için bile
  geçerlidir, çünkü bu uygulamaların bazıları ekranlarının yalnızca bir kısmını yeni ölçekte yeniden
  çizer.
- **System** (`system`): uygulama ölçeğini korur ve DSA penceresini yeni monitöre göre ölçekler.
  Uygulama hiçbir zaman yeniden oluşturulmaz. Büyütüldüğünde metin biraz yumuşak görünebilir.
- **System (forced)** (`system-forced`): uygulama, bunu işlediğini söylemese bile yeniden
  başlatılmadan yeni ölçeği alır. Henüz kullanılamaz: bu sürümde bulunmayan bir Android değişikliği
  gerektirir, bu yüzden o zamana kadar **System** gibi çalışır.

Uygulamalardaki metin, Windows metin boyutunu izler (Ayarlar > Erişilebilirlik > Metin boyutu).

## Bildirimler

Android bildirimleri, uygulamanın adı ve simgesiyle Windows bildirimleri olarak görünür.

- Uygulamayı açmak için bir bildirimi seçin.
- Bir bildirimi kapatın; uygulama onu gösterilmiş tutmadığı sürece Android'de de kapatılır.
- Android Doze'dayken bildirimler gelmeye devam eder. Bkz. [Bellek](memory.md).

## Arka planda çalışan uygulamalar

Penceresi kapandıktan sonra çalışmaya devam eden bir uygulama (örneğin bir müzik çalar), bildirim
alanında bir simge olarak görünür. Araç ipucu, uygulamanın bildirimidir.

- Uygulamayı açmak için simgeyi seçin.
- **Open**, bildirimin kendi eylemleri ve **Stop** için ona sağ tıklayın.
- **Stop** önce sorar. Bundan sonra soruyu atlamak için **Do not ask again** kutusunu işaretleyin.
  Yeniden sorulmak için DSA Settings'in **Running in the background** (arka planda çalışanlar)
  sayfasında **Ask before stopping an app from the notification area** ayarını açın.
- Medya oynatan bir uygulama medya tuşları ve Windows medya denetimleriyle de çalışır.

DSA Settings'in **Running in the background** sayfası bu uygulamaları her biri için **Open** ve
**Stop** ile listeler. Komut satırından: `dsa background`, `dsa background open <package>` ve
`dsa background stop <package>`.

## Masaüstündeki pencere öğeleri

Bir Android pencere öğesi (widget), masaüstünüzde kendine ait bir pencerede durabilir. Diğer
pencerelerin altında kalır, masaüstünü gösterdiğinizde yerinde durur ve görev çubuğunda veya
Alt+Tab'da görünmez.

1. DSA Settings'in **Widgets** (pencere öğeleri) sayfasını açın.
2. **Widgets your apps offer** altında bir pencere öğesinin yanındaki **Add to desktop** düğmesini
   seçin. Uygulama pencere öğesini ayarlamanızı isterse kurulumu bir pencerede açılır.
3. Bir pencere öğesini taşımak için üzerinde herhangi bir yere basın ve yarım saniye kıpırdatmadan
   tutun. Öğe, bir kenarlık ve yeniden boyutlandırılabildiği her kenarda bir tutamaçla birlikte
   kalkar. Taşımak için sürükleyin veya yeniden boyutlandırmak için bir tutamacı sürükleyin. Hızlı
   bir sürükleme pencere öğesinin kendisine kalır, böylece listeleri kaydırılmaya devam eder. Önce
   kaldırmadan, yeniden boyutlandırmak için pencere öğesinin bir kenarını da sürükleyebilirsiniz;
   bunu yalnızca yeniden boyutlandırılabildiği kenarlar yapar. DSA onu nereye ve hangi boyutta
   koyduğunuzu hatırlar.
4. **Widget settings** (pencere öğesi yeniden ayarlanabiliyorsa), izin verdiği boyutlardan birini
   seçmek için **Resize** ve **Remove** için pencere öğesine sağ tıklayın veya uzun basmadan sonra
   bırakın. Menü tuşu ve Shift+F10 da menüyü açar. Bu menüde ok tuşları, Enter ve Esc çalışır ve bir
   harf bir öğeyi seçer: **Resize** için R, **Remove** için M.

Komut satırından: `dsa widgets list`, `dsa widgets add <provider>` ve `dsa widgets remove <id>`.

## Bağlantılar ve pano

- Bir Android uygulamasında dokunduğunuz web bağlantısı varsayılan tarayıcınızda açılır. Yüklü bir
  uygulamanın kendine ait olduğunu doğruladığı bir bağlantı o uygulamada açılır.
- Bunun yerine tüm web bağlantılarını Android uygulamalarında açmak için DSA Settings'in
  **Advanced** (gelişmiş) sayfasında **Open web links in Android apps** ayarını açın
  (`dsa settings set links-in-android on`).
- Kopyaladığınız metin Windows ile Android arasında iki yönde paylaşılır. Bunu durdurmak için
  **Advanced** sayfasında **Share the clipboard with Android** ayarını kapatın
  (`dsa settings set clipboard off`).

## Google Play ve korumalı video

DSA hiçbir Google yazılımı içermez. İsterseniz DSA, size kaynağı ve koşulları gösterdikten sonra
onu sizin için kaynağından indirir. İkisi de Android'in bir sonraki başlatılmasından itibaren
geçerli olur ve ikisi de yeniden kaldırılabilir.

- **Google Play.** DSA Settings'in **Advanced** sayfasında **Google Play** altındaki **Install**
  düğmesini seçin veya `dsa gapps install` komutunu çalıştırın. DSA, Android sürümünüz için
  herkese açık LiteGapps Core paketini (Google Play hizmetleri, Play Store ve hesap eşitleme)
  indirir. Bu Android, Google sertifikalı değildir, bu yüzden cihazı kaydedene kadar Google oturum
  açmanızı reddedebilir: **Register this device with Google**, Android kimliğini gösterir ve kayıt
  sayfasını açar. `dsa gapps status` durumu gösterir ve `dsa gapps remove`, Android dururken bile
  Google'ın uygulamalarını kaldırır.
- **Widevine.** Yayın uygulamalarının korumalı video oynatması için Google'ın Widevine modülü
  gerekir. **Advanced** sayfasında **Protected content** altındaki **Get** düğmesini seçin veya
  `dsa drm widevine install` komutunu çalıştırın. DSA, Android sürümünüz için Google'ın Android
  öykünücü görüntüsünü (yaklaşık 2 GB) indirir, yalnızca Widevine modülünü değiştirmeden saklar ve
  gerisini siler. Buradaki Widevine L3 güvenlik düzeyindedir, bu yüzden yayın hizmetleri en fazla
  standart çözünürlük gönderir; yüksek çözünürlük için bir Windows tarayıcısı veya uygulaması
  kullanın. `dsa drm widevine remove` onu kaldırır.

## Yalnızca ARM için derlenmiş uygulamalar

x64 bir bilgisayarda yalnızca ARM işlemciler için kodu olan bir uygulama bir ARM çevirmeni
üzerinden çalışır. x64 bilgisayardaki Android bir çevirmenle, açık kaynaklı çevirmen Digitalis ile
birlikte gelir ve onu ilk başlatmadan itibaren kullanır. Bunun yerine başka bir çevirmen
kullanabilirsiniz:

- **Google'ın ARM çevirmeni.** **Advanced** sayfasında **Google's ARM translator** yanındaki
  **Get** düğmesini seçin veya `dsa arm-translation fetch google` komutunu çalıştırın. DSA önce
  nereden geldiğini ve Google'ın koşullarını
  ([Android SDK License](https://developer.android.com/studio/terms)) gösterir ve siz kabul edene
  kadar hiçbir şey indirmez. Ardından bilgisayarınız, Google'ın Android sürümünüz için sunduğu
  Android öykünücü görüntüsünü (yaklaşık 2 GB) indirir, yalnızca çevirmeni saklar ve gerisini
  siler. DSA, Google'ın çevirmenini dağıtmaz veya başkasına aktarmaz.
- **Kendi derlediğiniz veya başka bir yerden aldığınız bir paket** (örneğin açık kaynaklı çevirmen
  Digitalis'ten yapılmış bir paket): `dsa arm-translation install <folder>`.

Ardından Android'i yeniden başlatın: DSA Settings'in **General** (genel) sayfasında durdurun, sonra
bir uygulama açın.

Android ile birlikte gelen çevirmene dönmek için **Advanced** sayfasında **Use the default
translator** yanındaki **Use** düğmesini seçin veya `dsa arm-translation use-default` komutunu
çalıştırın. Bu, yüklediğiniz paketi Android'in bir sonraki başlatılmasından itibaren kaldırır.

`dsa arm-translation status`, Android'in hangi çevirmeni kullandığını söyler. **Advanced**
sayfasındaki **Run apps built only for ARM** anahtarı, hiçbir şeyi kaldırmadan çeviriyi açar veya
kapatır. Hem ARM hem de x64 kodu olan bir uygulama için **Apps** sayfasındaki **Run as ARM**
(`dsa apps run-as-arm <package> on`), bunun yerine ARM kodunu çalıştırır. Yalnızca 64 bit ARM
uygulamaları (`arm64-v8a`) çalışabilir. ARM64 bir bilgisayarda Android, ARM uygulamalarını kendisi
çalıştırır ve çevirmen gerektirmez.

## Kök erişimi

Yedekleme araçları gibi bazı uygulamalar kök erişimi gerektirir. DSA bunu, bir Linux
bilgisayardaki `sudo` gibi verir: yalnızca bilgisayarın bir yöneticisi onayladığında ve her
seferinde tek bir istek için. Varsayılan olarak kapalıdır ve izin verilmeyen uygulamalar `su`'yu
hiç göremez.

1. Bir yönetici, DSA Settings'in **Advanced** sayfasında **Root access** altındaki **Let Android
   apps ask for root** ayarını açar (`dsa settings set root-elevation on`). Android'in bir sonraki
   başlatılmasından itibaren geçerli olur.
2. **Apps** sayfasında uygulamanın **Root access** ayarını yapın:
   - **Root: not allowed** (varsayılan): uygulamanın `su`'su yoktur.
   - **Root: ask each time**: uygulama `su` kullandığı her seferde Windows, uygulamanın kendi adı ve
     simgesiyle yönetici istemini (UAC) gösterir. Standart bir kullanıcı bir yöneticinin parolasını
     girer. Bir onay, o uygulama için 15 dakika geçerlidir.
   - **Root: remember (no asking)**: uygulama `su`'yu sormadan kullanır. Bunu seçmek bir yöneticinin
     onayını gerektirir.

   Komut satırından: `dsa apps root <package> off|ask|remember`.

Geliştirici modu açıkken `adb shell` de `su` kullanabilir. **Apps** sayfasında kendine ait bir
satırı (**adb shell**) vardır ve varsayılan olarak her seferinde sorar (`dsa apps root adb ask`).
İstemi **adb shell** olarak adlandırılır ve komutu gösterir.

Android'in kendi bir parçası istediğinde istem onu adlandırır: **Android system**, **Android phone
services**, **Android Bluetooth**, **Android NFC** veya **Android system service**. Tek bir
kullanıcı kimliğini paylaşan uygulamalar tek bir grup olarak onaylanır ve istem hepsini listeler.

Her istek; uygulama, zaman, komut ve onaylanıp onaylanmadığıyla birlikte hizmet günlüğünde tutulur.
Özelliği kapatmak, yeniden başlatma gerektirmeden `su`'yu tüm uygulamalardan aynı anda gizler.

---

Android, Google LLC'nin ticari markasıdır. Windows, Microsoft şirketler grubunun ticari markasıdır.
DSA, Google veya Microsoft ile bağlantılı değildir, onlar tarafından onaylanmamış ve
desteklenmemektedir.
