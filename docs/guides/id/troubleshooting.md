# Pemecahan masalah dan log

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · [日本語](../ja/troubleshooting.md) · [简体中文](../zh-CN/troubleshooting.md) · [繁體中文](../zh-TW/troubleshooting.md) · [Español](../es/troubleshooting.md) · [Français](../fr/troubleshooting.md) · [Deutsch](../de/troubleshooting.md) · [Português (Brasil)](../pt-BR/troubleshooting.md) · [Русский](../ru/troubleshooting.md) · [Italiano](../it/troubleshooting.md) · [Tiếng Việt](../vi/troubleshooting.md) · **Bahasa Indonesia** · [Türkçe](../tr/troubleshooting.md)

Halaman ini membantu Anda mengetahui mengapa Desktop Subsystem for Android™ (DSA) tidak
berfungsi, membaca log Android, dan mengirim laporan masalah. Halaman ini mencakup Windows, dan
Linux jika disebutkan.

## Periksa status terlebih dahulu

```
dsa status          # status Android, masalah apa pun, dan pembaruan yang tertunda
dsa start --wait    # mulai Android dan tunggu sampai selesai boot
```

Halaman **General** (umum) DSA Settings menampilkan status yang sama.

| `dsa status` | DSA Settings | Arti |
|---|---|---|
| `Active` | **Running** | Android sudah boot dan agen DSA di dalamnya terhubung |
| `Dozing` | **Dozing (idle)** | Android sedang diam. Lihat [Memori](memory.md) |
| `Frozen` | **Frozen** | Android dibekukan sampai Anda membuka salah satu jendelanya |
| `Stalled` | **Not ready** | Android berjalan tetapi terlalu lama. Baris berikutnya menjelaskan alasannya dan apa yang harus dilakukan |

## Masalah umum

| Masalah | Yang harus dilakukan |
|---|---|
| Penyiapan menyatakan Platform Mesin Virtual nonaktif | Aktifkan fitur Windows tersebut, mulai ulang, lalu jalankan penyiapan lagi. Lihat [Memulai](getting-started.md) |
| Tidak ada jendela aplikasi yang terbuka | Keluar lalu masuk lagi. Pengelola jendela dimulai saat Anda masuk |
| Aplikasi terbuka dengan ukuran yang salah | `dsa app-window <package> reset` |
| Android tidak boot setelah pembaruan image | Tidak perlu apa-apa. DSA kembali ke image sebelumnya dengan sendirinya, dan `dsa status` memberitahukannya |
| Android tidak memiliki jaringan | Buka halaman **Network** (jaringan) DSA Settings, atau jalankan `dsa network`. Keduanya menampilkan mode, alamat, server DNS, dan masalah apa pun |
| Android rusak dan tidak ada cara lain yang membantu | Reset data Android (di bawah) |

### Reset data Android

Reset menghapus setiap aplikasi yang Anda instal, akun Anda, dan semua data aplikasi. Android
kemudian dimulai seperti saat pertama kali diinstal. Image Android dan pengaturan DSA tetap
ada.

- Di DSA Settings: halaman **Advanced** (lanjutan) > **Reset Android's data** > **Reset**.
- Di baris perintah: `dsa reset-data` memberitahukan apa yang akan hilang.
  `dsa reset-data --yes` menghapusnya.

## Membaca log Android

Log Android tetap berada di memori di dalam Android dan tidak pernah ditulis ke file. Log itu
baru sampai ke PC Anda selama ada yang membacanya.

- Di DSA Settings: halaman **Logs** (log). Anda dapat memfilter berdasarkan tag atau teks, dan
  menjedanya.
- Di baris perintah:

  ```
  dsa logcat                        # ikuti log; Ctrl+C mengakhirinya
  dsa logcat -d -b all *:W          # isi log saat ini, peringatan ke atas
  dsa logcat ActivityManager:I *:S  # hanya satu tag
  ```

## Mengirim laporan masalah

1. Simpan diagnostik. Di halaman **Logs**, pilih **Save diagnostics**, atau jalankan:

   ```
   dsa logs collect -o diag.zip
   ```

   Proses ini memakan waktu satu menit atau lebih.
2. Lampirkan file zip tersebut ke sebuah
   [issue](https://github.com/thisisthepy/desktop-subsystem-for-android/issues).

Arsip ini berisi seluruh log Android, catatan crash-nya (tombstone, jejak ANR, dropbox,
pstore), ringkasan layanan sistem Android, dan log DSA di PC ini yang berkaitan dengan Anda,
tanpa baris milik pengguna lain. `summary.txt` di dalam arsip menyebutkan apa yang tidak dapat
dikumpulkan.

## Lokasi log di Windows

| Folder | File |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log` (pengelola jendela), `dsa-app-window.log` (jendela aplikasi) |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log` (layanan), `android-<SID>.log` (konsol Android setiap pengguna, paling banyak 2 MiB per boot), `setup-<version>.log` (pembaruan mandiri) |

Untuk membukanya, gunakan **Open my log folder** dan **Open the service's log folder
(administrators)** di halaman **Advanced** DSA Settings.

## Lokasi log di Linux (belum dirilis)

Layanan menulis ke journal systemd:

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## Semua perintah

`dsa` tanpa argumen menampilkan semua perintah. Tambahkan `--json` untuk output yang dapat
dibaca program.

---

Android adalah merek dagang Google LLC. Windows adalah merek dagang grup perusahaan Microsoft.
DSA tidak berafiliasi dengan, tidak didukung oleh, dan tidak disponsori oleh Google maupun
Microsoft.
