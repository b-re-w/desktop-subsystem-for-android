# APK のインストール

[English](../install-apks.md) · [한국어](../ko/install-apks.md) · **日本語** · [简体中文](../zh-CN/install-apks.md) · [繁體中文](../zh-TW/install-apks.md) · [Español](../es/install-apks.md) · [Français](../fr/install-apks.md) · [Deutsch](../de/install-apks.md) · [Português (Brasil)](../pt-BR/install-apks.md) · [Русский](../ru/install-apks.md) · [Italiano](../it/install-apks.md) · [Tiếng Việt](../vi/install-apks.md) · [Bahasa Indonesia](../id/install-apks.md) · [Türkçe](../tr/install-apks.md)

Desktop Subsystem for Android™ (DSA) はアプリ ストアを同梱していません。
アプリは、APK Installer または `dsa` コマンドを使って APK ファイルからインストールします。
このページでは、Windows 上の DSA について説明します。

## APK Installer でインストールする

1. エクスプローラーで `.apk` ファイルをダブルクリックします。
   ファイルが **APK Installer** で開きます。
2. APK の内容を確認します。
   ウィンドウは Android を起動せずに次の情報を表示します。
   - アプリの名前、アイコン、パッケージ、バージョン。
   - 発行元。APK には発行元の名前が書かれていないため、署名証明書から取り出します。
   - 入手元。ファイルをダウンロードしたホスト、またはファイルのあるフォルダーです。
   - **Capabilities** (機能) の下に、要求する権限の最初のいくつか。
     **More** を選ぶと、すべての権限、対象の Android のバージョン、サイズ、プロセッサの種類、証明書の SHA-256 が表示されます。

   APK が別の種類のプロセッサ向けのコードしか持たない場合は、ウィンドウに警告が表示されます。
3. **Install** を選びます。
   アプリがすでにインストールされている場合、ボタンは **Update** または **Install again** と表示されます。
   Android が動いていないときは、まず Android が起動します。
4. **Launch when ready** にチェックが入っていると (既定)、インストールが終わるとアプリが開き、インストーラーは閉じます。
   そうでない場合は **Launch** を選ぶか、後でスタート メニューからアプリを開きます。

Android が動いている間は、そのファイルが新規インストールか、アップデートか、同じバージョンの再インストールか、Android が拒否するもの (古いバージョン、または別のキーで署名された同じアプリ) かも、ウィンドウに表示されます。

APK ファイルは、APK Installer、DSA Settings、アプリのウィンドウにドロップすることもできます。
APK はそれぞれ専用のインストーラー ウィンドウで開きます。

## コマンド ラインでインストールする

```
dsa apk-info app.apk            # APK の内容。Android を起動せずにファイルを読む
dsa install app.apk             # インストールする。Android が動いていなければ起動する
dsa install app.apk --progress  # 各ステップも表示する
```

Android が APK を拒否すると、`dsa install` は終了コード 1 で終わり、そのメッセージは `not installed (<reason>):` で始まります。

## インストールが拒否される理由

| 理由 | 意味 | 対処方法 |
|---|---|---|
| `incompatible` | アプリが別の種類のプロセッサ、より新しい Android のバージョン、またはこのデバイスにない機能を必要としている | [ARM 専用アプリ](apps.md#arm-専用アプリ) または [Android のバージョン](android-versions.md) を参照 |
| `conflicting-signature` | アプリがすでに別のキーで署名された状態でインストールされている | 先にインストール済みのアプリをアンインストールする |
| `downgrade` | より新しいバージョンのアプリがインストールされている | 先にインストール済みのアプリをアンインストールする |
| `invalid-apk` | ファイルが、Android が読める署名済みの APK ではない | APK をもう一度入手する |
| `insufficient-storage` | Android にアプリを置く空き容量がない | Android の空き容量を増やす |
| `blocked` | この Android のポリシーがアプリを許可していない | |
| `other` | その他。Android 自身のメッセージが続く | |

APK のサイズは最大 2 GiB です。
インストールは 1 つずつ行われ、2 つ目は 1 つ目が終わるのを待ちます。

## アプリをアンインストールする

次のどれかを使います。

- Windows の [設定] > [アプリ] > [インストールされているアプリ]。
- DSA Settings の **Apps** (アプリ) ページで、アプリの横にある **Uninstall**。
- `dsa apps uninstall <package>`。

Android の一部であるアプリはアンインストールできません。

---

Android は Google LLC の商標です。
Windows は Microsoft グループ企業の商標です。
DSA は Google および Microsoft とは提携しておらず、両社による承認や後援も受けていません。
