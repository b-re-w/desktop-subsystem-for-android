# アンインストール

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · **日本語** · [简体中文](../zh-CN/uninstall.md) · [繁體中文](../zh-TW/uninstall.md) · [Español](../es/uninstall.md) · [Français](../fr/uninstall.md) · [Deutsch](../de/uninstall.md) · [Português (Brasil)](../pt-BR/uninstall.md) · [Русский](../ru/uninstall.md) · [Italiano](../it/uninstall.md) · [Tiếng Việt](../vi/uninstall.md) · [Bahasa Indonesia](../id/uninstall.md) · [Türkçe](../tr/uninstall.md)

## Windows

Desktop Subsystem for Android™ (DSA) は、次のどちらかの方法でアンインストールします。

- Windows の [設定] > [アプリ] > [インストールされているアプリ] で Desktop Subsystem for Android を探し、[アンインストール] を選びます。
- セットアップをもう一度実行し、**Uninstall** を選びます。

アンインストールすると、サインインしているかどうかにかかわらず、PC のすべてのユーザーについて次のものが削除されます。

- プログラム、`DsaHost` サービス、`dsa` コマンド、DSA Settings、APK Installer。
- ダウンロードした Android イメージと更新。
- Android アプリのスタート メニューの項目と、[インストールされているアプリ] の項目。
- エクスプローラーの **Android**、`.apk` ファイルの関連付け、コンテキスト メニューの項目。
- DSA の設定とログ。

各ユーザーの Android ディスク `%LOCALAPPDATA%\DSA\android.vhdx` は **残ります**。
そのため、後でインストールし直すと、アプリとデータがまた見つかります。
データも削除するには、アンインストールの前に `dsa reset-data --yes` を実行するか、アンインストールの後にそのファイルを削除してください。

フォルダーのリダイレクトでユーザーのスタート メニューが移動している場合、そのユーザーの Android アプリの項目は残ります。
アンインストーラーは既定の場所しか探さないからです。
手動で削除してください。

DSA は WSL の設定を一切変更しないので、WSL の設定はそのままです。

## Linux (未リリース)

```
sudo apt remove desktop-subsystem-for-android    # Debian, Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

パッケージを削除すると、そのパッケージが追加したパッケージ リポジトリの項目も削除されます。

---

Android は Google LLC の商標です。
Windows は Microsoft グループ企業の商標です。
DSA は Google および Microsoft とは提携しておらず、両社による承認や後援も受けていません。
