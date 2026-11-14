# Aplikasi dan jendela

[English](../apps.md) · [한국어](../ko/apps.md) · [日本語](../ja/apps.md) · [简体中文](../zh-CN/apps.md) · [繁體中文](../zh-TW/apps.md) · [Español](../es/apps.md) · [Français](../fr/apps.md) · [Deutsch](../de/apps.md) · [Português (Brasil)](../pt-BR/apps.md) · [Русский](../ru/apps.md) · [Italiano](../it/apps.md) · [Tiếng Việt](../vi/apps.md) · **Bahasa Indonesia** · [Türkçe](../tr/apps.md)

Setiap tugas aplikasi Android terbuka sebagai jendela desktop tersendiri, dengan tombol bilah
tugasnya sendiri. Tidak ada bingkai emulator di sekeliling layar ponsel. Halaman ini
menjelaskan Desktop Subsystem for Android™ (DSA) di Windows.

## Membuka aplikasi

- **Menu Mulai.** Setiap aplikasi yang terinstal memiliki entri menu Mulai dengan nama dan
  ikonnya sendiri, sehingga pencarian Windows dapat menemukannya. Membukanya akan memulai
  Android terlebih dahulu jika Android belum berjalan.
- **DSA Settings.** Di halaman **Apps** (aplikasi), pilih **Open** di samping aplikasi.
- **Baris perintah.** `dsa apps list` menampilkan aplikasi yang terinstal beserta paketnya.
  `dsa launch <package>` membuka salah satunya, misalnya `dsa launch com.android.settings`.

Jika entri menu Mulai milik program lain sudah memakai nama aplikasi tersebut, DSA menambahkan
nama paket ke nama itu, misalnya `Clock (com.example.clock)`. DSA hanya mengubah atau menghapus
entri yang dibuatnya sendiri.

## Pintasan desktop dan jump list

Ikon aplikasi bekerja seperti ikon aplikasi Windows, bukan seperti layar beranda Android.

- **Pintasan desktop.** Di halaman **Apps** DSA Settings, pilih **Create desktop shortcut** di
  samping aplikasi. Anda juga dapat mengklik kanan tombol bilah tugas aplikasi dan memilih
  **Create desktop shortcut**. Dari baris perintah: `dsa apps desktop-shortcut <package>`.
  Pintasan ini membuka aplikasi seperti entri menu Mulainya.
- **Pintasan aplikasi.** Pintasan yang ditawarkan aplikasi Android, seperti obrolan baru di
  aplikasi pesan, muncul saat Anda mengklik kanan tombol bilah tugas aplikasi atau entri menu
  Mulainya. Memilih salah satunya membuka layar aplikasi tersebut di sebuah jendela, dengan
  memulai Android terlebih dahulu jika belum berjalan. Saat aplikasi mengubah pintasannya,
  daftar ini ikut berubah. Dari baris perintah: `dsa apps shortcut <package> <id>`;
  `dsa apps info <package> --json` menampilkan ID-nya.
- **Pintasan yang diminta aplikasi.** Saat aplikasi meminta untuk menambahkan salah satu
  pintasannya ke desktop Anda, DSA bertanya kepada Anda terlebih dahulu dan hanya menambahkannya
  jika Anda setuju.

Saat aplikasi berganti nama, misalnya setelah pembaruan, entri menu Mulai, pintasan desktop,
dan sematannya di bilah tugas mengikuti nama baru dan tetap berfungsi. Pintasan desktop yang
sudah Anda hapus tidak dibuat lagi. Mencopot aplikasi juga menghapus pintasan desktopnya.

## Bekerja dengan jendela aplikasi

- Setiap tugas memiliki jendela, tombol bilah tugas, dan entri Alt+Tab sendiri, dengan nama
  dan ikon aplikasi. Pengelola Tugas mencantumkan setiap aplikasi dengan namanya sendiri.
- Bilah judul adalah bagian dari aplikasi: warna aplikasi mencapai bagian atas jendela, dan
  tombol keterangan Windows 11 berada di atasnya. Ikon dan nama aplikasi ada di kiri. Saat
  aplikasi dapat kembali, ikon berubah menjadi panah kembali; pilih untuk kembali.
- Tata letak snap (arahkan penunjuk ke tombol maksimalkan), menyeret bilah judul, mengklik
  ganda untuk memaksimalkan, dan mengubah ukuran secara langsung semuanya berfungsi. Mengubah
  ukuran tidak memulai ulang aplikasi. Jendela yang dimaksimalkan mengisi area kerja monitor
  dan membiarkan bilah tugas tetap terlihat.
