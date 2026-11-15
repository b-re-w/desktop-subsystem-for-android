# メモリとアイドル時の動作

[English](../memory.md) · [한국어](../ko/memory.md) · **日本語** · [简体中文](../zh-CN/memory.md) · [繁體中文](../zh-TW/memory.md) · [Español](../es/memory.md) · [Français](../fr/memory.md) · [Deutsch](../de/memory.md) · [Português (Brasil)](../pt-BR/memory.md) · [Русский](../ru/memory.md) · [Italiano](../it/memory.md) · [Tiếng Việt](../vi/memory.md) · [Bahasa Indonesia](../id/memory.md) · [Türkçe](../tr/memory.md)

Desktop Subsystem for Android™ (DSA) は、Android が停止したときだけでなく、動いている間もメモリを PC に返します。
そのための設定は必要ありません。
このページでは、何が起きているのかと、それを確認する方法を説明します。

## Android のメモリ使用量を確認する

DSA Settings の **Memory** (メモリ) ページを開くか、`dsa memory` を実行します。
どちらにも次の情報が表示されます。

- **On this PC** (この PC 上): Android の仮想マシン、ウィンドウ マネージャー、各アプリ ウィンドウが確保しているメモリ。
- **Inside Android** (Android 内): 使用中のメモリ、キャッシュ、圧縮メモリ、最も大きいプロセス。
  Android は起動の数分後にこれを報告します。

## アイドル状態の Doze

すべての Android ウィンドウが閉じられるか最小化されてから 30 秒たつと、Android の画面がオフになり、ポケットの中のスマートフォンと同じように Android は Doze に入ります。

- Doze の間も、通知、アラーム、ウィジェットは動き続けます。その合間に CPU は休みます。
- `dsa status` には `Dozing` と表示されます。DSA Settings の **General** (全般) ページには **Dozing (idle)** と表示されます。
- どれか 1 つでも Android のウィンドウを表示すると、Android はすぐに復帰します。

Android は PC のスリープにも従います。
PC がスリープする前に状態を保存し、PC が復帰すると時計を合わせます。

## フリーズ (既定ではオフ)

フリーズすると、Android 内のすべてのプロセスが止まります。
その間 CPU はまったく使いませんが、通知、アラーム、ウィジェットも止まります。
そのため、あなたが求めない限り、DSA が Android をフリーズすることはありません。

- すぐにフリーズするには: **General** ページの **Freeze Android now** にある **Freeze** を選ぶか、`dsa freeze` を実行します。
- Doze が 30 分続いたらフリーズするには: **Advanced** (詳細) ページで **Freeze Android when idle** をオンにするか、`dsa settings set freeze on` を実行します。
- Android のウィンドウを開くか、ウィンドウで何か入力すると、Android は再開します。

## メモリが返される仕組み

- Android の仮想マシンは、Android が初めて使うときにだけ PC からメモリを受け取ります。
- Android が解放したメモリは、Android の実行中に PC へ返されます。
- アプリの最後のウィンドウが閉じると、Android はそのアプリのプロセスを終了するので、そのメモリは数秒以内に返されます。
  デスクトップにウィジェットがあるアプリや、音楽プレーヤーのようにバックグラウンドで仕事を続けているアプリは、動き続けます。
- ウィンドウも通知も音もウィジェットもないままバックグラウンド サービスを動かし続けるアプリは、5 分後に停止されます。
  Android がそれを何度も起動し直す場合、DSA は毎回待つ時間を延ばし、最大で 1 時間まで延ばします。
- アプリのウィンドウが 1 つも開いていない間、Android はファイル キャッシュを捨て、使われていないメモリを圧縮メモリに移すので、より多くのメモリを返せます。
- ウィンドウ マネージャーは、最後のアプリ ウィンドウが閉じてから 30 秒後に、使っていないメモリを返します。

## メモリの上限

上限は既定で **Automatic** (自動) で、固定の上限はありません。
Android は 2 GB で起動し、動いている間に、PC のメモリから Windows が確保する分 (メモリの 10% か 2 GB のどちらか大きいほう) を除いた量まで増えていきます。
Android が確保するのは、実際に使う分だけです。
上限を設定するには、**Memory** ページの **Memory limit** を使うか、次のコマンドを実行します。

```
dsa settings set memory-limit 4g     # 4 GiB
dsa settings set memory-limit auto   # 自動に戻す
```

新しい上限は、次に Android を起動したときに適用されます。

---

Android は Google LLC の商標です。
Windows は Microsoft グループ企業の商標です。
DSA は Google および Microsoft とは提携しておらず、両社による承認や後援も受けていません。
