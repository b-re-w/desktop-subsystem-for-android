# APK yükleme

[English](../install-apks.md) · [한국어](../ko/install-apks.md) · [日本語](../ja/install-apks.md) · [简体中文](../zh-CN/install-apks.md) · [繁體中文](../zh-TW/install-apks.md) · [Español](../es/install-apks.md) · [Français](../fr/install-apks.md) · [Deutsch](../de/install-apks.md) · [Português (Brasil)](../pt-BR/install-apks.md) · [Русский](../ru/install-apks.md) · [Italiano](../it/install-apks.md) · [Tiếng Việt](../vi/install-apks.md) · [Bahasa Indonesia](../id/install-apks.md) · **Türkçe**

Desktop Subsystem for Android™ (DSA) bir uygulama mağazası içermez. Uygulamaları APK dosyalarından,
APK Installer veya `dsa` komutuyla yüklersiniz. Bu sayfa Windows'taki DSA'yı anlatır.

## APK Installer ile yükleme

1. Dosya Gezgini'nde bir `.apk` dosyasına çift tıklayın. Dosya **APK Installer**'da (APK
   yükleyicisi) açılır.
2. APK'nin ne olduğunu denetleyin. Pencere bunu Android'i başlatmadan gösterir:
   - uygulamanın adı, simgesi, paketi ve sürümü;
   - imzalama sertifikasından alınan yayımcı, çünkü bir APK yayımcı adı belirtmez;
   - kaynak: dosyanın indirildiği ana bilgisayar veya klasörü;
   - **Capabilities** altında, istediği ilk izinler. **More** hepsini, hangi Android sürümleri için
     yapıldığını, boyutunu, işlemci türlerini ve sertifikasının SHA-256 değerini gösterir.

   APK yalnızca başka bir işlemci türü için kod içeriyorsa pencere sizi uyarır.
3. **Install** düğmesini seçin. Uygulama zaten yüklüyse düğmede **Update** veya **Install again**
   yazar. Android çalışmıyorsa önce o başlar.
4. **Launch when ready** işaretliyken (varsayılan), uygulama yüklenir yüklenmez açılır ve yükleyici
   kapanır. Aksi halde **Launch** düğmesini seçin veya uygulamayı daha sonra Başlat menüsünden açın.

Android çalışırken pencere ayrıca dosyanın yeni bir yükleme mi, bir güncelleme mi, aynı sürümün
yeniden yüklenmesi mi yoksa Android'in reddettiği bir şey mi (daha eski bir sürüm veya başka bir
anahtarla imzalanmış aynı uygulama) olduğunu söyler.

APK dosyalarını APK Installer'a, DSA Settings'e veya bir uygulamanın penceresine de
bırakabilirsiniz. Her APK kendine ait bir yükleyici penceresinde açılır.

## Komut satırından yükleme

```
dsa apk-info app.apk            # APK'nin ne olduğu; dosyayı Android'i başlatmadan okur
dsa install app.apk             # yükler; Android çalışmıyorsa başlatır
dsa install app.apk --progress  # her adımı da yazdırır
```

Android APK'yi reddettiğinde `dsa install` 1 koduyla çıkar ve iletisi `not installed (<reason>):`
ile başlar.

## Bir yükleme neden reddedilir

| Neden | Anlamı | Ne yapmalı |
|---|---|---|
| `incompatible` | Uygulama başka bir işlemci türü, daha yeni bir Android sürümü veya bu cihazda olmayan bir özellik gerektirir | Bkz. [Yalnızca ARM için derlenmiş uygulamalar](apps.md#yalnızca-arm-için-derlenmiş-uygulamalar) veya [Android sürümleri](android-versions.md) |
| `conflicting-signature` | Uygulama zaten yüklü ve başka bir anahtarla imzalanmış | Önce yüklü uygulamayı kaldırın |
| `downgrade` | Uygulamanın daha yeni bir sürümü yüklü | Önce yüklü uygulamayı kaldırın |
| `invalid-apk` | Dosya, Android'in okuyabildiği imzalı bir APK değil | APK'yi yeniden edinin |
| `insufficient-storage` | Android'de uygulama için yer yok | Android'de yer açın |
| `blocked` | Bu Android'in bir ilkesi uygulamaya izin vermiyor | |
| `other` | Diğer her şey; ardından Android'in kendi iletisi gelir | |

Bir APK en fazla 2 GiB olabilir. Yüklemeler birer birer çalışır: ikincisi birincisini bekler.

## Bir uygulamayı kaldırma

Bunlardan herhangi birini kullanın:

- Windows **Ayarlar** > **Uygulamalar** > **Yüklü uygulamalar**.
- DSA Settings'in **Apps** (uygulamalar) sayfası: uygulamanın yanındaki **Uninstall**.
- `dsa apps uninstall <package>`.

Android'in parçası olan uygulamalar kaldırılamaz.

---

Android, Google LLC'nin ticari markasıdır. Windows, Microsoft şirketler grubunun ticari markasıdır.
DSA, Google veya Microsoft ile bağlantılı değildir, onlar tarafından onaylanmamış ve
desteklenmemektedir.