- Tidak ada bilah navigasi Android. Bilah bawah aplikasi berada di tepi bawah jendela, tanpa
  pita kosong di bawahnya.
- Mouse (dengan hover), sentuhan, pena (tekanan, penghapus, dan tombol barrel), dan keyboard
  semuanya berfungsi. Teks berasal dari metode input Windows Anda, termasuk bahasa Korea dan
  bahasa lainnya; komposisi tampil langsung di editor Android, dan jendela kandidat berada di
  samping kursor. Tidak ada aplikasi keyboard Android.
- Seret file dari File Explorer ke jendela aplikasi, atau seret file keluar dari aplikasi ke
  File Explorer.
- Pembaca layar seperti Narator (Narrator) dan NVDA membaca aplikasi melalui UI Automation.
- Menutup jendela meminta Android untuk mengakhiri tugas tersebut.

## Menu jendela

Klik kanan bilah judul, pilih ikon aplikasi, atau tekan Alt+Space. Setelah item Windows yang
biasa, menu ini berisi:

- **Full screen** (layar penuh): mengisi seluruh layar, seperti F11. Pilih lagi, atau tekan
  F11, untuk keluar.
- **New window** (jendela baru): membuka jendela lain dari aplikasi, untuk aplikasi yang
  mendukung beberapa jendela.
- **Screenshot** (cuplikan layar): menyalin layar aplikasi, tanpa bilah judul, ke papan klip
  dan menyimpannya di folder Cuplikan Layar di folder Gambar Anda.
- **Aspect ratio** (rasio aspek): membuka pengaturan Android untuk rasio aspek aplikasi, bagi
  aplikasi yang tampak melar di jendela besar.
- **Open by default** (buka secara bawaan): memilih apakah tautan yang didukung aplikasi
  terbuka di aplikasi itu atau di browser Anda.
- **App info** (info aplikasi): membuka entri aplikasi di halaman **Apps** DSA Settings.

## Layar penuh

Saat aplikasi meminta layar penuh, jendelanya menampilkannya dengan salah satu dari dua cara:

- **Di seluruh layar**: jendela mengisi monitor, dan bilah judul disembunyikan. Sentuhkan
  penunjuk ke tepi atas layar untuk menampilkannya, dengan tombol minimalkan dan tutup; tombol
  tengah keluar dari layar penuh.
- **Di dalam jendela**: jendela mempertahankan ukuran dan bilah judulnya, dan aplikasi
  mengisinya.

Aplikasi meminta dengan dua jenis cara, dan masing-masing jenis memiliki bawaannya sendiri:

| Jenis permintaan | Contoh | Bawaan |
|---|---|---|
| Permintaan layar penuh yang jelas | Tombol layar penuh di pemutar video | Di seluruh layar |
| Hanya menyembunyikan bilah | Feed video pendek | Di dalam jendela |

Ubah bawaannya di **App full screen** dan **Apps that hide their bars** pada halaman **Display
& GPU** (tampilan & GPU) DSA Settings, atau untuk satu aplikasi di **Full screen** dan **Bars
hidden** pada halaman **Apps**. Pilihan aplikasi itu sendiri didahulukan. F11 dan **Full
screen** di menu jendela mengalihkan layar penuh di seluruh layar kapan saja, apa pun
pengaturannya.

Dari baris perintah:

