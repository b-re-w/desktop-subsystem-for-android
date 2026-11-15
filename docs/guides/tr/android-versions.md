# Android sürümleri

[English](../android-versions.md) · [한국어](../ko/android-versions.md) · [日本語](../ja/android-versions.md) · [简体中文](../zh-CN/android-versions.md) · [繁體中文](../zh-TW/android-versions.md) · [Español](../es/android-versions.md) · [Français](../fr/android-versions.md) · [Deutsch](../de/android-versions.md) · [Português (Brasil)](../pt-BR/android-versions.md) · [Русский](../ru/android-versions.md) · [Italiano](../it/android-versions.md) · [Tiếng Việt](../vi/android-versions.md) · [Bahasa Indonesia](../id/android-versions.md) · **Türkçe**

Desktop Subsystem for Android™ (DSA) en yeni iki Android ana sürümünü destekler: bugün Android 16
ve Android 17. Varsayılan olarak en yenisini çalıştırır. Bazı uygulamalar bunlardan yalnızca
birinde çalışır, bu yüzden seçim yapabilirsiniz.

## Hangi sürümün çalıştığını görme

DSA Settings'in **Updates** (güncellemeler) sayfasını açın. **Android** altında sürümü, derlemesini,
güvenlik yaması düzeyini, izlediği sürümü ve güncellemesinin durumunu indirmenin ilerlemesiyle
birlikte gösterir. **Check now**, hizmetten hemen bir güncelleme aramasını ister. Ya da şunu
çalıştırın:

```
dsa android version   # çalışan sürüm ve seçebileceğiniz sürümler
```

## Sürümü değiştirme

Bilgisayarın tüm kullanıcıları aynı Android sürümünü çalıştırır, bu yüzden onu yalnızca bir
yönetici değiştirebilir.

1. DSA Settings'in **Updates** sayfasını açın.
2. **Version to use** altında bir sürüm veya **Always the newest** seçin.
3. **Change** düğmesini seçin ve yönetici istemini onaylayın.

Seçilen sürüm indirilir ve Android'in bir sonraki başlatılmasından itibaren kullanılır. Yönetici
terminalinden:

```
dsa android use 16
dsa android use latest
```

### Daha eski bir sürüme geçmek verileri siler

Android, daha yeni bir sürümün yazdığı verileri okuyamaz. Bu nedenle daha eski bir ana sürüme
geçmek, bilgisayarın tüm kullanıcılarının Android verilerini siler: yükledikleri uygulamaları,
hesaplarını ve tüm uygulama verilerini. DSA'nın ayarları kalır. Bu işlem geri alınamaz.

DSA Settings, **Erase data and change** ile onaylamanızı ister. Komut satırında `--erase-data`
ekleyin:

```
dsa android use 16 --erase-data
```

Daha yeni bir sürüme geçmek uygulamalarınızı ve verilerinizi korur.

## Güncellemeler

- Seçilen sürüm güvenlik güncellemeleri almaya devam eder.
- Bir sürüm desteğin dışına çıktığında DSA bir sonrakine geçer ve uygulamalarınızı korur.
- DSA kendi başına hiçbir zaman daha eski bir ana sürüme geçmez.
- Her görüntü, kullanılmadan önce imzalı bir listeyle karşılaştırılır. Önyükleme yapamayan bir
  görüntü bir öncekiyle değiştirilir.

Bir yönetici, görüntü güncellemelerini DSA Settings'in **Updates** sayfasındaki **Update the
Android image automatically** ile veya `dsa settings set image-updates off` ile kapatabilir.

---

Android, Google LLC'nin ticari markasıdır. Windows, Microsoft şirketler grubunun ticari markasıdır.
DSA, Google veya Microsoft ile bağlantılı değildir, onlar tarafından onaylanmamış ve
desteklenmemektedir.
