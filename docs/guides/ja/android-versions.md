# Android のバージョン

[English](../android-versions.md) · [한국어](../ko/android-versions.md) · **日本語** · [简体中文](../zh-CN/android-versions.md) · [繁體中文](../zh-TW/android-versions.md) · [Español](../es/android-versions.md) · [Français](../fr/android-versions.md) · [Deutsch](../de/android-versions.md) · [Português (Brasil)](../pt-BR/android-versions.md) · [Русский](../ru/android-versions.md) · [Italiano](../it/android-versions.md) · [Tiếng Việt](../vi/android-versions.md) · [Bahasa Indonesia](../id/android-versions.md) · [Türkçe](../tr/android-versions.md)

Desktop Subsystem for Android™ (DSA) は Android の最新の 2 つのメジャー バージョン、現在は Android 16 と Android 17 に対応しています。
既定では最新のほうを実行します。
どちらか一方でしか動かないアプリもあるので、選べるようになっています。

## 動いているバージョンを確認する

DSA Settings の **Updates** (アップデート) ページを開きます。
**Android** の下に、バージョン、ビルド、セキュリティ パッチ レベル、追従しているバージョン、アップデートの状況がダウンロードの進み具合とともに表示されます。
**Check now** を選ぶと、すぐにアップデートを探すようサービスに求めます。
または、次のコマンドを実行します。

```
dsa android version   # 動いているバージョンと、選べるバージョン
```

## バージョンを変更する

PC のすべてのユーザーが同じ Android のバージョンを使うので、変更できるのは管理者だけです。

1. DSA Settings の **Updates** ページを開きます。
2. **Version to use** (使用するバージョン) で、バージョンまたは **Always the newest** を選びます。
3. **Change** を選び、管理者確認のプロンプトを承認します。

選んだバージョンがダウンロードされ、次に Android を起動したときから使われます。
管理者のターミナルからは次のとおりです。

```
dsa android use 16
dsa android use latest
```

### 古いバージョンに移るとデータが消去されます

Android は、新しいバージョンが書き込んだデータを読めません。
そのため、古いメジャー バージョンに移ると、PC のすべてのユーザーの Android のデータ (インストールしたアプリ、アカウント、すべてのアプリのデータ) が消去されます。
DSA の設定は残ります。
これは元に戻せません。

DSA Settings では、**Erase data and change** で確認を求められます。
コマンド ラインでは `--erase-data` を付けます。

```
dsa android use 16 --erase-data
```

新しいバージョンに移る場合は、アプリとデータはそのまま残ります。

## アップデート

- 選んだバージョンには、セキュリティ アップデートが届き続けます。
- バージョンのサポートが終わると、DSA は次のバージョンに移り、アプリはそのまま残ります。
- DSA が自分から古いメジャー バージョンに移ることはありません。
- 各イメージは、使う前に署名された一覧と照合されます。
  起動に失敗したイメージは、前のイメージに置き換えられます。

管理者は、DSA Settings の **Updates** ページにある **Update the Android image automatically**、または `dsa settings set image-updates off` で、イメージの更新をオフにできます。

---

Android は Google LLC の商標です。
Windows は Microsoft グループ企業の商標です。
DSA は Google および Microsoft とは提携しておらず、両社による承認や後援も受けていません。
