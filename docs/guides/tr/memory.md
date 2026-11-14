# Bellek ve boşta davranışı

[English](../memory.md) · [한국어](../ko/memory.md) · [日本語](../ja/memory.md) · [简体中文](../zh-CN/memory.md) · [繁體中文](../zh-TW/memory.md) · [Español](../es/memory.md) · [Français](../fr/memory.md) · [Deutsch](../de/memory.md) · [Português (Brasil)](../pt-BR/memory.md) · [Русский](../ru/memory.md) · [Italiano](../it/memory.md) · [Tiếng Việt](../vi/memory.md) · [Bahasa Indonesia](../id/memory.md) · **Türkçe**

Desktop Subsystem for Android™ (DSA), belleği yalnızca Android durduğunda değil, Android çalışırken
de bilgisayarınıza geri verir. Bunun için ayarlanacak bir şey yoktur. Bu sayfa neler olduğunu ve
bunu nasıl görebileceğinizi açıklar.

## Android'in ne kadar bellek kullandığını görme

DSA Settings'in **Memory** (bellek) sayfasını açın veya `dsa memory` komutunu çalıştırın. İkisi de
şunları gösterir:

- **On this PC** (bu bilgisayarda): Android sanal makinesinin, pencere yöneticisinin ve her
  uygulama penceresinin tuttuğu bellek.
- **Inside Android** (Android'in içinde): kullanımdaki bellek, önbellek, sıkıştırılmış bellek ve en
  büyük işlemler. Android bunu başladıktan birkaç dakika sonra bildirir.

## Boşta durumu Doze

Tüm Android pencereleri 30 saniye boyunca kapalı veya simge durumunda kaldığında Android'in ekranı
kapanır ve Android, cepteki bir telefon gibi Doze'a girer.

- Bildirimler, alarmlar ve pencere öğeleri Doze'da çalışmaya devam eder. İşlemci arada dinlenir.
- `dsa status`, `Dozing` der. DSA Settings'in **General** (genel) sayfası **Dozing (idle)** der.
- Herhangi bir Android penceresini göstermek Android'i hemen uyandırır.

Android, bilgisayarınızın uyku durumunu da izler. Bilgisayar uykuya geçmeden önce durumunu kaydeder
ve bilgisayar uyandığında saatini ayarlar.

## Dondurma (varsayılan olarak kapalı)

Dondurma, Android'deki tüm işlemleri durdurur. O sırada işlemci kullanmaz, ancak bildirimler,
alarmlar ve pencere öğeleri de durur. Bu yüzden DSA, siz istemedikçe Android'i hiçbir zaman
dondurmaz.

- Hemen dondurmak için: **General** sayfasında **Freeze Android now** altındaki **Freeze** veya
  `dsa freeze`.
- Doze'da 30 dakika kaldıktan sonra dondurmak için: **Advanced** (gelişmiş) sayfasında **Freeze
  Android when idle** ayarını açın veya `dsa settings set freeze on`.
- Bir Android penceresini açmak veya birinde giriş yapmak Android'i sürdürür.

## Bellek nasıl geri döner

- Android sanal makinesi bilgisayarınızdan belleği yalnızca Android onu ilk kez kullandığında alır.
- Android'in serbest bıraktığı bellek, Android çalışırken bilgisayarınıza geri verilir.
- Bir uygulamanın son penceresi kapandığında Android o uygulamanın işlemlerini sonlandırır, böylece
  bellekleri saniyeler içinde geri döner. Masaüstünde pencere öğesi olan bir uygulama veya arka
  planda hâlâ çalışan bir uygulama (örneğin bir müzik çalar) çalışmaya devam eder.
- Penceresi, bildirimi, sesi ve pencere öğesi olmadan bir arka plan hizmetini çalışır tutan bir
  uygulama beş dakika sonra durdurulur. Android onu tekrar tekrar başlatırsa DSA her seferinde daha
  uzun, en fazla bir saate kadar bekler.
- Hiçbir uygulama penceresi açık değilken Android dosya önbelleklerini bırakır ve boştaki belleği
  sıkıştırılmış belleğe taşır, böylece daha fazlası geri dönebilir.
- Pencere yöneticisi, son uygulama penceresi kapandıktan 30 saniye sonra kullanmadığı belleği geri
  verir.

## Bellek sınırı

Sınır varsayılan olarak **Automatic** (otomatik) ayarlıdır; bu, sabit bir sınır koymaz. Android 2 GB
ile başlar ve çalışırken büyür; en fazla bilgisayarınızın belleğinden Windows'un ayırdığı kısım
(belleğin %10'u veya 2 GB, hangisi büyükse) çıkarılana kadar. Android yalnızca kullandığını tutar.
Bir sınır koymak için **Memory** sayfasındaki **Memory limit** ayarını kullanın veya:

```
dsa settings set memory-limit 4g     # 4 GiB
dsa settings set memory-limit auto   # otomatiğe döner
```

Yeni bir sınır, Android'in bir sonraki başlatılmasında uygulanır.

---

Android, Google LLC'nin ticari markasıdır. Windows, Microsoft şirketler grubunun ticari markasıdır.
DSA, Google veya Microsoft ile bağlantılı değildir, onlar tarafından onaylanmamış ve
desteklenmemektedir.
