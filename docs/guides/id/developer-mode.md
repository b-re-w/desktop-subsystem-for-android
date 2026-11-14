# Mode pengembang dan adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · [日本語](../ja/developer-mode.md) · [简体中文](../zh-CN/developer-mode.md) · [繁體中文](../zh-TW/developer-mode.md) · [Español](../es/developer-mode.md) · [Français](../fr/developer-mode.md) · [Deutsch](../de/developer-mode.md) · [Português (Brasil)](../pt-BR/developer-mode.md) · [Русский](../ru/developer-mode.md) · [Italiano](../it/developer-mode.md) · [Tiếng Việt](../vi/developer-mode.md) · **Bahasa Indonesia** · [Türkçe](../tr/developer-mode.md)

Mode pengembang memungkinkan Android Debug Bridge (adb) di PC Anda terhubung ke Android Anda di
Desktop Subsystem for Android™ (DSA). Mode ini nonaktif secara bawaan.

adb bukan bagian dari DSA. Dapatkan dari Android SDK Platform-Tools.

## Menghubungkan adb

1. Buka halaman **Network** (jaringan) DSA Settings dan aktifkan **Developer mode**. Atau
   jalankan `dsa settings set developer-mode on`.
2. Mulai Android, jika belum berjalan: `dsa start --wait`.
3. Hubungkan:

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   Android tidak meminta Anda mengizinkan koneksi. Port ini hanya merespons di PC ini, sehingga
   komputer lain tidak dapat terhubung.
4. Setelah selesai, matikan lagi **Developer mode**, atau jalankan
   `dsa settings set developer-mode off`.

**Connect adb** di halaman **Network** menampilkan perintah yang tepat, dan `dsa settings`
mencetaknya di baris `Developer mode`.

## Menggunakan port lain

Port bawaannya 58526. Untuk mengubahnya, edit **Port** di bawah **Connect adb**, atau jalankan:

```
dsa settings set adb-port 5555
```

## Yang tidak diubah oleh mode pengembang

DSA tidak pernah menggunakan adb untuk pekerjaannya sendiri. Jendela, input, dan instalasi
menggunakan kanal kontrol milik DSA sendiri, sehingga semuanya bekerja sama saja saat mode
pengembang nonaktif.

Untuk membaca log Android, Anda tidak memerlukan adb. Lihat
[Pemecahan masalah](troubleshooting.md).

---

Android adalah merek dagang Google LLC. DSA tidak berafiliasi dengan, tidak didukung oleh, dan
tidak disponsori oleh Google.
