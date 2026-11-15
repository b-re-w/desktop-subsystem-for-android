# トラブルシューティングとログ

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · **日本語** · [简体中文](../zh-CN/troubleshooting.md) · [繁體中文](../zh-TW/troubleshooting.md) · [Español](../es/troubleshooting.md) · [Français](../fr/troubleshooting.md) · [Deutsch](../de/troubleshooting.md) · [Português (Brasil)](../pt-BR/troubleshooting.md) · [Русский](../ru/troubleshooting.md) · [Italiano](../it/troubleshooting.md) · [Tiếng Việt](../vi/troubleshooting.md) · [Bahasa Indonesia](../id/troubleshooting.md) · [Türkçe](../tr/troubleshooting.md)

このページは、Desktop Subsystem for Android™ (DSA) が動かない理由を調べ、Android のログを読み、問題の報告を送るのに役立ちます。
対象は Windows で、記載がある箇所では Linux も扱います。

## まず状態を確認する

```
dsa status          # Android の状態、問題、保留中の更新
dsa start --wait    # Android を起動し、起動が終わるまで待つ
```

DSA Settings の **General** (全般) ページにも同じ状態が表示されます。

| `dsa status` | DSA Settings | 意味 |
|---|---|---|
| `Active` | **Running** | Android の起動が終わり、その中の DSA のエージェントが接続しています |
| `Dozing` | **Dozing (idle)** | Android はアイドル状態です。[メモリ](memory.md) を参照してください |
| `Frozen` | **Frozen** | Android は、そのウィンドウのどれかを開くまでフリーズしています |
| `Stalled` | **Not ready** | Android は動いていますが、時間がかかりすぎています。次の行に理由と対処方法が表示されます |

## よくある問題

| 問題 | 対処方法 |
|---|---|
| セットアップで仮想マシン プラットフォームがオフだと表示される | Windows の機能をオンにして再起動し、もう一度セットアップを実行します。[はじめに](getting-started.md) を参照してください |
| アプリのウィンドウが開かない | サインアウトしてから、もう一度サインインします。ウィンドウ マネージャーはサインイン時に起動します |
| アプリがおかしなサイズで開く | `dsa app-window <package> reset` |
| イメージの更新後に Android が起動しない | 何もする必要はありません。DSA が自動で前のイメージに戻し、`dsa status` にその旨が表示されます |
| Android がネットワークにつながらない | DSA Settings の **Network** (ネットワーク) ページを開くか、`dsa network` を実行します。モード、アドレス、DNS サーバー、問題が表示されます |
| Android が壊れていて、ほかに何をしても直らない | Android のデータをリセットします (下記) |

### Android のデータをリセットする

リセットすると、インストールしたすべてのアプリ、アカウント、すべてのアプリのデータが消去されます。
その後、Android は最初にインストールしたときの状態で起動します。
Android イメージと DSA の設定は残ります。

- DSA Settings では: **Advanced** (詳細) ページ > **Reset Android's data** > **Reset**。
- コマンド ラインでは: `dsa reset-data` で失われるものを確認できます。
  `dsa reset-data --yes` で消去します。

## Android のログを読む

Android のログは Android 内のメモリにとどまり、ファイルに書き込まれることはありません。
何かが読み取っている間だけ PC に届きます。

- DSA Settings では: **Logs** (ログ) ページです。タグやテキストで絞り込んだり、一時停止したりできます。
- コマンド ラインでは:

  ```
  dsa logcat                        # ログを追い続ける。Ctrl+C で終了
  dsa logcat -d -b all *:W          # いまログにある内容、警告以上
  dsa logcat ActivityManager:I *:S  # 1 つのタグだけ
  ```

## 問題の報告を送る

1. 診断情報を保存します。
   **Logs** ページで **Save diagnostics** を選ぶか、次のコマンドを実行します。

   ```
   dsa logs collect -o diag.zip
   ```

   1 分以上かかります。
2. その zip ファイルを [issue](https://github.com/thisisthepy/desktop-subsystem-for-android/issues) に添付します。

アーカイブには、Android のログ全体、クラッシュの記録 (tombstone、ANR のトレース、dropbox、pstore)、Android のシステム サービスの概要、この PC にある DSA のログのうちあなたに関係するものが含まれます。
ほかのユーザーの行は含まれません。
集められなかったものは、アーカイブ内の `summary.txt` に書かれています。

## Windows でのログの場所

| フォルダー | ファイル |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log` (ウィンドウ マネージャー)、`dsa-app-window.log` (アプリのウィンドウ) |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log` (サービス)、`android-<SID>.log` (各ユーザーの Android のコンソール、1 回の起動につき最大 2 MiB)、`setup-<version>.log` (自動更新) |

これらを開くには、DSA Settings の **Advanced** ページにある **Open my log folder** と **Open the service's log folder (administrators)** を使います。

## Linux でのログの場所 (未リリース)

サービスは systemd のジャーナルに書き込みます。

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## すべてのコマンド

引数なしの `dsa` は、すべてのコマンドを表示します。
`--json` を付けると、プログラムが読める形式で出力します。

---

Android は Google LLC の商標です。
Windows は Microsoft グループ企業の商標です。
DSA は Google および Microsoft とは提携しておらず、両社による承認や後援も受けていません。
