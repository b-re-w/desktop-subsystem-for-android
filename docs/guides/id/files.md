# File

[English](../files.md) · [한국어](../ko/files.md) · [日本語](../ja/files.md) · [简体中文](../zh-CN/files.md) · [繁體中文](../zh-TW/files.md) · [Español](../es/files.md) · [Français](../fr/files.md) · [Deutsch](../de/files.md) · [Português (Brasil)](../pt-BR/files.md) · [Русский](../ru/files.md) · [Italiano](../it/files.md) · [Tiếng Việt](../vi/files.md) · **Bahasa Indonesia** · [Türkçe](../tr/files.md)

Desktop Subsystem for Android™ (DSA) memungkinkan Anda membuka file Android di File Explorer,
membagikan folder Windows Anda dengan aplikasi Android, dan mengirim file ke aplikasi Android.
Halaman ini menjelaskan DSA di Windows.

## Membuka file Android di File Explorer

Gunakan salah satu cara berikut. Masing-masing memulai Android jika belum berjalan.

- **Android** di panel navigasi File Explorer, tepat di bawah **Linux** milik WSL.
- **Android files** di halaman **Advanced** (lanjutan) DSA Settings.
- `dsa files open`. `dsa files` memberitahukan lokasi penyimpanan, tanpa membukanya.

Anda melihat penyimpanan bersama Android: file yang disimpan aplikasi Android, seperti foto
dan unduhan. Anda dapat menyalin file masuk dan keluar seperti di folder mana pun. File
Explorer menjangkau penyimpanan melalui klien WebDAV Windows. Jika penyimpanan tidak merespons,
DSA menampilkan kesalahan beserta alasannya.

## Membagikan folder Windows Anda dengan aplikasi Android

Folder Dokumen, Unduhan, Gambar, Musik, Video, dan Desktop Anda tampil di penyimpanan bersama
Android di bawah `Host`, misalnya `Host/Pictures`. Aplikasi Android dapat membuka dan menyimpan
file di sana. Aplikasi memerlukan izin penyimpanan Android terlebih dahulu, dan Windows hanya
mengizinkannya melakukan apa yang boleh dilakukan akun Anda sendiri.

Berbagi aktif secara bawaan. Untuk mengubahnya, buka halaman **Advanced** DSA Settings:

- Matikan **Share Windows folders with Android** agar tidak ada folder yang dibagikan.
- Di bawah **Shared folders**, hapus centang sebuah folder untuk berhenti membagikannya. Folder
  itu langsung hilang dari Android.

Dari baris perintah:

```
dsa settings set shared-folders Documents,Pictures   # hanya folder ini; juga all atau none
dsa settings set file-sharing off                     # tidak membagikan folder apa pun
```

Versi sebelumnya menggunakan folder bernama `Windows` alih-alih `Host`. Android menghapus
folder lama itu setelah kosong, sehingga file yang Anda simpan di sana tetap ada.

## Mengirim file ke aplikasi Android

- **Seret dan jatuhkan.** Seret file dari File Explorer ke jendela aplikasi. File tersebut
  masuk ke aplikasi itu. APK masuk ke APK Installer. Folder tidak dikirim.
- **Menu konteks.** Klik kanan file dan pilih **Open in an Android app** atau **Share with an
  Android app**. Di Windows 11, keduanya ada di bawah **Tampilkan opsi lainnya**. Android lalu
  menanyakan aplikasi mana yang menerima file tersebut.
- **Baris perintah.** `dsa open <file>` dan `dsa share <file>`.

Cara-cara ini mengirim file hingga 64 MiB. File yang lebih besar ditolak sebelum apa pun
dikirim. Untuk memberikan file yang lebih besar ke aplikasi, letakkan file itu di folder
bersama.

## Mengambil file dari Android

- Simpan file di Android di bawah `Host/<folder>`. File itu kemudian ada di folder tersebut di
  PC Anda.
- Atau buka **Android** di File Explorer dan salin file keluar.

---

Android adalah merek dagang Google LLC. Windows adalah merek dagang grup perusahaan Microsoft.
DSA tidak berafiliasi dengan, tidak didukung oleh, dan tidak disponsori oleh Google maupun
Microsoft.
