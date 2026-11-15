# はじめに

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · **日本語** · [简体中文](../zh-CN/getting-started.md) · [繁體中文](../zh-TW/getting-started.md) · [Español](../es/getting-started.md) · [Français](../fr/getting-started.md) · [Deutsch](../de/getting-started.md) · [Português (Brasil)](../pt-BR/getting-started.md) · [Русский](../ru/getting-started.md) · [Italiano](../it/getting-started.md) · [Tiếng Việt](../vi/getting-started.md) · [Bahasa Indonesia](../id/getting-started.md) · [Türkçe](../tr/getting-started.md)

Desktop Subsystem for Android™ (DSA) は、独自のカーネルを持つ軽量の仮想マシンで Android を実行します。
Android アプリはそれぞれ、デスクトップ上の専用のウィンドウで開きます。

## プラットフォームごとの状況

| ホスト | 状況 |
|---|---|
| Windows 11、x64 | リリース済み。 |
| Windows 11、ARM64 | リリース済み。Android も ARM64 なので、ARM アプリは変換なしで動きます。 |
| Linux、x64 と ARM64 | 開発中。`.deb` と `.rpm` のパッケージは CI でビルドされますが、まだリリースされていません。 |
| macOS、Apple シリコン | 開発中。`DSA.app` とその `.pkg`、`.dmg` は CI でビルドされますが、まだリリースされていません。 |

## Windows へのインストール

x64 または ARM64 PC 上の Windows 11 が必要で、Windows の機能 **仮想マシン プラットフォーム** (Virtual Machine Platform) がオンになっている必要があります。

1. 仮想マシン プラットフォームをオンにします。
   **Windows の機能の有効化または無効化** を開き、**仮想マシン プラットフォーム** にチェックを入れて **OK** を選び、再起動します。
   または、管理者のターミナルで `dism /online /enable-feature /featurename:VirtualMachinePlatform /all` を実行します。
   セットアップはこの機能をオンにしません。
   機能がオフの場合は、メッセージを表示して止まります。
2. [最新のリリース](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest) から、セットアップ プログラムと `SHA256SUMS.txt` をダウンロードします。
   x64 PC では `DSA-v<version>-win-x64-setup.exe`、ARM64 PC では `DSA-v<version>-win-arm64-setup.exe` です。
3. ダウンロードしたファイルを確認します。
   このコマンドが表示するハッシュが、`SHA256SUMS.txt` の行と一致する必要があります。

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. セットアップを実行し、**Install** を選んで、管理者確認のプロンプトを承認します。
5. 最初の Android イメージを待ちます。
   セットアップにイメージが含まれていない場合は、DSA のサービスがバックグラウンドでダウンロードします。
   ダウンロードの状況は `dsa status` で確認できます。
6. スタート メニューからアプリを開くか、ターミナルで確認します。

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## セットアップがインストールするもの

- 各ユーザーの Android を実行する `DsaHost` サービス。
- `dsa` コマンド。
- スタート メニューの **DSA Settings**。
- エクスプローラーのナビゲーション ウィンドウの **Android**。Android のファイルを開きます。
- `.apk` ファイルを開く APK Installer。
- DSA Window Manager (`dsa-wm`)。
  サインイン時に起動し、そのときに Android も起動するので、最初に開くアプリが Android の起動を待つことはありません。

Windows のユーザーごとに専用の Android と専用のデータ ディスク `%LOCALAPPDATA%\DSA\android.vhdx` があります。
ほかのユーザーはこれを読めません。

DSA は独自のカーネルと独自の仮想マシンを使います。
WSL の設定、カーネル、ディストリビューションを使ったり変更したりすることはありません。

## アップデート

DSA は自分自身と Android イメージを更新します。
どちらも署名されており、署名、サイズ、ハッシュのいずれかが一致しないものを DSA は適用しません。

- **DSA 本体** は、どのユーザーの Android も動いていないときに更新されます。
  それまで更新は待機しますが、`dsa update` ですぐにインストールできます。
  DSA が下のバージョンに移ることはありません。
- **リリース チャネル。** 1.0 以降、DSA には安定版チャネルと、仕上げ中のバージョン向けのプレリリース チャネルがあります。
  チャネルは使ったインストーラーで決まり、アップグレードしても変わりません。
  管理者は DSA Settings の **Updates** ページにある **Get pre-release versions** (プレリリース版を受け取る) で変更できます。
  1.0 より前のリリースはすべてプレリリースです。
- **Android イメージ** はチャネルに左右されません。
  イメージの起動に失敗した場合、DSA は前のイメージに戻し、`dsa status` にその旨が表示されます。

管理者は DSA Settings の **Updates** ページで更新をオフにできます。
すべてのスイッチについては [設定](settings.md) を参照してください。

## Linux (開発中、未リリース)

Linux 版の DSA は、`.deb` (Debian、Ubuntu) と `.rpm` (Fedora、openSUSE) としてビルドされます。
KVM (`/dev/kvm`) と Wayland のデスクトップが必要です。
Android は crosvm の仮想マシンで動きます。

パッケージをインストールすると、DSA の署名付きパッケージ リポジトリも追加されるので、以降のバージョンは `apt`、`dnf`、`zypper`、またはソフトウェア センターから届きます。
Linux では、DSA は独自の更新機能を持ちません。

- Debian と Ubuntu: `/etc/apt/sources.list.d/dsa.sources`。キーは `/usr/share/keyrings/dsa-archive-keyring.gpg` にあります。
- Fedora と openSUSE: `/etc/yum.repos.d/dsa.repo` (zypper がある場合は `/etc/zypp/repos.d/dsa.repo` も)。キーは `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa` にあります。

リポジトリは `https://thisisthepy.github.io/desktop-subsystem-for-android/` から提供される予定です。
まだ公開されていません。

リリース後の最初のインストールは、次のようになります。

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian, Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

パッケージは、`dsa-hostd` システム サービス、`dsa-devices` と `dsa-compositor` のユーザー サービス、`/usr/bin` の `dsa` コマンド、メニュー項目 **DSA Files**、ファイルの操作 **Open in an Android app**、**Share with an Android app**、**Install in Android** をインストールします。

## macOS (開発中、未リリース)

macOS 版の DSA は、Apple シリコン上で Apple の Virtualization フレームワークを使って Android を実行します。
`DSA.app` として、インストーラー パッケージ (`DSA-<version>.pkg`) とディスク イメージ (`DSA-<version>.dmg`) でビルドされます。
CI のビルドはアドホック署名なので、ビルドしたマシンでしか動きません。
Developer ID で署名されたリリースはまだありません。

## 次へ

- [アプリとウィンドウ](apps.md)
- [APK のインストール](install-apks.md)

---

Android は Google LLC の商標です。
Windows は Microsoft グループ企業の商標です。
macOS は Apple Inc. の商標です。
DSA は Google、Microsoft、Apple とは提携しておらず、これらの企業による承認や後援も受けていません。
