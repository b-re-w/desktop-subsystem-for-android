# アプリとウィンドウ

[English](../apps.md) · [한국어](../ko/apps.md) · **日本語** · [简体中文](../zh-CN/apps.md) · [繁體中文](../zh-TW/apps.md) · [Español](../es/apps.md) · [Français](../fr/apps.md) · [Deutsch](../de/apps.md) · [Português (Brasil)](../pt-BR/apps.md) · [Русский](../ru/apps.md) · [Italiano](../it/apps.md) · [Tiếng Việt](../vi/apps.md) · [Bahasa Indonesia](../id/apps.md) · [Türkçe](../tr/apps.md)

Android アプリのタスクはそれぞれ専用のデスクトップ ウィンドウとして開き、専用のタスクバー ボタンを持ちます。
スマートフォンの画面を囲むエミュレーターの枠はありません。
このページでは、Windows 上の Desktop Subsystem for Android™ (DSA) について説明します。

## アプリを開く

- **スタート メニュー。** インストールしたアプリには、それぞれの名前とアイコンでスタート メニューの項目があるので、Windows 検索で見つかります。
  Android が動いていないときは、開くとまず Android が起動します。
- **DSA Settings。** **Apps** (アプリ) ページで、アプリの横にある **Open** を選びます。
- **コマンド ライン。** `dsa apps list` は、インストール済みのアプリをパッケージ名とともに一覧表示します。
  `dsa launch <package>` でアプリを開きます。たとえば `dsa launch com.android.settings` です。

ほかのプログラムのスタート メニュー項目にすでに同じ名前がある場合、DSA は `Clock (com.example.clock)` のように名前にパッケージ名を加えます。
DSA が変更や削除をするのは、自分で作った項目だけです。

## デスクトップのショートカットとジャンプ リスト

アプリのアイコンは、Android のホーム画面ではなく、Windows アプリと同じように動作します。

- **デスクトップのショートカット。** DSA Settings の **Apps** ページで、アプリの横にある **Create desktop shortcut** (デスクトップにショートカットを作成) を選びます。
  アプリのタスクバー ボタンを右クリックして **Create desktop shortcut** を選ぶこともできます。
  コマンド ラインでは `dsa apps desktop-shortcut <package>` です。
  ショートカットは、スタート メニューの項目と同じようにアプリを開きます。
- **アプリのショートカット。** メッセンジャーの新しいチャットのような、Android アプリが提供するショートカットは、アプリのタスクバー ボタンまたはスタート メニューの項目を右クリックすると表示されます。
  どれかを選ぶと、アプリのその画面がウィンドウで開きます。Android が動いていないときは、まず Android が起動します。
  アプリがショートカットを変更すると、一覧もそれに従います。
  コマンド ラインでは `dsa apps shortcut <package> <id>` で、ID は `dsa apps info <package> --json` で一覧表示できます。
- **アプリが求めるショートカット。** アプリが自分のショートカットをデスクトップに追加しようとすると、DSA はまずあなたに確認し、同意した場合にだけ追加します。

アップデートの後などにアプリの名前が変わると、スタート メニューの項目、デスクトップのショートカット、タスクバーのピン留めも新しい名前に従い、そのまま使えます。
削除したデスクトップのショートカットが再び作られることはありません。
アプリをアンインストールすると、そのデスクトップのショートカットも削除されます。

## アプリのウィンドウを使う

- タスクごとに、アプリの名前とアイコンが付いた専用のウィンドウ、タスクバー ボタン、Alt+Tab の項目があります。
  タスク マネージャーには、各アプリがそれぞれの名前で表示されます。
- タイトル バーはアプリの一部です。
  アプリの色がウィンドウの上端まで届き、その上に Windows 11 のキャプション ボタンが重なります。
  左端にはアプリのアイコンと名前があります。
  アプリが前の画面に戻れるときはアイコンが戻る矢印に変わり、それを選ぶと戻ります。
- スナップ レイアウト (最大化ボタンにポインターを合わせます)、タイトル バーのドラッグ、ダブルクリックでの最大化、ライブでのサイズ変更が使えます。
  サイズを変えてもアプリは再起動しません。
  最大化したウィンドウはモニターの作業領域いっぱいに広がり、タスクバーは見えたままです。
