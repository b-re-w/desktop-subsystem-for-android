# Başlarken

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · [日本語](../ja/getting-started.md) · [简体中文](../zh-CN/getting-started.md) · [繁體中文](../zh-TW/getting-started.md) · [Español](../es/getting-started.md) · [Français](../fr/getting-started.md) · [Deutsch](../de/getting-started.md) · [Português (Brasil)](../pt-BR/getting-started.md) · [Русский](../ru/getting-started.md) · [Italiano](../it/getting-started.md) · [Tiếng Việt](../vi/getting-started.md) · [Bahasa Indonesia](../id/getting-started.md) · **Türkçe**

Desktop Subsystem for Android™ (DSA), Android'i kendi çekirdeğine sahip hafif bir sanal makinede
çalıştırır. Her Android uygulaması masaüstünüzde kendi penceresinde açılır.

## Platformlara göre durum

| Ana bilgisayar | Durum |
|---|---|
| Windows 11, x64 | Yayımlandı. |
| Windows 11, ARM64 | Yayımlandı. Android de ARM64 olduğundan ARM uygulamaları çeviri olmadan çalışır. |
| Linux, x64 ve ARM64 | Üzerinde çalışılıyor. `.deb` ve `.rpm` paketleri CI'da derlenir; henüz yayımlanmadı. |
| macOS, Apple silicon | Üzerinde çalışılıyor. `DSA.app`, onun `.pkg` ve `.dmg` dosyaları CI'da derlenir; henüz yayımlanmadı. |

## Windows'a yükleme

x64 veya ARM64 bir bilgisayarda Windows 11 ve açık olan **Sanal Makine Platformu** (Virtual
Machine Platform) Windows özelliği gerekir.

1. Sanal Makine Platformu'nu açın. **Windows özelliklerini aç veya kapat**'ı açın, **Sanal Makine
   Platformu**'nu işaretleyin, **Tamam**'ı seçin ve yeniden başlatın. Ya da yönetici terminalinde
   `dism /online /enable-feature /featurename:VirtualMachinePlatform /all` komutunu çalıştırın.
   Kurulum bunu sizin için açmaz: özellik kapalıysa bir iletiyle durur.
2. Kurulum programını ve `SHA256SUMS.txt` dosyasını
   [en son sürümden](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest)
   indirin: x64 bilgisayarda `DSA-v<version>-win-x64-setup.exe`, ARM64 bilgisayarda
   `DSA-v<version>-win-arm64-setup.exe`.
3. İndirilen dosyayı denetleyin. Bu komutun yazdırdığı özet, `SHA256SUMS.txt` içindeki satırla
   eşleşmelidir:

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. Kurulumu çalıştırın, **Install** (yükle) düğmesini seçin ve yönetici istemini onaylayın.
5. İlk Android görüntüsünü bekleyin. Kurulum bir görüntü içermiyorsa DSA hizmeti onu arka planda
   indirir. `dsa status` indirmeyi gösterir.
6. Başlat menüsünden bir uygulama açın veya terminalden denetleyin:

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## Kurulumun yükledikleri

- Her kullanıcının Android'ini çalıştıran `DsaHost` hizmeti.
- `dsa` komutu.
- Başlat menüsünde **DSA Settings** (DSA ayarları).
- Dosya Gezgini'nin gezinti bölmesinde, Android dosyalarını açan **Android**.
- `.apk` dosyalarını açan APK Installer (APK yükleyicisi).
- DSA pencere yöneticisi (`dsa-wm`). Oturum açtığınızda başlar ve Android'i de o zaman başlatır,
  böylece açtığınız ilk uygulama Android'in önyüklemesini beklemez.

Her Windows kullanıcısının kendi Android'i ve kendi veri diski vardır:
`%LOCALAPPDATA%\DSA\android.vhdx`. Diğer kullanıcılar onu okuyamaz.

DSA kendi çekirdeğini ve kendi sanal makinesini kullanır. WSL ayarlarınızı, çekirdeğinizi veya
dağıtımlarınızı kullanmaz ve değiştirmez.

