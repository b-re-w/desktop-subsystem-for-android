# Pengaturan

[English](../settings.md) · [한국어](../ko/settings.md) · [日本語](../ja/settings.md) · [简体中文](../zh-CN/settings.md) · [繁體中文](../zh-TW/settings.md) · [Español](../es/settings.md) · [Français](../fr/settings.md) · [Deutsch](../de/settings.md) · [Português (Brasil)](../pt-BR/settings.md) · [Русский](../ru/settings.md) · [Italiano](../it/settings.md) · [Tiếng Việt](../vi/settings.md) · **Bahasa Indonesia** · [Türkçe](../tr/settings.md)

**DSA Settings** adalah aplikasi pengaturan Desktop Subsystem for Android™ (DSA) di Windows.
Buka dari menu Mulai. Aplikasi ini tampil dalam bahasa Inggris atau Korea, mengikuti bahasa
tampilan Windows Anda, dan mengikuti tema Windows Anda. Aplikasi ini dapat digunakan dengan
keyboard dan pembaca layar.

Setiap perubahan yang dibuat DSA Settings melewati perintah `dsa`, sehingga baris perintah
melihat nilai yang sama:

```
dsa settings                        # semua pengaturan beserta nilainya
dsa settings set <name> <value>     # ubah satu; sakelar menerima on atau off
```

Sebagian pengaturan berlaku untuk setiap pengguna PC, sehingga hanya administrator yang dapat
mengubahnya. DSA Settings menandainya dengan ikon perisai dan meminta persetujuan administrator
saat Anda mengubahnya. Pengaturan itu ditandai "(admin)" di bawah.

## Halaman

| Halaman | Yang ada di sana | Baris perintah |
|---|---|---|
| **General** (umum) | Status Android, **Start**, **Stop**, **Freeze Android now**, masalah terakhir, **Open my log folder**, **Refresh** | `dsa status`, `dsa start`, `dsa stop`, `dsa freeze` |
| **Memory** (memori) | **Memory limit**, ukuran VM Android, penggunaan setiap bagian | `memory-limit`, `dsa memory` |
| **Apps** (aplikasi) | Aplikasi yang terinstal: **Open**, **App info**, **Uninstall**, **Create desktop shortcut**. Untuk setiap aplikasi: **Resizable**, ukuran jendela saat dibuka, **Scaling on another monitor**, **Full screen**, **Bars hidden**, **Run as ARM**, **Root access**. Satu baris untuk akses root **adb shell** | `dsa apps`, `dsa app-window`, `dsa apps run-as-arm`, `dsa apps root` |
| **Running in the background** (berjalan di latar belakang) | Aplikasi yang berjalan tanpa jendela: **Open**, **Stop**; apakah perlu bertanya sebelum menghentikan | `dsa background`, `stop-without-asking` |
| **Widgets** (widget) | Widget di desktop Anda, dan **Add to desktop** | `dsa widgets` |
| **Display & GPU** (tampilan & GPU) | **Render GPU**, **GPU of the primary monitor**, **App full screen**, **Apps that hide their bars**, **Turn off Windows touch gestures in DSA app windows** (hanya Windows 11, aktif secara bawaan), **Restart the window manager** | `render-gpu`, `dsa gpus`, `app-fullscreen`, `app-immersive`, `touch-gestures-off` |
| **Device identity** (identitas perangkat) | Pertama komputer ini, hanya baca: nama, pembuat dan modelnya, serta **Device specifications** dengan **Copy**. Lalu produsen, model, dan sidik jari build yang dilaporkan Android ke aplikasi | `dsa settings device`, `device.<field>` |
| **Network** (jaringan) | Cara Android menjangkau jaringan, alamatnya, siapa yang menjawab nama-namanya (DNS), dan gateway-nya, **Advanced networking**, **Use this PC's added root certificates**, **Developer mode** (adb), **Connect adb automatically** | `dsa network`, `advanced-networking`, `host-certificates`, `developer-mode`, `adb-port`, `adb-auto-connect` |
| **Updates** (pembaruan) | **Android**: versi dan build yang berjalan, tingkat patch keamanannya, versi yang diikutinya, pembaruan Android beserta kemajuannya dan **Check now** (admin), **Restart Android into updates by itself**, **Update the Android image automatically** (admin), **Version to use** (admin). **DSA**: **Update DSA automatically** (admin), **Get pre-release versions** (admin), **Install the newest version of DSA now** | `dsa android version`, `dsa android use`, `image-auto-restart`, `image-updates`, `program-updates`, `prerelease-updates`, `dsa update` |
| **Advanced** (lanjutan) | Papan klip, file, tautan web, bekukan saat diam, pencadangan, reset, aplikasi ARM, Google Play, konten yang dilindungi, akses root, jejak (admin), folder log | lihat di bawah |
| **Logs** (log) | Log langsung Android, **Save diagnostics** | `dsa logcat`, `dsa logs collect` |
| **About** (tentang) | Versi, lisensi, kode sumber | |