- Android のナビゲーション バーはありません。
  アプリの下部のバーはウィンドウの下端にあり、その下に空白の帯はありません。
- マウス (ホバーを含む)、タッチ、ペン (筆圧、消しゴム、バレル ボタン)、キーボードがすべて使えます。
  文字は、韓国語やほかの言語を含む Windows の入力方式から入力します。
  変換中の文字は Android のエディターの中にそのまま表示され、候補ウィンドウはカーソルの隣に表示されます。
  Android のキーボード アプリはありません。
- エクスプローラーからアプリのウィンドウにファイルをドラッグしたり、アプリからエクスプローラーにファイルをドラッグしたりできます。
- ナレーター (Narrator) や NVDA などのスクリーン リーダーは、UI Automation を通してアプリを読み上げます。
- ウィンドウを閉じると、Android にそのタスクの終了を求めます。

## ウィンドウ メニュー

タイトル バーを右クリックするか、アプリのアイコンを選ぶか、Alt+Space を押します。
メニューには、Windows のいつもの項目に続いて次の項目があります。

- **Full screen** (全画面表示): F11 キーと同じように、画面全体に表示します。もう一度選ぶか F11 キーを押すと元に戻ります。
- **New window** (新しいウィンドウ): 複数のウィンドウに対応したアプリで、別のウィンドウを開きます。
- **Screenshot** (スクリーンショット): タイトル バーを除いたアプリの画面をクリップボードにコピーし、ピクチャ フォルダーの中のスクリーンショット フォルダーに保存します。
- **Aspect ratio** (縦横比): 大きなウィンドウで引き伸ばされて見えるアプリのために、アプリの縦横比を設定する Android の設定を開きます。
- **Open by default** (既定で開く): アプリが対応するリンクを、アプリで開くかブラウザーで開くかを選びます。
- **App info** (アプリ情報): DSA Settings の **Apps** ページで、そのアプリの項目を開きます。

## 全画面表示

アプリが全画面表示を求めると、ウィンドウは次の 2 つのどちらかの方法で表示します。

- **画面全体に表示**: ウィンドウがモニターいっぱいに広がり、タイトル バーが隠れます。
  ポインターを画面の上端に当てるとタイトル バーが現れ、最小化と閉じるのボタンが使えます。中央のボタンで全画面表示を終了します。
- **ウィンドウ内に表示**: ウィンドウはサイズとタイトル バーを保ったまま、アプリがウィンドウいっぱいに表示されます。

アプリの求め方には 2 種類あり、それぞれに既定の動作があります。

| 要求の種類 | 例 | 既定 |
|---|---|---|
| はっきりした全画面表示の要求 | 動画プレーヤーの全画面ボタン | 画面全体に表示 |
| バーを隠すだけ | ショート動画のフィード | ウィンドウ内に表示 |

既定は DSA Settings の **Display & GPU** (ディスプレイと GPU) ページにある **App full screen** と **Apps that hide their bars** で変更できます。
アプリごとには、**Apps** ページの **Full screen request** と **When only the bars hide** で変更できます。
アプリ自身の選択が優先されます。
F11 キーとウィンドウ メニューの **Full screen** では、設定にかかわらず、いつでも画面全体での全画面表示を切り替えられます。

コマンド ラインでは次のとおりです。

