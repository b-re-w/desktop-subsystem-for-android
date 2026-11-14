# Dosyalar

[English](../files.md) · [한국어](../ko/files.md) · [日本語](../ja/files.md) · [简体中文](../zh-CN/files.md) · [繁體中文](../zh-TW/files.md) · [Español](../es/files.md) · [Français](../fr/files.md) · [Deutsch](../de/files.md) · [Português (Brasil)](../pt-BR/files.md) · [Русский](../ru/files.md) · [Italiano](../it/files.md) · [Tiếng Việt](../vi/files.md) · [Bahasa Indonesia](../id/files.md) · **Türkçe**

Desktop Subsystem for Android™ (DSA) ile Android dosyalarını Dosya Gezgini'nde açabilir, Windows
klasörlerinizi Android uygulamalarıyla paylaşabilir ve bir Android uygulamasına dosya
gönderebilirsiniz. Bu sayfa Windows'taki DSA'yı anlatır.

## Android dosyalarını Dosya Gezgini'nde açma

Bunlardan herhangi birini kullanın. Her biri, Android çalışmıyorsa onu başlatır.

- Dosya Gezgini'nin gezinti bölmesinde, WSL'nin **Linux** girdisinin hemen altındaki **Android**.
- DSA Settings'in **Advanced** (gelişmiş) sayfasındaki **Android files**.
- `dsa files open`. `dsa files`, depolamanın nerede olduğunu açmadan söyler.

Android'in paylaşılan depolamasını görürsünüz: fotoğraflar ve indirilenler gibi Android
uygulamalarının kaydettiği dosyalar. Dosyaları her klasörde olduğu gibi içeri ve dışarı
kopyalayabilirsiniz. Dosya Gezgini depolamaya Windows'un WebDAV istemcisi üzerinden ulaşır.
Depolama yanıt vermezse DSA nedenini belirten bir hata gösterir.

## Windows klasörlerinizi Android uygulamalarıyla paylaşma

Belgeler, İndirilenler, Resimler, Müzik, Videolar ve Masaüstü klasörleriniz, Android'in paylaşılan
depolamasında `Host` altında görünür, örneğin `Host/Pictures`. Android uygulamaları orada dosya
açabilir ve kaydedebilir. Bir uygulamanın önce Android'in depolama iznine sahip olması gerekir ve
Windows ona yalnızca kendi hesabınızın yapabildiklerini yaptırır.

Paylaşım varsayılan olarak açıktır. Bunu değiştirmek için DSA Settings'in **Advanced** sayfasını
açın:

- Hiçbir klasörü paylaşmamak için **Share Windows folders with Android** ayarını kapatın.
- **Shared folders** altında bir klasörün paylaşımını durdurmak için işaretini kaldırın. Klasör
  Android'den hemen çıkar.

Komut satırından:

```
dsa settings set shared-folders Documents,Pictures   # yalnızca bunlar; all veya none da olur
dsa settings set file-sharing off                     # hiçbir klasörü paylaşmaz
```

Önceki sürümler `Host` yerine `Windows` adlı bir klasör kullanıyordu. Android eski klasörü boşaldığı
anda kaldırır, bu yüzden oraya kaydettiğiniz dosyalar kalır.

## Bir Android uygulamasına dosya gönderme

- **Sürükleyip bırakma.** Dosya Gezgini'nden bir dosyayı bir uygulamanın penceresine sürükleyin.
  Dosya o uygulamaya gider. Bir APK ise APK Installer'a gider. Klasörler gönderilmez.
- **Bağlam menüsü.** Bir dosyaya sağ tıklayıp **Open in an Android app** veya **Share with an
  Android app** öğesini seçin. Windows 11'de bunlar **Diğer seçenekleri göster** altındadır.
  Ardından Android, dosyayı hangi uygulamanın alacağını sorar.
- **Komut satırı.** `dsa open <file>` ve `dsa share <file>`.

Bunlar 64 MiB'ye kadar dosya gönderir. Daha büyük bir dosya, hiçbir şey gönderilmeden reddedilir.
Bir uygulamaya daha büyük bir dosya vermek için onu bunun yerine paylaşılan bir klasöre koyun.

## Android'den dosya alma

- Dosyayı Android'de `Host/<folder>` altına kaydedin. Ardından bilgisayarınızda o klasörde olur.
- Ya da Dosya Gezgini'nde **Android**'i açıp dosyayı dışarı kopyalayın.

---

Android, Google LLC'nin ticari markasıdır. Windows, Microsoft şirketler grubunun ticari markasıdır.
DSA, Google veya Microsoft ile bağlantılı değildir, onlar tarafından onaylanmamış ve
desteklenmemektedir.
