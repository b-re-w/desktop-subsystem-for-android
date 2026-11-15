# Versi Android

[English](../android-versions.md) · [한국어](../ko/android-versions.md) · [日本語](../ja/android-versions.md) · [简体中文](../zh-CN/android-versions.md) · [繁體中文](../zh-TW/android-versions.md) · [Español](../es/android-versions.md) · [Français](../fr/android-versions.md) · [Deutsch](../de/android-versions.md) · [Português (Brasil)](../pt-BR/android-versions.md) · [Русский](../ru/android-versions.md) · [Italiano](../it/android-versions.md) · [Tiếng Việt](../vi/android-versions.md) · **Bahasa Indonesia** · [Türkçe](../tr/android-versions.md)

Desktop Subsystem for Android™ (DSA) mendukung dua versi mayor Android terbaru: saat ini
Android 16 dan Android 17. Secara bawaan DSA menjalankan yang terbaru. Sebagian aplikasi hanya
berjalan di salah satunya, jadi Anda dapat memilih.

## Melihat versi yang berjalan

Buka halaman **Updates** (pembaruan) DSA Settings. Di bawah **Android** ditampilkan versinya,
build-nya, tingkat patch keamanannya, versi yang diikutinya, dan status pembaruannya, beserta
kemajuan unduhan. **Check now** meminta layanan untuk segera mencari pembaruan. Atau jalankan:

```
dsa android version   # versi yang berjalan, dan versi yang dapat Anda pilih
```

## Mengubah versi

Setiap pengguna PC menjalankan versi Android yang sama, sehingga hanya administrator yang dapat
mengubahnya.

1. Buka halaman **Updates** DSA Settings.
2. Di bawah **Version to use**, pilih sebuah versi, atau **Always the newest**.
3. Pilih **Change**, lalu setujui prompt administrator.

Versi yang dipilih diunduh dan digunakan sejak Android dimulai berikutnya. Dari terminal
administrator:

```
dsa android use 16
dsa android use latest
```

### Pindah ke versi yang lebih lama menghapus data

Android tidak dapat membaca data yang ditulis oleh versi yang lebih baru. Oleh karena itu,
pindah ke versi mayor yang lebih lama menghapus data Android setiap pengguna PC: aplikasi yang
mereka instal, akun mereka, dan semua data aplikasi. Pengaturan DSA tetap ada. Tindakan ini
tidak dapat dibatalkan.

DSA Settings meminta Anda mengonfirmasi dengan **Erase data and change**. Di baris perintah,
tambahkan `--erase-data`:

```
dsa android use 16 --erase-data
```

Pindah ke versi yang lebih baru mempertahankan aplikasi dan data Anda.

## Pembaruan

- Versi yang dipilih terus menerima pembaruan keamanan.
- Saat sebuah versi tidak lagi didukung, DSA pindah ke versi berikutnya dan mempertahankan
  aplikasi Anda.
- DSA tidak pernah pindah ke versi mayor yang lebih lama dengan sendirinya.
- Setiap image diperiksa terhadap daftar yang ditandatangani sebelum digunakan. Image yang
  gagal boot diganti dengan image sebelumnya.

Administrator dapat mematikan pembaruan image dengan **Update the Android image
automatically** di halaman **Updates** DSA Settings, atau dengan
`dsa settings set image-updates off`.

---

Android adalah merek dagang Google LLC. Windows adalah merek dagang grup perusahaan Microsoft.
DSA tidak berafiliasi dengan, tidak didukung oleh, dan tidak disponsori oleh Google maupun
Microsoft.