```
dsa settings set app-fullscreen screen|window           # はっきりした要求の既定
dsa settings set app-immersive screen|window            # バーを隠す場合の既定
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## アプリのウィンドウ サイズを設定する

DSA Settings の **Apps** ページで次のことができます。

- **Resizable** (サイズ変更可能) をオフにすると、アプリのウィンドウは開いたときのサイズのままになります。
- アプリのウィンドウが開くときのサイズを選びます。
  Android が選ぶサイズか、縦向きまたは横向きのスマートフォンやタブレットのサイズです。

変更は、アプリが次に開くウィンドウから適用されます。
コマンド ラインでは次のとおりです。

```
dsa app-window <package> --fixed-size on        # サイズ変更も最大化もできない
dsa app-window <package> --start-size 1280x800  # このサイズで開く。auto なら Android が選ぶ
dsa app-window <package> --dpi-scaling system   # 別のモニターでの拡大縮小の方法。下記を参照
dsa app-window <package> reset                  # 既定に戻す
```

## タイトルバーの下からアプリを始める

アプリはウィンドウの一番上、タイトルバーの下まで描画します。そのため、上部のバーが押しつぶされたり、一部が
タイトルバーに隠れたりするアプリがあります。そのようなアプリでは、DSA Settings の **Apps** ページで
**Start below the title bar** (タイトルバーの下から開始) をオンにします。アプリの内容がタイトルバーの下から始まり、タイトルバーはアプリが
そこに表示する色になります。既定はオフで、次にアプリのウィンドウを開いたときから適用されます。コマンドラインでは次のとおりです。

```
dsa app-window <package> --below-title-bar on   # off で既定に戻す
```

## 拡大率の異なるモニターにアプリを移す

アプリは、ウィンドウが開いたモニターの拡大率で、最初のフレームから開きます。
そのため、開いた直後に作り直されることはありません。
拡大率の異なるモニターにウィンドウを移しても、アプリの画面上の大きさは変わりません。
アプリごとに、次のどれかの方法でこれを行います。
方法は **Apps** ページの **Scaling** (拡大縮小) か、`dsa app-window <package> --dpi-scaling` で選びます。

- **Automatic** (`auto`、既定): 実行中に拡大率が変わると表示が崩れるアプリを集めた DSA の組み込み一覧にあるアプリには **System**、それ以外のアプリには **App** を使います。
  一覧には YouTube が入っています。
  YouTube は起動したときの拡大率のレイアウトを保つので、移動した後は再起動するまで下部のタブとミニプレーヤーが崩れたままになるからです。
  そのようなアプリの既定は、Settings に **Automatic (system, recommended for this app)** と表示されます。
  そのアプリについてあなたが選んだ設定が優先されます。
- **App** (`app`): ウィンドウをドラッグしている間は、DSA が新しいモニターに合わせてウィンドウを拡大縮小します。
  ドラッグを終えると、アプリは新しいモニターの拡大率で描き直し、文字はくっきりしたままです。
  Android がアプリの画面を一度作り直して状態 (スクロール位置、入力した文字) を復元するので、移動のたびに短時間描き直しが起こります。
  これは、Windows のプログラムで「アプリケーション」の拡大縮小を選んだときと同じです。
  拡大率の変更を自分で処理すると申告しているアプリでも、この方法が使われます。
  そうしたアプリの中には、画面の一部しか新しい拡大率で描き直さないものがあるからです。
- **System** (`system`): アプリは拡大率を変えず、DSA が新しいモニターに合わせてウィンドウを拡大縮小します。
  アプリが作り直されることはありません。
  拡大すると、文字が少しぼやけて見えることがあります。
- **System (forced)** (`system-forced`): 拡大率の変更を処理すると申告していないアプリにも、再起動せずに新しい拡大率を与えます。
  まだ使えません。
  このバージョンにはない Android の変更が必要なので、それまでは **System** と同じように動きます。

アプリ内の文字は Windows の文字サイズ ([設定] > [アクセシビリティ] > [テキストのサイズ]) に従います。

## 通知

Android の通知は、アプリの名前とアイコンで Windows の通知として表示されます。

- 通知を選ぶとアプリが開きます。
- 通知を閉じると、アプリが表示し続ける場合を除き、Android 側でも閉じられます。
- Android が Doze の間も通知は届き続けます。[メモリ](memory.md) を参照してください。

## バックグラウンドで動くアプリ

音楽プレーヤーのように、ウィンドウを閉じた後も動き続けるアプリは、通知領域にアイコンとして表示されます。
そのツールチップは、アプリの通知の内容です。

- アイコンを選ぶとアプリが開きます。
- 右クリックすると、**Open**、通知自身の操作、**Stop** が表示されます。
- **Stop** は先に確認します。
  **Do not ask again** にチェックを入れると、それ以降は確認しません。
  再び確認させるには、DSA Settings の **Running in the background** (バックグラウンドで実行中) ページで **Ask before stopping an app from the notification area** をオンにします。
- メディアを再生するアプリは、メディア キーと Windows のメディア コントロールでも操作できます。

DSA Settings の **Running in the background** ページには、こうしたアプリが **Open** と **Stop** とともに一覧表示されます。
コマンド ラインでは `dsa background`、`dsa background open <package>`、`dsa background stop <package>` です。

## デスクトップのウィジェット

Android のウィジェットを、専用のウィンドウとしてデスクトップに置けます。
ウィジェットはほかのウィンドウの下にとどまり、デスクトップを表示しても残ります。
タスクバーや Alt+Tab には表示されません。

1. DSA Settings の **Widgets** (ウィジェット) ページを開きます。
2. **Widgets your apps offer** で、ウィジェットの横にある **Add to desktop** を選びます。
   アプリからウィジェットの設定を求められた場合は、設定画面がウィンドウで開きます。
3. ウィジェットを動かすには、ウィジェットのどこかを押したまま 0.5 秒じっとします。
   ウィジェットが持ち上がり、枠と、サイズを変えられる辺ごとのハンドルが表示されます。
   ドラッグすると移動し、ハンドルをドラッグするとサイズが変わります。
   すばやいドラッグはウィジェット自身の操作として扱われるので、リストはこれまでどおりスクロールできます。
   持ち上げずに、ウィジェットの辺をドラッグしてサイズを変えることもできます。これができるのは、サイズを変えられる辺だけです。
   置いた場所と大きさは DSA が覚えています。
4. ウィジェットを右クリックするか、長押しの後に指やボタンを離すと、**Widget settings** (ウィジェットを再設定できる場合)、許可されたサイズの中から選ぶ **Resize**、**Remove** が表示されます。
   アプリケーション キーと Shift+F10 でもこのメニューが開きます。
   このメニューでは矢印キー、Enter、Esc が使え、文字キーで項目を選べます。**Resize** は R、**Remove** は M です。

コマンド ラインでは `dsa widgets list`、`dsa widgets add <provider>`、`dsa widgets remove <id>` です。

## リンクとクリップボード

- Android アプリでタップした Web リンクは、既定のブラウザーで開きます。
  インストール済みのアプリが自分のものとして確認したリンクは、そのアプリで開きます。
- すべての Web リンクを Android アプリで開くには、DSA Settings の **Advanced** (詳細) ページで **Open web links in Android apps** をオンにします (`dsa settings set links-in-android on`)。
- コピーしたテキストは、Windows と Android の間で双方向に共有されます。
  やめるには、**Advanced** ページで **Share the clipboard with Android** をオフにします (`dsa settings set clipboard off`)。

## Google Play と保護された動画

DSA は Google のソフトウェアを同梱していません。
必要な場合は、DSA が入手元と利用条件を示したうえで、入手元からダウンロードします。
どちらも次に Android を起動したときから適用され、どちらも再び削除できます。

- **Google Play。** DSA Settings の **Advanced** ページで **Google Play** の下にある **Install** を選ぶか、`dsa gapps install` を実行します。
  DSA は、使っている Android のバージョン向けの公開 LiteGapps Core パッケージ (Google Play 開発者サービス、Play ストア、アカウント同期) をダウンロードします。
  この Android は Google の認定を受けていないため、デバイスを登録するまで Google がサインインを拒否することがあります。
  **Register this device with Google** は Android ID を表示し、登録用のページを開きます。
  `dsa gapps status` で状態を確認でき、`dsa gapps remove` で Google のアプリを削除できます。Android が停止している間でも削除できます。
- **Widevine。** ストリーミング アプリが保護された動画を再生するには、Google の Widevine モジュールが必要です。
  **Advanced** ページで **Protected content** の下にある **Get** を選ぶか、`dsa drm widevine install` を実行します。
  DSA は、使っている Android のバージョン向けの Google の Android エミュレーター イメージ (約 2 GB) をダウンロードし、Widevine モジュールだけを変更せずに残して、残りを削除します。
  ここでの Widevine はセキュリティ レベル L3 なので、ストリーミング サービスは最大でも標準画質しか送りません。
  高画質で見るには、Windows のブラウザーやアプリを使ってください。
  `dsa drm widevine remove` で削除できます。

## ARM 専用アプリ

x64 PC では、ARM プロセッサ向けのコードしか持たないアプリは、ARM 変換ツールを通して動きます。
x64 PC の Android には、オープンソースの変換ツール Digitalis が含まれており、最初の起動から使われます。
代わりに別の変換ツールを使うこともできます。

- **Google の ARM 変換ツール。** **Advanced** ページで **Google's ARM translator** の横にある **Get** を選ぶか、`dsa arm-translation fetch google` を実行します。
  DSA はまず入手元と Google の利用条件 ([Android SDK License](https://developer.android.com/studio/terms)) を表示し、同意するまで何もダウンロードしません。
  その後、PC は Google が提供する、使っている Android のバージョン向けの Android エミュレーター イメージ (約 2 GB) をダウンロードし、変換ツールだけを残して残りを削除します。
  DSA が Google の変換ツールを同梱したり、配布したりすることはありません。
- **自分でビルドした、またはほかで入手したパッケージ。** たとえば、オープンソースの変換ツール Digitalis から作ったものです: `dsa arm-translation install <folder>`。

その後、Android を再起動します。
DSA Settings の **General** (全般) ページで Android を停止してから、アプリを開きます。

Android に含まれている変換ツールに戻すには、**Advanced** ページで **Use the default translator** の横にある **Use** を選ぶか、`dsa arm-translation use-default` を実行します。
インストールしたパッケージは、次に Android を起動したときから削除されます。

Android が使っている変換ツールは `dsa arm-translation status` で確認できます。
**Advanced** ページの **Run apps built only for ARM** スイッチは、何も削除せずに変換をオンまたはオフにします。
ARM と x64 の両方のコードを持つアプリでは、**Apps** ページの **Run as ARM** (`dsa apps run-as-arm <package> on`) で、ARM のコードを実行させることができます。
動くのは 64 ビット ARM アプリ (`arm64-v8a`) だけです。
ARM64 PC では、Android が ARM アプリをそのまま実行するので、変換ツールは不要です。

## root 権限

バックアップ ツールのように、root 権限が必要なアプリがあります。
DSA は、Linux PC の `sudo` と同じように root 権限を与えます。
PC の管理者が承認したときだけ、1 回の要求ごとに与えます。
既定ではオフで、許可されていないアプリからは `su` がまったく見えません。

1. 管理者が、DSA Settings の **Advanced** ページにある **Root access** の下で **Let Android apps ask for root** をオンにします (`dsa settings set root-elevation on`)。
   次に Android を起動したときから適用されます。
2. **Apps** ページで、アプリの **Root access** を設定します。
   - **Root: not allowed** (既定): アプリは `su` を持ちません。
   - **Root: ask each time**: アプリが `su` を使うたびに、アプリ自身の名前とアイコンで Windows の管理者確認のプロンプト (UAC) が表示されます。
     標準ユーザーは管理者のパスワードを入力します。
     承認は、そのアプリについて 15 分間有効です。
   - **Root: remember (no asking)**: アプリは確認なしで `su` を使います。
     これを選ぶには管理者の承認が必要です。

   コマンド ラインでは `dsa apps root <package> off|ask|remember` です。

開発者モードがオンのときは、`adb shell` でも `su` を使えます。
`adb shell` には **Apps** ページに専用の行 **adb shell** があり、既定では毎回確認します (`dsa apps root adb ask`)。
そのプロンプトには **adb shell** という名前が付き、コマンドが表示されます。

Android 自身の一部が要求した場合は、プロンプトにその名前が表示されます。
**Android system**、**Android phone services**、**Android Bluetooth**、**Android NFC**、**Android system service** のいずれかです。
1 つのユーザー ID を共有するアプリは 1 つのグループとして承認され、プロンプトにはそのすべてが一覧表示されます。

すべての要求は、アプリ、時刻、コマンド、承認されたかどうかとともにサービスのログに残ります。
この機能をオフにすると、再起動なしで、すべてのアプリから一度に `su` が見えなくなります。

---

Android は Google LLC の商標です。
Windows は Microsoft グループ企業の商標です。
DSA は Google および Microsoft とは提携しておらず、両社による承認や後援も受けていません。
