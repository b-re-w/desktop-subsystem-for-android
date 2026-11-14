# Ayarlar

[English](../settings.md) · [한국어](../ko/settings.md) · [日本語](../ja/settings.md) · [简体中文](../zh-CN/settings.md) · [繁體中文](../zh-TW/settings.md) · [Español](../es/settings.md) · [Français](../fr/settings.md) · [Deutsch](../de/settings.md) · [Português (Brasil)](../pt-BR/settings.md) · [Русский](../ru/settings.md) · [Italiano](../it/settings.md) · [Tiếng Việt](../vi/settings.md) · [Bahasa Indonesia](../id/settings.md) · **Türkçe**

**DSA Settings** (DSA ayarları), Windows'taki Desktop Subsystem for Android™ (DSA) yazılımının
ayarlar uygulamasıdır. Başlat menüsünden açın. Windows görüntüleme dilinize göre İngilizce veya
Korece görünür ve Windows temanızı izler. Klavyeyle ve ekran okuyucularla çalışır.

DSA Settings'in yaptığı her değişiklik `dsa` komutundan geçer, böylece komut satırı da aynı
değerleri görür:

```
dsa settings                        # her ayar ve değeri
dsa settings set <name> <value>     # birini değiştirir; anahtarlar on veya off alır
```

Bazı ayarlar bilgisayarın tüm kullanıcıları için geçerlidir, bu yüzden onları yalnızca bir
yönetici değiştirebilir. DSA Settings bunları bir kalkan simgesiyle işaretler ve birini
değiştirdiğinizde yönetici onayı ister. Aşağıda "(admin)" ile işaretlenmişlerdir.

## Sayfalar

| Sayfa | Orada bulduklarınız | Komut satırı |
|---|---|---|
| **General** | Android'in durumu, **Start**, **Stop**, **Freeze Android now**, son sorun, **Open my log folder**, **Refresh** | `dsa status`, `dsa start`, `dsa stop`, `dsa freeze` |
| **Memory** | **Memory limit**, Android sanal makinesinin boyutu, her parçanın kullandığı | `memory-limit`, `dsa memory` |
| **Apps** | Yüklü uygulamalar: **Open**, **App info**, **Uninstall**, **Create desktop shortcut**. Her uygulama için: **Resizable**, pencerelerin açıldığı boyut, **Scaling on another monitor**, **Full screen**, **Bars hidden**, **Run as ARM**, **Root access**. **adb shell**'in kök erişimi için bir satır | `dsa apps`, `dsa app-window`, `dsa apps run-as-arm`, `dsa apps root` |
| **Running in the background** | Penceresi olmadan çalışan uygulamalar: **Open**, **Stop**; durdurmadan önce sorulup sorulmayacağı | `dsa background`, `stop-without-asking` |
| **Widgets** | Masaüstünüzdeki pencere öğeleri ve **Add to desktop** | `dsa widgets` |
| **Display & GPU** | **Render GPU**, **GPU of the primary monitor**, **App full screen**, **Apps that hide their bars**, **Turn off Windows touch gestures in DSA app windows** (yalnızca Windows 11, varsayılan olarak açık), **Restart the window manager** | `render-gpu`, `dsa gpus`, `app-fullscreen`, `app-immersive`, `touch-gestures-off` |
| **Device identity** | Önce bu bilgisayar, salt okunur: adı, üreticisi ve modeli, **Copy** ile birlikte **Device specifications**. Ardından Android'in uygulamalara bildirdiği üretici, model ve derleme parmak izi | `dsa settings device`, `device.<field>` |
| **Network** | Android'in ağa nasıl ulaştığı, adresi, adlarını kimin yanıtladığı (DNS) ve ağ geçidi, **Advanced networking**, **Use this PC's added root certificates**, **Developer mode** (adb), **Connect adb automatically** | `dsa network`, `advanced-networking`, `host-certificates`, `developer-mode`, `adb-port`, `adb-auto-connect` |
| **Updates** | **Android**: çalışan sürüm ve derleme, güvenlik yaması düzeyi, izlediği sürüm, ilerlemesiyle birlikte Android güncellemesi ve **Check now** (admin), **Restart Android into updates by itself**, **Update the Android image automatically** (admin), **Version to use** (admin). **DSA**: **Update DSA automatically** (admin), **Get pre-release versions** (admin), **Install the newest version of DSA now** | `dsa android version`, `dsa android use`, `image-auto-restart`, `image-updates`, `program-updates`, `prerelease-updates`, `dsa update` |
| **Advanced** | Pano, dosyalar, web bağlantıları, boştayken dondurma, yedekleme, sıfırlama, ARM uygulamaları, Google Play, korumalı içerik, kök erişimi, izlemeler (admin), günlük klasörleri | aşağıya bakın |
| **Logs** | Android'in canlı günlüğü, **Save diagnostics** | `dsa logcat`, `dsa logs collect` |
| **About** | Sürümler, lisanslar, kaynak kodu | |

