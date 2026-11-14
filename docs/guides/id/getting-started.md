# Memulai

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · [日本語](../ja/getting-started.md) · [简体中文](../zh-CN/getting-started.md) · [繁體中文](../zh-TW/getting-started.md) · [Español](../es/getting-started.md) · [Français](../fr/getting-started.md) · [Deutsch](../de/getting-started.md) · [Português (Brasil)](../pt-BR/getting-started.md) · [Русский](../ru/getting-started.md) · [Italiano](../it/getting-started.md) · [Tiếng Việt](../vi/getting-started.md) · **Bahasa Indonesia** · [Türkçe](../tr/getting-started.md)

Desktop Subsystem for Android™ (DSA) menjalankan Android di mesin virtual ringan dengan
kernelnya sendiri. Setiap aplikasi Android terbuka di jendelanya sendiri di desktop Anda.

## Status per platform

| Host | Status |
|---|---|
| Windows 11, x64 | Dirilis. |
| Windows 11, ARM64 | Dirilis. Android juga ARM64, sehingga aplikasi ARM berjalan tanpa terjemahan. |
| Linux, x64 dan ARM64 | Sedang dikerjakan. Paket `.deb` dan `.rpm` dibangun di CI; belum dirilis. |
| macOS, Apple silicon | Sedang dikerjakan. `DSA.app`, `.pkg`-nya, dan `.dmg`-nya dibangun di CI; belum dirilis. |

## Instal di Windows

Anda memerlukan Windows 11 di PC x64 atau ARM64, dengan fitur Windows **Platform Mesin
Virtual** (Virtual Machine Platform) diaktifkan.

1. Aktifkan Platform Mesin Virtual. Buka **Aktifkan atau nonaktifkan fitur Windows**, centang
   **Platform Mesin Virtual**, pilih **OK**, lalu mulai ulang. Atau jalankan
   `dism /online /enable-feature /featurename:VirtualMachinePlatform /all` di terminal
   administrator. Penyiapan tidak mengaktifkannya untuk Anda: penyiapan berhenti dengan pesan
   jika fitur itu nonaktif.
2. Unduh program penyiapan dan `SHA256SUMS.txt` dari
   [rilis terbaru](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest):
   `DSA-v<version>-win-x64-setup.exe` di PC x64, `DSA-v<version>-win-arm64-setup.exe` di PC
   ARM64.
3. Periksa unduhan. Hash yang ditampilkan perintah ini harus cocok dengan barisnya di
   `SHA256SUMS.txt`:

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. Jalankan penyiapan, pilih **Install**, lalu setujui prompt administrator.
5. Tunggu image Android pertama. Jika penyiapan tidak membawanya, layanan DSA mengunduhnya di
   latar belakang. `dsa status` menunjukkan unduhan tersebut.
6. Buka aplikasi dari menu Mulai, atau periksa dari terminal:

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## Yang diinstal oleh penyiapan

- Layanan `DsaHost`, yang menjalankan Android setiap pengguna.
- Perintah `dsa`.
- **DSA Settings** di menu Mulai.
- **Android** di panel navigasi File Explorer, yang membuka file Android.
- APK Installer, yang membuka file `.apk`.
- DSA Window Manager (`dsa-wm`). Ia dimulai saat Anda masuk dan memulai Android pada saat itu,
  sehingga aplikasi pertama yang Anda buka tidak menunggu Android boot.

Setiap pengguna Windows memiliki Android dan disk data sendiri,
`%LOCALAPPDATA%\DSA\android.vhdx`. Pengguna lain tidak dapat membacanya.

DSA menggunakan kernel dan mesin virtualnya sendiri. DSA tidak menggunakan atau mengubah
pengaturan, kernel, maupun distribusi WSL Anda.

## Pembaruan

DSA memperbarui dirinya sendiri dan image Android. Keduanya ditandatangani, dan DSA tidak
menerapkan apa pun yang tanda tangan, ukuran, atau hash-nya tidak cocok.

- **DSA itu sendiri** diperbarui saat Android milik pengguna mana pun tidak sedang berjalan.
  Sampai saat itu pembaruan menunggu; `dsa update` langsung menginstalnya. DSA tidak pernah
  pindah ke versi yang lebih rendah.
- **Kanal rilis.** Mulai 1.0, DSA memiliki kanal stabil dan kanal pra-rilis untuk versi yang
  masih disempurnakan. Kanal Anda ditentukan oleh penginstal yang Anda gunakan, dan
  peningkatan versi mempertahankannya. Administrator mengubahnya dengan **Get pre-release
  versions** (dapatkan versi pra-rilis) di halaman **Updates** (pembaruan) DSA Settings.
  Setiap rilis sebelum 1.0 adalah pra-rilis.
- **Image Android** tidak bergantung pada kanal. Jika sebuah image gagal boot, DSA kembali ke
  image sebelumnya, dan `dsa status` memberitahukannya.

Administrator dapat mematikan pembaruan di halaman **Updates** DSA Settings. Lihat
[Pengaturan](settings.md) untuk semua sakelar.

## Linux (sedang dikerjakan, belum dirilis)

DSA untuk Linux dibangun sebagai `.deb` (Debian, Ubuntu) dan `.rpm` (Fedora, openSUSE). DSA
memerlukan KVM (`/dev/kvm`) dan desktop Wayland. Android berjalan di mesin virtual crosvm.

Menginstal paket juga menambahkan repositori paket DSA yang ditandatangani, sehingga versi
berikutnya datang melalui `apt`, `dnf`, `zypper`, atau pusat perangkat lunak. DSA tidak
memiliki pembaru sendiri di Linux.

- Debian dan Ubuntu: `/etc/apt/sources.list.d/dsa.sources`, dengan kunci di
  `/usr/share/keyrings/dsa-archive-keyring.gpg`.
- Fedora dan openSUSE: `/etc/yum.repos.d/dsa.repo` (dan `/etc/zypp/repos.d/dsa.repo` jika
  zypper tersedia), dengan kunci di `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa`.

Repositori ini akan disajikan dari
`https://thisisthepy.github.io/desktop-subsystem-for-android/`. Repositori ini belum
diterbitkan.

Setelah dirilis, instalasi pertama terlihat seperti ini:

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian, Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

Paket ini menginstal layanan sistem `dsa-hostd`, layanan pengguna `dsa-devices` dan
`dsa-compositor`, perintah `dsa` di `/usr/bin`, entri menu **DSA Files**, serta aksi file
**Open in an Android app**, **Share with an Android app**, dan **Install in Android**.

## macOS (sedang dikerjakan, belum dirilis)

DSA untuk macOS menjalankan Android di atas Virtualization framework milik Apple, di Apple
silicon. DSA dibangun sebagai `DSA.app`, dalam paket penginstal (`DSA-<version>.pkg`) dan
image disk (`DSA-<version>.dmg`). Build CI ditandatangani secara ad hoc, sehingga hanya
berjalan di mesin yang membangunnya. Belum ada rilis yang ditandatangani dengan Developer ID.

## Selanjutnya

- [Aplikasi dan jendela](apps.md)
- [Menginstal APK](install-apks.md)

---

Android adalah merek dagang Google LLC. Windows adalah merek dagang grup perusahaan Microsoft.
macOS adalah merek dagang Apple Inc. DSA tidak berafiliasi dengan, tidak didukung oleh, dan
tidak disponsori oleh Google, Microsoft, maupun Apple.