```
dsa settings set app-fullscreen screen|window           # bawaan untuk permintaan yang jelas
dsa settings set app-immersive screen|window            # bawaan untuk bilah yang disembunyikan
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## Mengatur ukuran jendela aplikasi

Di halaman **Apps** DSA Settings:

- Matikan **Resizable** (dapat diubah ukurannya) agar jendela aplikasi tetap pada ukuran saat
  dibuka.
- Pilih ukuran jendela aplikasi saat dibuka: ukuran yang dipilih Android, atau ukuran ponsel
  atau tablet dalam mode potret atau lanskap.

Perubahan berlaku untuk jendela berikutnya yang dibuka aplikasi. Dari baris perintah:

```
dsa app-window <package> --fixed-size on        # tidak dapat diubah ukurannya atau dimaksimalkan
dsa app-window <package> --start-size 1280x800  # dibuka pada ukuran ini; auto membiarkan Android memilih
dsa app-window <package> --dpi-scaling system   # cara penskalaan di monitor lain; lihat di bawah
dsa app-window <package> reset                  # kembali ke bawaan
```

## Memindahkan aplikasi ke monitor dengan skala lain

Aplikasi terbuka dengan skala monitor tempat jendelanya dibuka, sejak frame pertamanya,
sehingga aplikasi tidak dibuat ulang tepat setelah terbuka. Saat Anda memindahkan jendela ke
monitor dengan skala lain, aplikasi mempertahankan ukurannya di layar. Setiap aplikasi
melakukannya dengan salah satu cara berikut, yang Anda pilih di **Scaling on another monitor**
(penskalaan di monitor lain) pada halaman **Apps**, atau dengan
`dsa app-window <package> --dpi-scaling`:

- **Automatic** (`auto`, bawaan): **System** untuk aplikasi yang ada di daftar bawaan DSA
  berisi aplikasi yang rusak jika skalanya berubah saat berjalan, dan **App** untuk semua
  aplikasi lainnya. Daftar ini berisi YouTube: YouTube mempertahankan tata letaknya untuk skala
  saat ia dimulai, sehingga setelah dipindahkan, tab bawah dan miniplayer-nya akan tetap rusak
  sampai dimulai ulang. Pengaturan menampilkan bawaan aplikasi seperti itu sebagai **Scaling:
  system (recommended for this app)**. Pilihan Anda sendiri untuk aplikasi tersebut
  didahulukan.
- **App** (`app`): selama Anda menyeret jendela, DSA menskalakannya ke monitor baru. Saat Anda
  melepasnya, aplikasi menggambar ulang pada skala monitor baru, dan teksnya tetap tajam:
  Android membuat ulang layarnya sekali dan memulihkan statusnya (posisi gulir, teks yang
  diketik), sehingga aplikasi menggambar ulang sebentar setelah setiap pemindahan, seperti
  program Windows dengan penskalaan "Application". Hal ini berlaku bahkan untuk aplikasi yang
  menyatakan dapat menangani perubahan skala sendiri, karena sebagian aplikasi itu hanya
  menggambar ulang sebagian layarnya pada skala baru.
- **System** (`system`): aplikasi mempertahankan skalanya, dan DSA menskalakan jendelanya ke
  monitor baru. Aplikasi tidak pernah dibuat ulang. Jika diperbesar, teks dapat tampak sedikit
  kabur.
- **System (forced)** (`system-forced`): aplikasi diberi skala baru tanpa dimulai ulang,
  meskipun aplikasi tidak menyatakan dapat menanganinya. Belum tersedia: fitur ini memerlukan
  perubahan pada Android yang belum ada di versi ini, sehingga sampai saat itu ia bekerja
  seperti **System**.

Teks di aplikasi mengikuti ukuran teks Windows (Pengaturan > Aksesibilitas > Ukuran teks).

## Pemberitahuan

Pemberitahuan Android tampil sebagai pemberitahuan Windows, dengan nama dan ikon aplikasi.

- Pilih pemberitahuan untuk membuka aplikasi.
- Tutup pemberitahuan, dan pemberitahuan itu juga ditutup di Android, kecuali aplikasi tetap
  menampilkannya.
- Pemberitahuan tetap masuk selama Android dalam Doze. Lihat [Memori](memory.md).

## Aplikasi yang berjalan di latar belakang

Aplikasi yang tetap berjalan setelah jendelanya ditutup, seperti pemutar musik, tampil sebagai
ikon di area pemberitahuan. Tooltip-nya adalah pemberitahuan aplikasi tersebut.

- Pilih ikon untuk membuka aplikasi.
- Klik kanan ikon untuk **Open**, aksi milik pemberitahuan itu sendiri, dan **Stop**.
- **Stop** bertanya terlebih dahulu. Centang **Do not ask again** untuk melewati pertanyaan itu
  seterusnya. Agar ditanya lagi, aktifkan **Ask before stopping an app from the notification
  area** di halaman **Running in the background** (berjalan di latar belakang) DSA Settings.
- Aplikasi yang memutar media juga dapat dikendalikan dengan tombol media dan kontrol media
  Windows.

Halaman **Running in the background** DSA Settings mencantumkan aplikasi-aplikasi ini, dengan
**Open** dan **Stop** untuk masing-masing. Dari baris perintah: `dsa background`,
`dsa background open <package>`, dan `dsa background stop <package>`.

## Widget di desktop

Widget Android dapat berada di desktop Anda dalam jendelanya sendiri. Widget tetap berada di
bawah jendela lain, tetap ada saat Anda menampilkan desktop, dan tidak ada di bilah tugas
maupun Alt+Tab.

1. Buka halaman **Widgets** (widget) DSA Settings.
2. Di **Widgets your apps offer**, pilih **Add to desktop** di samping sebuah widget. Jika
   aplikasi meminta Anda menyiapkan widget tersebut, penyiapannya terbuka di sebuah jendela.
3. Untuk memindahkan widget, tekan di mana saja pada widget dan tahan diam selama setengah
   detik. Widget terangkat, dengan bingkai dan pegangan di setiap tepi yang dapat diubah
   ukurannya. Seret untuk memindahkannya, atau seret pegangan untuk mengubah ukurannya. Seretan
   cepat tetap menjadi milik widget, sehingga daftarnya tetap dapat digulir. Anda juga dapat
   menyeret tepi widget untuk mengubah ukurannya tanpa mengangkatnya terlebih dahulu; hanya
   tepi yang dapat diubah ukurannya yang bisa melakukan ini. DSA mengingat di mana Anda
   meletakkannya dan seberapa besar.
4. Klik kanan widget, atau lepaskan setelah tekan lama, untuk **Widget settings** (jika widget
   dapat disiapkan ulang), **Resize**, untuk memilih salah satu ukuran yang diizinkannya, dan
   **Remove**. Tombol menu dan Shift+F10 juga membukanya. Tombol panah, Enter, dan Esc
   berfungsi di menu ini, dan sebuah huruf memilih item: R untuk **Resize** dan M untuk
   **Remove**.

Dari baris perintah: `dsa widgets list`, `dsa widgets add <provider>`, dan
`dsa widgets remove <id>`.

## Tautan dan papan klip

- Tautan web yang Anda ketuk di aplikasi Android terbuka di browser bawaan Anda. Tautan yang
  telah diverifikasi sebagai milik aplikasi yang terinstal terbuka di aplikasi tersebut.
- Untuk membuka semua tautan web di aplikasi Android, aktifkan **Open web links in Android
  apps** di halaman **Advanced** (lanjutan) DSA Settings
  (`dsa settings set links-in-android on`).
- Teks yang Anda salin dibagikan ke dua arah antara Windows dan Android. Untuk
  menghentikannya, matikan **Share the clipboard with Android** di halaman **Advanced**
  (`dsa settings set clipboard off`).

## Google Play dan video yang dilindungi

DSA tidak menyertakan perangkat lunak Google. Jika Anda menginginkannya, DSA mengunduhnya dari
sumbernya untuk Anda, setelah menunjukkan sumber dan ketentuannya. Keduanya berlaku sejak
Android dimulai berikutnya, dan keduanya dapat dihapus lagi.

- **Google Play.** Di halaman **Advanced** DSA Settings, pilih **Install** di bawah **Google
  Play**, atau jalankan `dsa gapps install`. DSA mengunduh paket publik LiteGapps Core (layanan
  Google Play, Play Store, dan sinkronisasi akun) untuk versi Android Anda. Android ini tidak
  tersertifikasi Google, sehingga Google mungkin menolak Anda masuk sampai Anda mendaftarkan
  perangkat: **Register this device with Google** menunjukkan Android ID dan membuka halaman
  pendaftaran. `dsa gapps status` menunjukkan statusnya, dan `dsa gapps remove` menghapus
  aplikasi Google, bahkan saat Android berhenti.
- **Widevine.** Aplikasi streaming memerlukan modul Widevine Google untuk memutar video yang
  dilindungi. Di halaman **Advanced**, pilih **Get** di bawah **Protected content**, atau
  jalankan `dsa drm widevine install`. DSA mengunduh image emulator Android milik Google untuk
  versi Android Anda (sekitar 2 GB), hanya menyimpan modul Widevine tanpa diubah, dan
  menghapus sisanya. Widevine di sini memiliki tingkat keamanan L3, sehingga layanan streaming
  mengirim paling tinggi definisi standar; untuk definisi tinggi, gunakan browser atau aplikasi
  Windows. `dsa drm widevine remove` menghapusnya.

## Aplikasi yang hanya dibuat untuk ARM

Di PC x64, aplikasi yang kodenya hanya untuk prosesor ARM berjalan melalui penerjemah ARM.
Android di PC x64 sudah dilengkapi satu penerjemah, yaitu penerjemah sumber terbuka Digitalis,
dan menggunakannya sejak pertama kali dimulai. Anda dapat menggunakan penerjemah lain:

- **Penerjemah ARM Google.** Di halaman **Advanced**, pilih **Get** di samping **Google's ARM
  translator**, atau jalankan `dsa arm-translation fetch google`. DSA terlebih dahulu
  menunjukkan asalnya dan ketentuan Google
  ([Android SDK License](https://developer.android.com/studio/terms)), dan tidak mengunduh
  apa pun sampai Anda menyetujuinya. PC Anda kemudian mengunduh image emulator Android yang
  ditawarkan Google untuk versi Android Anda (sekitar 2 GB), hanya menyimpan penerjemahnya, dan
  menghapus sisanya. DSA tidak menyertakan atau meneruskan penerjemah Google.
- **Paket yang Anda bangun atau dapatkan dari tempat lain**, seperti yang dibuat dari
  penerjemah sumber terbuka Digitalis: `dsa arm-translation install <folder>`.

Lalu mulai ulang Android: hentikan di halaman **General** (umum) DSA Settings, lalu buka
sebuah aplikasi.

Untuk kembali ke penerjemah bawaan Android, pilih **Use** di samping **Use the default
translator** di halaman **Advanced**, atau jalankan `dsa arm-translation use-default`. Ini
menghapus paket yang Anda instal, sejak Android dimulai berikutnya.

`dsa arm-translation status` memberitahukan penerjemah mana yang digunakan Android. Sakelar
**Run apps built only for ARM** di halaman **Advanced** mengaktifkan atau mematikan terjemahan
tanpa menghapus apa pun. Untuk aplikasi yang memiliki kode ARM dan x64, **Run as ARM** di
halaman **Apps** (`dsa apps run-as-arm <package> on`) menjalankan kode ARM-nya. Hanya aplikasi
ARM 64-bit (`arm64-v8a`) yang dapat berjalan. Di PC ARM64, Android menjalankan aplikasi ARM
secara langsung dan tidak memerlukan penerjemah.

## Akses root

Sebagian aplikasi, seperti alat pencadangan, memerlukan root. DSA memberikannya seperti `sudo`
di PC Linux: hanya saat administrator PC menyetujuinya, satu permintaan setiap kali. Fitur ini
nonaktif secara bawaan, dan aplikasi yang tidak diizinkan sama sekali tidak dapat melihat `su`.

1. Administrator mengaktifkan **Let Android apps ask for root** di bawah **Root access** pada
   halaman **Advanced** DSA Settings (`dsa settings set root-elevation on`). Pengaturan ini
   berlaku sejak Android dimulai berikutnya.
2. Di halaman **Apps**, atur **Root access** aplikasi tersebut:
   - **Root: not allowed** (bawaan): aplikasi tidak memiliki `su`.
   - **Root: ask each time**: setiap kali aplikasi menggunakan `su`, Windows menampilkan prompt
     administratornya (UAC) dengan nama dan ikon aplikasi itu sendiri. Pengguna standar
     memasukkan kata sandi administrator. Persetujuan berlaku 15 menit untuk aplikasi
     tersebut.
   - **Root: remember (no asking)**: aplikasi menggunakan `su` tanpa bertanya. Memilih opsi ini
     memerlukan persetujuan administrator.

   Dari baris perintah: `dsa apps root <package> off|ask|remember`.

Dengan mode pengembang aktif, `adb shell` juga dapat menggunakan `su`. Ia memiliki barisnya
sendiri di halaman **Apps**, **adb shell**, dan secara bawaan bertanya setiap kali
(`dsa apps root adb ask`). Prompt-nya bernama **adb shell** dan menampilkan perintahnya.

Saat bagian dari Android itu sendiri yang meminta, prompt menyebutkan namanya: **Android
system**, **Android phone services**, **Android Bluetooth**, **Android NFC**, atau **Android
system service**. Aplikasi yang berbagi satu ID pengguna disetujui sebagai satu kelompok, dan
prompt mencantumkan semuanya.

Setiap permintaan disimpan di log layanan beserta aplikasinya, waktunya, perintahnya, dan
apakah permintaan itu disetujui. Mematikan fitur ini menyembunyikan `su` dari semua aplikasi
sekaligus, tanpa memulai ulang.

---

Android adalah merek dagang Google LLC. Windows adalah merek dagang grup perusahaan Microsoft.
DSA tidak berafiliasi dengan, tidak didukung oleh, dan tidak disponsori oleh Google maupun
Microsoft.
