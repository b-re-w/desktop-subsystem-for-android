# 設定

[English](../settings.md) · [한국어](../ko/settings.md) · **日本語** · [简体中文](../zh-CN/settings.md) · [繁體中文](../zh-TW/settings.md) · [Español](../es/settings.md) · [Français](../fr/settings.md) · [Deutsch](../de/settings.md) · [Português (Brasil)](../pt-BR/settings.md) · [Русский](../ru/settings.md) · [Italiano](../it/settings.md) · [Tiếng Việt](../vi/settings.md) · [Bahasa Indonesia](../id/settings.md) · [Türkçe](../tr/settings.md)

**DSA Settings** は、Windows 上の Desktop Subsystem for Android™ (DSA) の設定アプリです。
スタート メニューから開きます。
表示言語は Windows の表示言語に従って英語または韓国語になり、テーマも Windows のテーマに従います。
キーボードとスクリーン リーダーで操作できます。

DSA Settings で行う変更はすべて `dsa` コマンドを通るので、コマンド ラインからも同じ値が見えます。

```
dsa settings                        # すべての設定とその値
dsa settings set <name> <value>     # 1 つを変更する。スイッチは on または off を取る
```

一部の設定は PC のすべてのユーザーに適用されるため、変更できるのは管理者だけです。
DSA Settings ではそうした設定に盾のアイコンが付き、変更するときに管理者の承認を求めます。
以下では「(admin)」と記しています。

## ページ

| ページ | 内容 | コマンド ライン |
|---|---|---|
| **General** (全般) | Android の状態、**Start**、**Stop**、**Freeze Android now**、最後に起きた問題、**Open my log folder**、**Refresh** | `dsa status`, `dsa start`, `dsa stop`, `dsa freeze` |
| **Memory** (メモリ) | **Memory limit**、Android の VM のサイズ、各部分の使用量 | `memory-limit`, `dsa memory` |
| **Apps** (アプリ) | インストール済みのアプリ: **Open**、**App info**、**Uninstall**、**Create desktop shortcut**。アプリごとに: **Resizable**、ウィンドウが開くときのサイズ、**Scaling on another monitor**、**Full screen**、**Bars hidden**、**Run as ARM**、**Root access**。**adb shell** の root 権限の行 | `dsa apps`, `dsa app-window`, `dsa apps run-as-arm`, `dsa apps root` |
| **Running in the background** (バックグラウンドで実行中) | ウィンドウなしで動いているアプリ: **Open**、**Stop**。停止する前に確認するかどうか | `dsa background`, `stop-without-asking` |
| **Widgets** (ウィジェット) | デスクトップのウィジェットと **Add to desktop** | `dsa widgets` |
| **Display & GPU** (ディスプレイと GPU) | **Render GPU**、**GPU of the primary monitor**、**App full screen**、**Apps that hide their bars**、**Turn off Windows touch gestures in DSA app windows** (Windows 11 のみ、既定でオン)、**Restart the window manager** | `render-gpu`, `dsa gpus`, `app-fullscreen`, `app-immersive`, `touch-gestures-off` |
| **Device identity** (デバイス ID) | 最初にこのコンピューターの情報 (読み取り専用): 名前、メーカーとモデル、**Copy** の付いた **Device specifications**。続いて、Android がアプリに報告するメーカー、モデル、ビルドのフィンガープリント | `dsa settings device`, `device.<field>` |
| **Network** (ネットワーク) | Android がネットワークにつながる方法、そのアドレス、名前に応答するもの (DNS)、ゲートウェイ、**Advanced networking**、**Use this PC's added root certificates**、**Developer mode** (adb)、**Connect adb automatically** | `dsa network`, `advanced-networking`, `host-certificates`, `developer-mode`, `adb-port`, `adb-auto-connect` |
| **Updates** (アップデート) | **Android**: 動いているバージョンとビルド、セキュリティ パッチ レベル、追従しているバージョン、進み具合の付いた Android のアップデートと **Check now** (admin)、**Restart Android into updates by itself**、**Update the Android image automatically** (admin)、**Version to use** (admin)。**DSA**: **Update DSA automatically** (admin)、**Get pre-release versions** (admin)、**Install the newest version of DSA now** | `dsa android version`, `dsa android use`, `image-auto-restart`, `image-updates`, `program-updates`, `prerelease-updates`, `dsa update` |
| **Advanced** (詳細) | クリップボード、ファイル、Web リンク、アイドル時のフリーズ、バックアップ、リセット、ARM アプリ、Google Play、保護されたコンテンツ、root 権限、トレース (admin)、ログ フォルダー | 下記を参照 |
| **Logs** (ログ) | Android のライブ ログ、**Save diagnostics** | `dsa logcat`, `dsa logs collect` |
| **About** (バージョン情報) | バージョン、ライセンス、ソース コード | |

## Advanced ページ

| 設定 | 既定 | コマンド ライン |
|---|---|---|
| **Share the clipboard with Android** | オン | `clipboard` |
| **Share Windows folders with Android** | オン | `file-sharing` |
| **Shared folders** | 6 つすべて | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`, `dsa share` |
| **Freeze Android when idle** | オフ | `freeze` |
| **Open web links in Android apps** | オフ | `links-in-android` |
| **Back up apps' data to this PC**、**Back up now**、**Restore** | オフ | `dsa backup on`, `dsa backup now`, `dsa backup restore` |
| **Recovery key**、**Use a recovery key** | | `dsa backup key show`, `dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator** (ARM64 にはありません) | | `dsa arm-translation fetch google` |
| **Use the default translator** (ARM64 にはありません) | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**、**Remove Google Play** | 未インストール | `dsa gapps install`, `dsa gapps remove` |
| **Widevine** (**Protected content** の下) | 未インストール | `dsa drm widevine install`, `dsa drm widevine remove` |
| **Let Android apps ask for root** (admin) | オフ | `root-elevation` |
| **Input trace** (admin) | オフ | `input-trace` |
| **Resize trace** (admin) | オフ | `resize-trace` |

