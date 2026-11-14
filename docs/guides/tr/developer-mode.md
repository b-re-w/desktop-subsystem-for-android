# Geliştirici modu ve adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · [日本語](../ja/developer-mode.md) · [简体中文](../zh-CN/developer-mode.md) · [繁體中文](../zh-TW/developer-mode.md) · [Español](../es/developer-mode.md) · [Français](../fr/developer-mode.md) · [Deutsch](../de/developer-mode.md) · [Português (Brasil)](../pt-BR/developer-mode.md) · [Русский](../ru/developer-mode.md) · [Italiano](../it/developer-mode.md) · [Tiếng Việt](../vi/developer-mode.md) · [Bahasa Indonesia](../id/developer-mode.md) · **Türkçe**

Geliştirici modu, bilgisayarınızdaki Android Debug Bridge'in (adb), Desktop Subsystem for
Android™ (DSA) içindeki Android'inize bağlanmasını sağlar. Varsayılan olarak kapalıdır.

adb, DSA'nın parçası değildir. Onu Android SDK Platform-Tools'tan edinin.

## adb'yi bağlama

1. DSA Settings'in **Network** (ağ) sayfasını açın ve **Developer mode** ayarını açın. Ya da
   `dsa settings set developer-mode on` komutunu çalıştırın.
2. Android çalışmıyorsa başlatın: `dsa start --wait`.
3. Bağlanın:

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   Android bağlantıya izin vermenizi istemez. Bağlantı noktası yalnızca bu bilgisayarda yanıt
   verir, bu yüzden diğer bilgisayarlar bağlanamaz.
4. İşiniz bittiğinde **Developer mode** ayarını yeniden kapatın veya
   `dsa settings set developer-mode off` komutunu çalıştırın.

**Network** sayfasındaki **Connect adb** tam komutu gösterir ve `dsa settings` onu
`Developer mode` satırında yazdırır.

## Başka bir bağlantı noktası kullanma

Varsayılan bağlantı noktası 58526'dır. Değiştirmek için **Connect adb** altındaki **Port** alanını
düzenleyin veya şunu çalıştırın:

```
dsa settings set adb-port 5555
```

## Geliştirici modunun değiştirmedikleri

DSA kendi işi için hiçbir zaman adb kullanmaz. Pencereler, giriş ve yüklemeler DSA'nın kendi denetim
kanalını kullanır, bu yüzden geliştirici modu kapalıyken de her şey aynı şekilde çalışır.

Android'in günlüğünü okumak için adb gerekmez. Bkz. [Sorun giderme](troubleshooting.md).

---

Android, Google LLC'nin ticari markasıdır. DSA, Google ile bağlantılı değildir, Google tarafından
onaylanmamış ve desteklenmemektedir.
