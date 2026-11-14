# Pencopotan

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · [日本語](../ja/uninstall.md) · [简体中文](../zh-CN/uninstall.md) · [繁體中文](../zh-TW/uninstall.md) · [Español](../es/uninstall.md) · [Français](../fr/uninstall.md) · [Deutsch](../de/uninstall.md) · [Português (Brasil)](../pt-BR/uninstall.md) · [Русский](../ru/uninstall.md) · [Italiano](../it/uninstall.md) · [Tiếng Việt](../vi/uninstall.md) · **Bahasa Indonesia** · [Türkçe](../tr/uninstall.md)

## Windows

Copot Desktop Subsystem for Android™ (DSA) dengan salah satu cara berikut:

- **Pengaturan** Windows > **Aplikasi** > **Aplikasi yang diinstal**: cari Desktop Subsystem
  for Android, lalu pilih **Copot instalan**.
- Jalankan penyiapan lagi dan pilih **Uninstall**.

Pencopotan menghapus hal-hal berikut, untuk setiap pengguna PC, baik yang sedang masuk maupun
tidak:

- program, layanan `DsaHost`, perintah `dsa`, DSA Settings, dan APK Installer;
- image Android dan pembaruan yang telah diunduh;
- entri menu Mulai aplikasi Android dan entrinya di **Aplikasi yang diinstal**;
- **Android** di File Explorer, asosiasi file `.apk`, dan entri menu konteks;
- pengaturan dan log DSA.

Pencopotan **mempertahankan** disk Android setiap pengguna, `%LOCALAPPDATA%\DSA\android.vhdx`,
sehingga instalasi berikutnya menemukan aplikasi dan data Anda lagi. Untuk menghapus data Anda
juga, jalankan `dsa reset-data --yes` sebelum mencopot, atau hapus file itu sesudahnya.

Jika pengalihan folder memindahkan menu Mulai seorang pengguna, entri aplikasi Android milik
pengguna itu tetap ada, karena pencopot hanya mencari di lokasi bawaan. Hapuslah secara manual.

Penyiapan WSL Anda tidak tersentuh, karena DSA tidak pernah mengubahnya.

## Linux (belum dirilis)

```
sudo apt remove desktop-subsystem-for-android    # Debian, Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

Menghapus paket juga menghapus entri repositori paket yang ditambahkannya.

---

Android adalah merek dagang Google LLC. Windows adalah merek dagang grup perusahaan Microsoft.
DSA tidak berafiliasi dengan, tidak didukung oleh, dan tidak disponsori oleh Google maupun
Microsoft.