## Advanced sayfası

| Ayar | Varsayılan | Komut satırı |
|---|---|---|
| **Share the clipboard with Android** | Açık | `clipboard` |
| **Share Windows folders with Android** | Açık | `file-sharing` |
| **Shared folders** | Altısı da | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`, `dsa share` |
| **Freeze Android when idle** | Kapalı | `freeze` |
| **Open web links in Android apps** | Kapalı | `links-in-android` |
| **Back up apps' data to this PC**, **Back up now**, **Restore** | Kapalı | `dsa backup on`, `dsa backup now`, `dsa backup restore` |
| **Recovery key**, **Use a recovery key** | | `dsa backup key show`, `dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator** (ARM64'te yok) | | `dsa arm-translation fetch google` |
| **Use the default translator** (ARM64'te yok) | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**, **Remove Google Play** | Yüklü değil | `dsa gapps install`, `dsa gapps remove` |
| **Widevine** (**Protected content** altında) | Yüklü değil | `dsa drm widevine install`, `dsa drm widevine remove` |
| **Let Android apps ask for root** (admin) | Kapalı | `root-elevation` |
| **Input trace** (admin) | Kapalı | `input-trace` |
| **Resize trace** (admin) | Kapalı | `resize-trace` |

Yedekleme, uygulamalarınızın verilerinin şifreli bir kopyasını, Windows'un sizin için sakladığı bir
anahtarla kullanıcı klasörünüzde tutar. Yedeği başka bir bilgisayarda veya Windows yeniden
yüklendikten sonra geri yüklemek için kurtarma anahtarı gerekir, bu yüzden onu bilgisayardan uzak
bir yere not edin. Google Play, Widevine ve kök erişimi [Uygulamalar ve pencereler](apps.md)
sayfasında anlatılır.

## Bir değişiklik ne zaman uygulanır

- **Render GPU**: pencere yöneticisi yeniden başladığında, yani bir sonraki oturum açışınızda veya
  **Restart the window manager** düğmesini seçtiğinizde.
- **Memory limit**, **Device identity**, **Advanced networking**, **Run apps built only for ARM**,
  Google Play, Widevine, kök erişimini açma ve Android sürümü: Android'in bir sonraki
  başlatılmasında. Android'i yeniden başlatmak için **General** sayfasında durdurun, sonra bir
  uygulama açın.
- Bir uygulamanın pencere ayarları: uygulamanın açtığı bir sonraki pencereden itibaren.
- Kök erişimini kapatma dahil diğer her şey: hemen.

## Android adları nasıl bulur

Android uygulamalarının aradığı her adı bilgisayarınız, Windows uygulamalarının kullandığı aynı
çözümleyiciyle yanıtlar (Linux ve macOS'ta sistemin çözümleyicisiyle). Böylece hosts dosyanız,
şirketinizin ad kuralları, bir VPN'in kendi DNS sunucuları ve bir DNS filtresi, bilgisayara olduğu
gibi Android uygulamalarına da uygulanır. Android kendisi bir DNS sunucusuna sormaz.

- **Advanced networking** açıkken de durum aynıdır: bilgisayarınız aynı ağdadır ve adlarını bilir,
  üzerindeki bir VPN veya DNS filtresi de uygulanmaya devam eder.
- Android'in kendi **Private DNS** (özel DNS) ayarı varsayılan olarak kapalıdır, çünkü bilgisayarınızı
  atlayarak bir DNS sunucusuna sorardı. Android'in ayarlarında özel bir DNS sunucusu seçerseniz
  Android onu kullanır.
- DSA bir süre yanıt veremezse Android, DSA yeniden yanıt verebilene kadar ağın DNS sunucularına
  sorar.
- **Network** sayfasının **DNS** satırı ve `dsa network`, şu anda hangisinin yanıtladığını söyler:
  bilgisayarınızın çözümleyicisi veya listeledikleri ağ DNS sunucuları.

## Komut satırının aldığı değerler

- `render-gpu`: `display` (birincil monitörün GPU'su, varsayılan), `integrated`, `discrete` veya
  `dsa gpus`'ın listelediği biçimde bir GPU adının bir parçası.
- `memory-limit`: `auto` (varsayılan: sabit sınır yok, bilgisayarın belleğinden Windows'un
  ayırdığı kısım çıkarılana kadar) veya `4g` ya da `3072` (MiB) gibi bir boyut.
- `shared-folders`: `all`, `none` veya virgülle ayrılmış adlar, örneğin `Documents,Pictures`.
- `adb-port`: 1 ile 65535 arasında bir TCP bağlantı noktası. Varsayılan 58526'dır.
- `app-fullscreen`: bir uygulamanın açık tam ekran isteği için `screen` (varsayılan) veya
  `window`. `app-immersive`: yalnızca çubuklarını gizleyen bir uygulama için `screen` veya
  `window` (varsayılan). Bkz. [Tam ekran](apps.md#tam-ekran).
- `touch-gestures-off`: `on` (varsayılan), bir DSA uygulama penceresi öndeyken Windows 11'in üç ve
  dört parmaklı dokunma hareketlerini kapatır, böylece uygulamalar en fazla on parmak alır; başka
  bir pencere öne geldiğinde kendi ayarınızı geri koyar. Windows reddederse ayar kullanılamaz ve
  ayarlar uygulaması nedenini söyler; hareketleri Windows Ayarları'nda, Bluetooth ve cihazlar >
  Dokunma altında tüm uygulamalar için yine de kapatabilirsiniz.
- `host-certificates`: `on` (varsayılan), Android uygulamalarının bu bilgisayara eklenmiş kök
  sertifikalara (örneğin şirketinizin veya bir güvenlik programının) güvenmesini sağlar.
- `adb-auto-connect`: `on` (varsayılan), geliştirici modu açıkken Android'i `adb connect`
  gerekmeden `adb devices` ve Android Studio'da gösterir.
- `image-auto-restart`: `on` (varsayılan), hiçbir uygulama penceresi açık değilken Android'i
  indirilmiş bir görüntü güncellemesiyle hemen yeniden başlatır. Aksi halde DSA önce sorar veya
  pencereleri kapatmanızı bekler.
- `prerelease-updates`: `on` kararlı sürümlerin yanı sıra ön sürümleri de alır, `off` yalnızca
  kararlı sürümleri alır. Kullandığınız yükleyiciyle başlar: bir ön sürüm yükleyicisi onu açar,
  kararlı bir yükleyici kapalı bırakır ve yükseltmeler seçiminizi korur. 1.0'dan önceki her sürüm
  bir ön sürümdür. Onu kapatmak DSA'yı hiçbir zaman daha eski bir sürüme döndürmez; DSA yüklü
  olandan daha yeni bir kararlı sürümü bekler.
- `root-elevation`: `on`, **Apps** sayfasında izin verdiğiniz uygulamaların kök erişimi istemesini
  sağlar. Her uygulamanın düzeyi `dsa apps root <package> off|ask|remember` ile ayarlanır.
- `device.<field>`: `manufacturer`, `brand`, `model`, `device`, `name`, `characteristics`,
  `fingerprint` veya parmak izinin `fingerprint.release` gibi bir parçası.
  `dsa settings reset device` her değeri geri koyar.

---

Android, Google LLC'nin ticari markasıdır. Windows, Microsoft şirketler grubunun ticari markasıdır.
DSA, Google veya Microsoft ile bağlantılı değildir, onlar tarafından onaylanmamış ve
desteklenmemektedir.