## Halaman Advanced

| Pengaturan | Bawaan | Baris perintah |
|---|---|---|
| **Share the clipboard with Android** | Aktif | `clipboard` |
| **Share Windows folders with Android** | Aktif | `file-sharing` |
| **Shared folders** | Keenam folder | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`, `dsa share` |
| **Freeze Android when idle** | Nonaktif | `freeze` |
| **Open web links in Android apps** | Nonaktif | `links-in-android` |
| **Back up apps' data to this PC**, **Back up now**, **Restore** | Nonaktif | `dsa backup on`, `dsa backup now`, `dsa backup restore` |
| **Recovery key**, **Use a recovery key** | | `dsa backup key show`, `dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator** (tidak ada di ARM64) | | `dsa arm-translation fetch google` |
| **Use the default translator** (tidak ada di ARM64) | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**, **Remove Google Play** | Tidak terinstal | `dsa gapps install`, `dsa gapps remove` |
| **Widevine** (di bawah **Protected content**) | Tidak terinstal | `dsa drm widevine install`, `dsa drm widevine remove` |
| **Let Android apps ask for root** (admin) | Nonaktif | `root-elevation` |
| **Input trace** (admin) | Nonaktif | `input-trace` |
| **Resize trace** (admin) | Nonaktif | `resize-trace` |

Pencadangan menyimpan salinan terenkripsi data aplikasi Anda di folder pengguna Anda, dengan
kunci yang disimpan Windows untuk Anda. Untuk memulihkannya di PC lain, atau setelah Windows
diinstal ulang, Anda memerlukan kunci pemulihan, jadi catatlah di tempat yang terpisah dari PC.
Google Play, Widevine, dan akses root dijelaskan di [Aplikasi dan jendela](apps.md).

## Kapan perubahan berlaku

- **Render GPU**: saat pengelola jendela dimulai lagi, pada saat Anda masuk berikutnya atau
  saat Anda memilih **Restart the window manager**.
- **Memory limit**, **Device identity**, **Advanced networking**, **Run apps built only for
  ARM**, Google Play, Widevine, mengaktifkan akses root, dan versi Android: sejak Android
  dimulai berikutnya. Untuk memulai ulang Android, hentikan di halaman **General**, lalu buka
  sebuah aplikasi.
- Pengaturan jendela aplikasi: sejak jendela berikutnya yang dibuka aplikasi tersebut.
- Semua yang lain, termasuk mematikan akses root: seketika.

## Cara Android menemukan nama

Komputer Anda menjawab setiap nama yang dicari aplikasi Android, dengan resolver yang sama yang
digunakan aplikasi Windows (di Linux dan macOS, resolver sistem). Jadi file hosts Anda, aturan
nama perusahaan Anda, server DNS milik VPN, dan filter DNS berlaku untuk aplikasi Android
seperti halnya untuk komputer. Android sendiri tidak bertanya ke server DNS.

- Hal ini sama saat **Advanced networking** aktif: komputer Anda berada di jaringan yang sama
  dan mengetahui nama-namanya, dan VPN atau filter DNS di komputer tetap berlaku.
