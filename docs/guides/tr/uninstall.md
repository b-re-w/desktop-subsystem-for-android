# Kaldırma

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · [日本語](../ja/uninstall.md) · [简体中文](../zh-CN/uninstall.md) · [繁體中文](../zh-TW/uninstall.md) · [Español](../es/uninstall.md) · [Français](../fr/uninstall.md) · [Deutsch](../de/uninstall.md) · [Português (Brasil)](../pt-BR/uninstall.md) · [Русский](../ru/uninstall.md) · [Italiano](../it/uninstall.md) · [Tiếng Việt](../vi/uninstall.md) · [Bahasa Indonesia](../id/uninstall.md) · **Türkçe**

## Windows

Desktop Subsystem for Android™ (DSA) yazılımını iki yoldan biriyle kaldırın:

- Windows **Ayarlar** > **Uygulamalar** > **Yüklü uygulamalar**: Desktop Subsystem for Android'i
  bulun, ardından **Kaldır**'ı seçin.
- Kurulumu yeniden çalıştırın ve **Uninstall** (kaldır) düğmesini seçin.

Kaldırma, oturum açmış olsun ya da olmasın bilgisayarın tüm kullanıcıları için şunları siler:

- program, `DsaHost` hizmeti, `dsa` komutu, DSA Settings ve APK Installer;
- indirilen Android görüntüleri ve güncellemeleri;
- Android uygulamalarının Başlat menüsü girdileri ve **Yüklü uygulamalar** içindeki girdileri;
- Dosya Gezgini'ndeki **Android**, `.apk` dosya ilişkilendirmesi ve bağlam menüsü girdileri;
- DSA'nın ayarları ve günlükleri.

Her kullanıcının Android diskini, `%LOCALAPPDATA%\DSA\android.vhdx` dosyasını **korur**; böylece
sonraki bir yükleme uygulamalarınızı ve verilerinizi yeniden bulur. Verilerinizi de silmek için
kaldırmadan önce `dsa reset-data --yes` komutunu çalıştırın veya sonradan o dosyayı silin.

Klasör yeniden yönlendirmesi bir kullanıcının Başlat menüsünü taşıdıysa, o kullanıcının Android
uygulama girdileri kalır, çünkü kaldırıcı yalnızca varsayılan konuma bakar. Bunları elle silin.

WSL kurulumunuza dokunulmaz, çünkü DSA onu hiçbir zaman değiştirmez.

## Linux (henüz yayımlanmadı)

```
sudo apt remove desktop-subsystem-for-android    # Debian, Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

Paketi kaldırmak, eklediği paket deposu girdisini de kaldırır.

---

Android, Google LLC'nin ticari markasıdır. Windows, Microsoft şirketler grubunun ticari markasıdır.
DSA, Google veya Microsoft ile bağlantılı değildir, onlar tarafından onaylanmamış ve
desteklenmemektedir.