## Güncellemeler

DSA kendisini ve Android görüntüsünü günceller. İkisi de imzalıdır ve DSA imzası, boyutu veya
özeti eşleşmeyen hiçbir şeyi uygulamaz.

- **DSA'nın kendisi**, hiçbir kullanıcının Android'i çalışmıyorken güncellenir. O zamana kadar
  güncelleme bekler; `dsa update` onu hemen yükler. DSA hiçbir zaman daha düşük bir sürüme geçmez.
- **Sürüm kanalları.** 1.0'dan itibaren DSA'nın bir kararlı kanalı ve hâlâ tamamlanmakta olan
  sürümler için bir ön sürüm kanalı vardır. Kanalınız kullandığınız yükleyiciyle başlar ve
  yükseltmeler onu korur. Bir yönetici bunu DSA Settings'in **Updates** (güncellemeler)
  sayfasındaki **Get pre-release versions** ile değiştirir. 1.0'dan önceki her sürüm bir ön
  sürümdür.
- **Android görüntüsü** kanala bağlı değildir. Bir görüntü önyükleme yapamazsa DSA bir öncekine
  döner ve `dsa status` bunu bildirir.

Bir yönetici, güncellemeleri DSA Settings'in **Updates** sayfasında kapatabilir. Her anahtar için
bkz. [Ayarlar](settings.md).

## Linux (üzerinde çalışılıyor, yayımlanmadı)

Linux için DSA, bir `.deb` (Debian, Ubuntu) ve bir `.rpm` (Fedora, openSUSE) olarak derlenir.
KVM (`/dev/kvm`) ve bir Wayland masaüstü gerektirir. Android bir crosvm sanal makinesinde çalışır.

Bir paketi yüklemek, DSA'nın imzalı paket deposunu da ekler; böylece sonraki sürümler `apt`,
`dnf`, `zypper` veya yazılım merkezi üzerinden gelir. DSA'nın Linux'ta kendi güncelleyicisi yoktur.

- Debian ve Ubuntu: `/etc/apt/sources.list.d/dsa.sources`, anahtar
  `/usr/share/keyrings/dsa-archive-keyring.gpg` içinde.
- Fedora ve openSUSE: `/etc/yum.repos.d/dsa.repo` (zypper bulunan yerlerde
  `/etc/zypp/repos.d/dsa.repo` de), anahtar `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa` içinde.

Depo `https://thisisthepy.github.io/desktop-subsystem-for-android/` adresinden sunulacak. Henüz
yayımlanmadı.

Yayımlandığında ilk yükleme şöyle görünür:

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian, Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

Paket; `dsa-hostd` sistem hizmetini, `dsa-devices` ve `dsa-compositor` kullanıcı hizmetlerini,
`/usr/bin` içindeki `dsa` komutunu, **DSA Files** menü girdisini ve **Open in an Android app**,
**Share with an Android app** ile **Install in Android** dosya eylemlerini yükler.

## macOS (üzerinde çalışılıyor, yayımlanmadı)

macOS için DSA, Android'i Apple silicon üzerinde Apple'ın Virtualization framework'üyle çalıştırır.
`DSA.app` olarak, bir yükleyici paketinde (`DSA-<version>.pkg`) ve bir disk görüntüsünde
(`DSA-<version>.dmg`) derlenir. CI derlemeleri geçici (ad hoc) olarak imzalanır, bu yüzden yalnızca
onları derleyen makinede çalışır. Developer ID ile imzalanmış bir sürüm henüz yoktur.

## Sonraki adımlar

- [Uygulamalar ve pencereler](apps.md)
- [APK yükleme](install-apks.md)

---

Android, Google LLC'nin ticari markasıdır. Windows, Microsoft şirketler grubunun ticari markasıdır.
macOS, Apple Inc.'in ticari markasıdır. DSA; Google, Microsoft veya Apple ile bağlantılı değildir,
onlar tarafından onaylanmamış ve desteklenmemektedir.
