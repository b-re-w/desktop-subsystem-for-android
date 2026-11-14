# Menginstal APK

[English](../install-apks.md) · [한국어](../ko/install-apks.md) · [日本語](../ja/install-apks.md) · [简体中文](../zh-CN/install-apks.md) · [繁體中文](../zh-TW/install-apks.md) · [Español](../es/install-apks.md) · [Français](../fr/install-apks.md) · [Deutsch](../de/install-apks.md) · [Português (Brasil)](../pt-BR/install-apks.md) · [Русский](../ru/install-apks.md) · [Italiano](../it/install-apks.md) · [Tiếng Việt](../vi/install-apks.md) · **Bahasa Indonesia** · [Türkçe](../tr/install-apks.md)

Desktop Subsystem for Android™ (DSA) tidak menyertakan toko aplikasi. Anda menginstal aplikasi
dari file APK, dengan APK Installer atau perintah `dsa`. Halaman ini menjelaskan DSA di
Windows.

## Menginstal dengan APK Installer

1. Klik ganda file `.apk` di File Explorer. File itu terbuka di **APK Installer**.
2. Periksa isi APK tersebut. Jendela menampilkannya tanpa memulai Android:
   - nama, ikon, paket, dan versi aplikasi;
   - penerbit, diambil dari sertifikat penandatanganan, karena APK tidak menyebutkan
     penerbit;
   - sumber: host tempat file diunduh, atau foldernya;
   - di bawah **Capabilities** (kemampuan), izin pertama yang dimintanya. **More** menampilkan
     semuanya, versi Android yang menjadi sasarannya, ukurannya, jenis prosesornya, dan SHA-256
     sertifikatnya.

   Jika APK hanya memiliki kode untuk jenis prosesor lain, jendela akan memperingatkan Anda.
3. Pilih **Install**. Jika aplikasi sudah terinstal, tombol bertuliskan **Update** atau
   **Install again**. Android dimulai terlebih dahulu jika belum berjalan.
4. Dengan **Launch when ready** dicentang (bawaan), aplikasi terbuka setelah terinstal dan
   penginstal ditutup. Jika tidak, pilih **Launch**, atau buka aplikasi nanti dari menu Mulai.

Selama Android berjalan, jendela juga memberitahukan apakah file tersebut instalasi baru,
pembaruan, versi yang sama lagi, atau sesuatu yang ditolak Android: versi yang lebih lama, atau
aplikasi yang sama yang ditandatangani dengan kunci lain.

Anda juga dapat menjatuhkan file APK ke APK Installer, ke DSA Settings, atau ke jendela
aplikasi. Setiap APK terbuka di jendela penginstalnya sendiri.

## Menginstal dari baris perintah

```
dsa apk-info app.apk            # isi APK; membaca file tanpa memulai Android
dsa install app.apk             # instal; memulai Android jika belum berjalan
dsa install app.apk --progress  # juga menampilkan setiap langkah
```

Jika Android menolak APK, `dsa install` keluar dengan kode 1, dan pesannya diawali
`not installed (<reason>):`.

## Mengapa instalasi ditolak

| Alasan | Arti | Yang harus dilakukan |
|---|---|---|
| `incompatible` | Aplikasi memerlukan jenis prosesor lain, versi Android yang lebih baru, atau fitur yang tidak dimiliki perangkat ini | Lihat [Aplikasi yang hanya dibuat untuk ARM](apps.md#aplikasi-yang-hanya-dibuat-untuk-arm), atau [Versi Android](android-versions.md) |
| `conflicting-signature` | Aplikasi sudah terinstal, ditandatangani dengan kunci lain | Copot aplikasi yang terinstal terlebih dahulu |
| `downgrade` | Versi aplikasi yang lebih baru sudah terinstal | Copot aplikasi yang terinstal terlebih dahulu |
| `invalid-apk` | File bukan APK bertanda tangan yang dapat dibaca Android | Dapatkan APK itu lagi |
| `insufficient-storage` | Android tidak memiliki ruang untuk aplikasi tersebut | Kosongkan ruang di Android |
| `blocked` | Kebijakan Android ini tidak mengizinkan aplikasi tersebut | |
| `other` | Hal lainnya; pesan Android sendiri menyusul | |

Ukuran APK dapat mencapai 2 GiB. Instalasi berjalan satu per satu: instalasi kedua menunggu
yang pertama selesai.

## Mencopot aplikasi

Gunakan salah satu cara berikut:

- **Pengaturan** Windows > **Aplikasi** > **Aplikasi yang diinstal**.
- Halaman **Apps** DSA Settings: **Uninstall** di samping aplikasi.
- `dsa apps uninstall <package>`.

Aplikasi yang merupakan bagian dari Android tidak dapat dicopot.

---

Android adalah merek dagang Google LLC. Windows adalah merek dagang grup perusahaan Microsoft.
DSA tidak berafiliasi dengan, tidak didukung oleh, dan tidak disponsori oleh Google maupun
Microsoft.