- **Private DNS** milik Android nonaktif secara bawaan, karena fitur itu akan bertanya ke
  server DNS dengan melewati komputer Anda. Jika Anda memilih server DNS pribadi di pengaturan
  Android, Android menggunakannya.
- Jika DSA tidak dapat menjawab untuk sementara waktu, Android bertanya ke server DNS jaringan
  sampai DSA dapat menjawab lagi.
- Baris **DNS** di halaman **Network**, dan `dsa network`, memberitahukan siapa yang menjawab
  saat ini: resolver komputer Anda, atau server DNS jaringan, yang juga dicantumkan.

## Nilai yang diterima baris perintah

- `render-gpu`: `display` (GPU monitor utama, bawaan), `integrated`, `discrete`, atau bagian
  dari nama GPU seperti yang dicantumkan `dsa gpus`.
- `memory-limit`: `auto` (bawaan: tanpa batas tetap, hingga memori PC dikurangi bagian yang
  disisakan untuk Windows), atau ukuran seperti `4g` atau `3072` (MiB).
- `shared-folders`: `all`, `none`, atau nama yang dipisahkan koma, seperti
  `Documents,Pictures`.
- `adb-port`: port TCP dari 1 hingga 65535. Bawaannya 58526.
- `app-fullscreen`: `screen` (bawaan) atau `window`, untuk permintaan layar penuh yang jelas
  dari aplikasi. `app-immersive`: `screen` atau `window` (bawaan), untuk aplikasi yang hanya
  menyembunyikan bilahnya. Lihat [Layar penuh](apps.md#layar-penuh).
- `touch-gestures-off`: `on` (bawaan) mematikan gestur sentuh tiga dan empat jari Windows 11
  selama jendela aplikasi DSA berada di depan, sehingga aplikasi menerima hingga sepuluh jari,
  dan mengembalikan pengaturan Anda sendiri saat jendela lain berada di depan. Jika Windows
  menolak, pengaturan ini tidak dapat digunakan dan aplikasi pengaturan menjelaskan alasannya;
  Anda tetap dapat mematikan gestur untuk semua aplikasi di Pengaturan Windows, di Bluetooth &
  perangkat, Sentuh.
- `host-certificates`: `on` (bawaan) memungkinkan aplikasi Android memercayai sertifikat root
  yang ditambahkan ke PC ini, seperti milik perusahaan Anda atau program keamanan.
- `adb-auto-connect`: `on` (bawaan) menampilkan Android di `adb devices` dan Android Studio
  selama mode pengembang aktif, tanpa `adb connect`.
- `image-auto-restart`: `on` (bawaan) memulai ulang Android ke pembaruan image yang sudah
  diunduh secara langsung saat tidak ada jendela aplikasi yang terbuka. Jika tidak, DSA
  bertanya terlebih dahulu, atau menunggu sampai Anda menutupnya.
- `prerelease-updates`: `on` mengambil pra-rilis maupun rilis stabil, `off` hanya mengambil
  rilis stabil. Nilai awalnya ditentukan oleh penginstal yang Anda gunakan: penginstal
  pra-rilis mengaktifkannya, penginstal stabil membiarkannya nonaktif, dan peningkatan versi
  mempertahankan pilihan Anda. Setiap rilis sebelum 1.0 adalah pra-rilis. Mematikannya tidak
  pernah memundurkan DSA ke versi yang lebih lama; DSA menunggu rilis stabil yang lebih baru
  daripada yang terinstal.
- `root-elevation`: `on` memungkinkan aplikasi yang Anda izinkan di halaman **Apps** meminta
  root. Tingkat setiap aplikasi diatur dengan `dsa apps root <package> off|ask|remember`.
- `device.<field>`: `manufacturer`, `brand`, `model`, `device`, `name`, `characteristics`,
  `fingerprint`, atau bagian dari sidik jari seperti `fingerprint.release`.
  `dsa settings reset device` mengembalikan semua nilai.

---

Android adalah merek dagang Google LLC. Windows adalah merek dagang grup perusahaan Microsoft.
DSA tidak berafiliasi dengan, tidak didukung oleh, dan tidak disponsori oleh Google maupun
Microsoft.
