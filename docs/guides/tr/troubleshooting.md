# Sorun giderme ve günlükler

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · [日本語](../ja/troubleshooting.md) · [简体中文](../zh-CN/troubleshooting.md) · [繁體中文](../zh-TW/troubleshooting.md) · [Español](../es/troubleshooting.md) · [Français](../fr/troubleshooting.md) · [Deutsch](../de/troubleshooting.md) · [Português (Brasil)](../pt-BR/troubleshooting.md) · [Русский](../ru/troubleshooting.md) · [Italiano](../it/troubleshooting.md) · [Tiếng Việt](../vi/troubleshooting.md) · [Bahasa Indonesia](../id/troubleshooting.md) · **Türkçe**

Bu sayfa, Desktop Subsystem for Android™ (DSA) çalışmadığında nedenini bulmanıza, Android'in
günlüğünü okumanıza ve bir sorun raporu göndermenize yardımcı olur. Windows'u ve belirtilen
yerlerde Linux'u kapsar.

## Önce durumu denetleyin

```
dsa status          # Android'in durumu, varsa sorunlar ve bekleyen güncellemeler
dsa start --wait    # Android'i başlatır ve önyükleme bitene kadar bekler
```

DSA Settings'in **General** (genel) sayfası aynı durumu gösterir.

| `dsa status` | DSA Settings | Anlamı |
|---|---|---|
| `Active` | **Running** | Android önyüklendi ve içindeki DSA aracısı bağlı |
| `Dozing` | **Dozing (idle)** | Android boşta. Bkz. [Bellek](memory.md) |
| `Frozen` | **Frozen** | Android, pencerelerinden birini açana kadar donduruldu |
| `Stalled` | **Not ready** | Android çalışıyor ancak çok uzun sürdü. Sonraki satır nedenini ve ne yapılacağını söyler |

## Sık karşılaşılan sorunlar

| Sorun | Ne yapmalı |
|---|---|
| Kurulum, Sanal Makine Platformu'nun kapalı olduğunu söylüyor | Windows özelliğini açın, yeniden başlatın ve kurulumu yeniden çalıştırın. Bkz. [Başlarken](getting-started.md) |
| Hiçbir uygulama penceresi açılmıyor | Oturumu kapatıp yeniden açın. Pencere yöneticisi oturum açılırken başlar |
| Bir uygulama yanlış boyutta açılıyor | `dsa app-window <package> reset` |
| Bir görüntü güncellemesinden sonra Android önyükleme yapmıyor | Hiçbir şey. DSA kendi başına bir önceki görüntüye döner ve `dsa status` bunu bildirir |
| Android'in ağı yok | DSA Settings'in **Network** sayfasını açın veya `dsa network` komutunu çalıştırın. Modu, adresi, DNS sunucularını ve varsa sorunu gösterir |
| Android bozuk ve başka hiçbir şey işe yaramıyor | Android'in verilerini sıfırlayın (aşağıda) |

### Android'in verilerini sıfırlama

Sıfırlama, yüklediğiniz her uygulamayı, hesaplarınızı ve tüm uygulama verilerini siler. Ardından
Android ilk yüklendiği haliyle başlar. Android görüntüsü ve DSA'nın ayarları kalır.

- DSA Settings'te: **Advanced** (gelişmiş) sayfası > **Reset Android's data** > **Reset**.
- Komut satırında: `dsa reset-data` neyin kaybolacağını söyler. `dsa reset-data --yes` siler.

## Android'in günlüğünü okuma

Android'in günlüğü Android'de bellekte kalır ve hiçbir zaman bir dosyaya yazılmaz. Bilgisayarınıza
yalnızca bir şey onu okurken ulaşır.

- DSA Settings'te: **Logs** (günlükler) sayfası. Etikete veya metne göre süzebilir ve
  duraklatabilirsiniz.
- Komut satırında:

  ```
  dsa logcat                        # günlüğü izler; Ctrl+C sonlandırır
  dsa logcat -d -b all *:W          # günlüğün şu an tuttukları, uyarılar ve üzeri
  dsa logcat ActivityManager:I *:S  # yalnızca bir etiket
  ```

## Sorun raporu gönderme

1. Tanılama bilgilerini kaydedin. **Logs** sayfasında **Save diagnostics** düğmesini seçin veya şunu
   çalıştırın:

   ```
   dsa logs collect -o diag.zip
   ```

   Bu bir dakika veya daha uzun sürer.
2. Zip dosyasını bir
   [soruna](https://github.com/thisisthepy/desktop-subsystem-for-android/issues) ekleyin.

Arşiv; Android'in tüm günlüğünü, çökme kayıtlarını (tombstones, ANR izleri, dropbox, pstore),
Android sistem hizmetlerinin bir özetini ve bu bilgisayardaki sizi ilgilendiren DSA günlüklerini
diğer kullanıcıların satırları olmadan içerir. Arşivdeki `summary.txt`, nelerin toplanamadığını
söyler.

## Windows'ta günlüklerin yeri

| Klasör | Dosyalar |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log` (pencere yöneticisi), `dsa-app-window.log` (uygulama pencereleri) |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log` (hizmet), `android-<SID>.log` (her kullanıcının Android konsolu, önyükleme başına en fazla 2 MiB), `setup-<version>.log` (kendi kendine güncellemeler) |

Bunları açmak için DSA Settings'in **Advanced** sayfasındaki **Open my log folder** ve **Open the
service's log folder (administrators)** düğmelerini kullanın.

## Linux'ta günlüklerin yeri (henüz yayımlanmadı)

Hizmetler systemd günlüğüne (journal) yazar:

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## Tüm komutlar

Bağımsız değişken olmadan `dsa` her komutu yazdırır. Programların okuyabileceği bir çıktı için
`--json` ekleyin.

---

Android, Google LLC'nin ticari markasıdır. Windows, Microsoft şirketler grubunun ticari markasıdır.
DSA, Google veya Microsoft ile bağlantılı değildir, onlar tarafından onaylanmamış ve
desteklenmemektedir.
