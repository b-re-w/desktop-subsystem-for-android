# ファイル

[English](../files.md) · [한국어](../ko/files.md) · **日本語** · [简体中文](../zh-CN/files.md) · [繁體中文](../zh-TW/files.md) · [Español](../es/files.md) · [Français](../fr/files.md) · [Deutsch](../de/files.md) · [Português (Brasil)](../pt-BR/files.md) · [Русский](../ru/files.md) · [Italiano](../it/files.md) · [Tiếng Việt](../vi/files.md) · [Bahasa Indonesia](../id/files.md) · [Türkçe](../tr/files.md)

Desktop Subsystem for Android™ (DSA) では、Android のファイルをエクスプローラーで開いたり、Windows のフォルダーを Android アプリと共有したり、Android アプリにファイルを送ったりできます。
このページでは、Windows 上の DSA について説明します。

## Android のファイルをエクスプローラーで開く

次のどれかを使います。
どれも、Android が動いていないときは Android を起動します。

- エクスプローラーのナビゲーション ウィンドウで、WSL の **Linux** のすぐ下にある **Android**。
- DSA Settings の **Advanced** (詳細) ページにある **Android files**。
- `dsa files open`。`dsa files` は、ストレージを開かずにその場所を表示します。

表示されるのは Android の共有ストレージで、写真やダウンロードなど、Android アプリが保存したファイルがあります。
ほかのフォルダーと同じように、ファイルをコピーして出し入れできます。
エクスプローラーは Windows の WebDAV クライアントを通してストレージにアクセスします。
ストレージが応答しない場合は、DSA が理由とともにエラーを表示します。

## Windows のフォルダーを Android アプリと共有する

ドキュメント、ダウンロード、ピクチャ、ミュージック、ビデオ、デスクトップの各フォルダーは、Android の共有ストレージの `Host` の下に、`Host/Pictures` のように表示されます。
Android アプリはそこでファイルを開いたり保存したりできます。
アプリにはまず Android のストレージの権限が必要で、Windows はあなた自身のアカウントに許されている操作だけをアプリに許可します。

共有は既定でオンです。
変更するには、DSA Settings の **Advanced** ページを開きます。

- **Share Windows folders with Android** をオフにすると、どのフォルダーも共有しません。
- **Shared folders** でフォルダーのチェックを外すと、そのフォルダーの共有をやめます。
  フォルダーはすぐに Android から消えます。

コマンド ラインでは次のとおりです。

```
dsa settings set shared-folders Documents,Pictures   # これらだけ。all または none も可
dsa settings set file-sharing off                     # どのフォルダーも共有しない
```

以前のバージョンでは、`Host` の代わりに `Windows` という名前のフォルダーを使っていました。
古いフォルダーは空になってから Android が削除するので、そこに保存したファイルは残ります。

## Android アプリにファイルを送る

- **ドラッグ アンド ドロップ。** エクスプローラーからアプリのウィンドウにファイルをドラッグします。
  ファイルはそのアプリに送られます。
  APK の場合は、代わりに APK Installer に送られます。
  フォルダーは送られません。
- **コンテキスト メニュー。** ファイルを右クリックし、**Open in an Android app** または **Share with an Android app** を選びます。
  Windows 11 では、これらは [その他のオプションを確認] の中にあります。
  その後、どのアプリでファイルを受け取るかを Android が尋ねます。
- **コマンド ライン。** `dsa open <file>` と `dsa share <file>` です。

これらの方法で送れるファイルは 64 MiB までです。
それより大きいファイルは、何も送られる前に拒否されます。
大きなファイルをアプリに渡すには、代わりに共有フォルダーに置いてください。

## Android からファイルを受け取る

- Android で `Host/<folder>` の下に保存します。
  すると、そのファイルは PC のそのフォルダーに入ります。
- または、エクスプローラーで **Android** を開き、ファイルをコピーして取り出します。

---

Android は Google LLC の商標です。
Windows は Microsoft グループ企業の商標です。
DSA は Google および Microsoft とは提携しておらず、両社による承認や後援も受けていません。