バックアップは、アプリのデータの暗号化されたコピーを、Windows が保管するキーとともにユーザー フォルダーに保存します。
別の PC で、または Windows を再インストールした後に復元するには回復キーが必要なので、PC とは別の場所に書き留めておいてください。
Google Play、Widevine、root 権限については [アプリとウィンドウ](apps.md) で説明しています。

## 変更が適用されるタイミング

- **Render GPU**: ウィンドウ マネージャーが再び起動したとき。次にサインインしたとき、または **Restart the window manager** を選んだときです。
- **Memory limit**、**Device identity**、**Advanced networking**、**Run apps built only for ARM**、Google Play、Widevine、root 権限をオンにすること、Android のバージョン: 次に Android を起動したとき。
  Android を再起動するには、**General** ページで Android を停止してから、アプリを開きます。
- アプリのウィンドウの設定: アプリが次に開くウィンドウから。
- root 権限をオフにすることを含め、それ以外のすべて: すぐに。

## Android が名前を解決する仕組み

Android のアプリが調べる名前には、すべてコンピューターが、Windows アプリと同じリゾルバーで応答します (Linux と macOS ではシステムのリゾルバーです)。
そのため、hosts ファイル、会社の名前解決のルール、VPN 独自の DNS サーバー、DNS フィルターは、コンピューターと同じように Android のアプリにも適用されます。
Android が自分で DNS サーバーに問い合わせることはありません。

- **Advanced networking** をオンにしても同じです。
  コンピューターは同じネットワーク上にあってその名前を知っており、コンピューター上の VPN や DNS フィルターも引き続き適用されます。
- Android 自身の **プライベート DNS** は既定でオフです。
  オンにすると、コンピューターを経由せずに DNS サーバーに問い合わせてしまうからです。
  Android の設定でプライベート DNS サーバーを選んだ場合は、Android はそれを使います。
- DSA がしばらく応答できない場合は、応答できるようになるまで、Android はネットワークの DNS サーバーに問い合わせます。
- **Network** ページの **DNS** の行と `dsa network` には、いま応答しているのがコンピューターのリゾルバーか、ネットワークの DNS サーバーかが表示されます。後者の場合は、そのサーバーが一覧表示されます。

## コマンド ラインで指定できる値

- `render-gpu`: `display` (プライマリ モニターの GPU、既定)、`integrated`、`discrete`、または `dsa gpus` が一覧表示する GPU の名前の一部。
- `memory-limit`: `auto` (既定。固定の上限はなく、PC のメモリから Windows が確保する分を除いた量まで)、または `4g` や `3072` (MiB) のようなサイズ。
- `shared-folders`: `all`、`none`、または `Documents,Pictures` のようにコンマで区切った名前。
- `adb-port`: 1 から 65535 までの TCP ポート。既定は 58526 です。
- `app-fullscreen`: アプリのはっきりした全画面表示の要求に対して、`screen` (既定) または `window`。
  `app-immersive`: バーを隠すだけのアプリに対して、`screen` または `window` (既定)。
  [全画面表示](apps.md#全画面表示) を参照してください。
- `touch-gestures-off`: `on` (既定) にすると、DSA のアプリ ウィンドウが前面にある間は Windows 11 の 3 本指と 4 本指のタッチ ジェスチャがオフになり、アプリは最大 10 本の指を受け取れます。
  ほかのウィンドウが前面に来ると、あなた自身の設定に戻します。
  Windows が拒否した場合はこの設定は使えず、設定アプリにその理由が表示されます。
  その場合でも、Windows の設定の [Bluetooth とデバイス] > [タッチ] で、すべてのアプリについてジェスチャをオフにできます。
- `host-certificates`: `on` (既定) にすると、会社やセキュリティ プログラムのルート証明書など、この PC に追加されたルート証明書を Android アプリが信頼できます。
- `adb-auto-connect`: `on` (既定) にすると、開発者モードがオンの間、`adb connect` なしで `adb devices` と Android Studio に Android が表示されます。
- `image-auto-restart`: `on` (既定) にすると、アプリのウィンドウが開いていないときに、ダウンロードしたイメージの更新ですぐに Android が再起動します。
  そうでない場合、DSA はまず確認するか、ウィンドウを閉じるまで待ちます。
- `prerelease-updates`: `on` にすると安定版に加えてプレリリースも受け取り、`off` にすると安定版だけを受け取ります。
  初期値は使ったインストーラーで決まります。
  プレリリースのインストーラーではオンになり、安定版のインストーラーではオフのままで、アップグレードしてもあなたの選択は変わりません。
  1.0 より前のリリースはすべてプレリリースです。
  オフにしても DSA が古いバージョンに戻ることはなく、インストール済みのものより新しい安定版のリリースを待ちます。
- `root-elevation`: `on` にすると、**Apps** ページで許可したアプリが root 権限を求められます。
  各アプリのレベルは `dsa apps root <package> off|ask|remember` で設定します。
- `device.<field>`: `manufacturer`、`brand`、`model`、`device`、`name`、`characteristics`、`fingerprint`、または `fingerprint.release` のようなフィンガープリントの一部。
  `dsa settings reset device` ですべての値が元に戻ります。

---

Android は Google LLC の商標です。
Windows は Microsoft グループ企業の商標です。
DSA は Google および Microsoft とは提携しておらず、両社による承認や後援も受けていません。
