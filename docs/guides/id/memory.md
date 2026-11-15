# Memori dan perilaku saat diam

[English](../memory.md) · [한국어](../ko/memory.md) · [日本語](../ja/memory.md) · [简体中文](../zh-CN/memory.md) · [繁體中文](../zh-TW/memory.md) · [Español](../es/memory.md) · [Français](../fr/memory.md) · [Deutsch](../de/memory.md) · [Português (Brasil)](../pt-BR/memory.md) · [Русский](../ru/memory.md) · [Italiano](../it/memory.md) · [Tiếng Việt](../vi/memory.md) · **Bahasa Indonesia** · [Türkçe](../tr/memory.md)

Desktop Subsystem for Android™ (DSA) mengembalikan memori ke PC Anda selama Android berjalan,
tidak hanya saat Android berhenti. Tidak ada yang perlu disiapkan untuk ini. Halaman ini
menjelaskan apa yang terjadi, dan cara melihatnya.

## Melihat berapa banyak memori yang digunakan Android

Buka halaman **Memory** (memori) DSA Settings, atau jalankan `dsa memory`. Keduanya
menampilkan:

- **On this PC** (di PC ini): memori yang dipegang mesin virtual Android, pengelola jendela,
  dan setiap jendela aplikasi.
- **Inside Android** (di dalam Android): memori yang digunakan, cache, memori terkompresi, dan
  proses terbesar. Android melaporkan ini beberapa menit setelah dimulai.

## Doze, keadaan diam

Saat setiap jendela Android telah ditutup atau diminimalkan selama 30 detik, layar Android
mati dan Android masuk ke Doze, seperti ponsel di dalam saku.

- Pemberitahuan, alarm, dan widget tetap berfungsi dalam Doze. CPU beristirahat di sela-selanya.
- `dsa status` menampilkan `Dozing`. Halaman **General** (umum) DSA Settings menampilkan
  **Dozing (idle)**.
- Menampilkan jendela Android mana pun langsung membangunkan Android.

Android juga mengikuti mode tidur PC Anda. Android menyimpan statusnya sebelum PC tidur dan
menyetel jamnya saat PC bangun.

## Pembekuan (nonaktif secara bawaan)

Pembekuan menghentikan setiap proses di Android. Android tidak menggunakan CPU saat itu, tetapi
pemberitahuan, alarm, dan widget juga berhenti. Itulah sebabnya DSA tidak pernah membekukan
Android kecuali Anda memintanya.

- Untuk membekukan sekarang: **Freeze** di bawah **Freeze Android now** di halaman **General**,
  atau `dsa freeze`.
- Untuk membekukan setelah 30 menit dalam Doze: aktifkan **Freeze Android when idle** di
  halaman **Advanced** (lanjutan), atau `dsa settings set freeze on`.
- Membuka jendela Android, atau input di dalamnya, melanjutkan Android.

## Bagaimana memori dikembalikan

- Mesin virtual Android mengambil memori dari PC Anda hanya saat Android pertama kali
  menggunakannya.
- Memori yang dibebaskan Android dikembalikan ke PC Anda selama Android berjalan.
- Saat jendela terakhir sebuah aplikasi ditutup, Android mengakhiri proses aplikasi itu,
  sehingga memorinya kembali dalam hitungan detik. Aplikasi yang memiliki widget di desktop,
  atau yang masih bekerja di latar belakang (seperti pemutar musik), tetap berjalan.
- Aplikasi yang mempertahankan layanan latar belakang tanpa jendela, tanpa pemberitahuan,
  tanpa suara, dan tanpa widget dihentikan setelah lima menit. Jika Android terus memulainya
  lagi, DSA menunggu lebih lama setiap kali, hingga satu jam.
- Selama tidak ada jendela aplikasi yang terbuka, Android membuang cache file-nya dan
  memindahkan memori yang menganggur ke memori terkompresi, sehingga lebih banyak memori yang
  dapat dikembalikan.
- Pengelola jendela mengembalikan memori yang tidak digunakannya 30 detik setelah jendela
  aplikasi terakhir ditutup.

## Batas memori

Batasnya **Automatic** secara bawaan, yang tidak menetapkan batas tetap. Android dimulai dengan
2 GB dan bertambah selama berjalan, hingga memori PC Anda dikurangi bagian yang disisakan untuk
Windows (10% dari memori atau 2 GB, mana yang lebih besar). Android hanya memegang apa yang
digunakannya.
Untuk menetapkan batas, gunakan **Memory limit** di halaman **Memory**, atau:

```
dsa settings set memory-limit 4g     # 4 GiB
dsa settings set memory-limit auto   # kembali ke otomatis
```

Batas baru berlaku sejak Android dimulai berikutnya.

---

Android adalah merek dagang Google LLC. Windows adalah merek dagang grup perusahaan Microsoft.
DSA tidak berafiliasi dengan, tidak didukung oleh, dan tidak disponsori oleh Google maupun
Microsoft.
