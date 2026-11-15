# 開発者モードと adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · **日本語** · [简体中文](../zh-CN/developer-mode.md) · [繁體中文](../zh-TW/developer-mode.md) · [Español](../es/developer-mode.md) · [Français](../fr/developer-mode.md) · [Deutsch](../de/developer-mode.md) · [Português (Brasil)](../pt-BR/developer-mode.md) · [Русский](../ru/developer-mode.md) · [Italiano](../it/developer-mode.md) · [Tiếng Việt](../vi/developer-mode.md) · [Bahasa Indonesia](../id/developer-mode.md) · [Türkçe](../tr/developer-mode.md)

開発者モードを使うと、PC 上の Android Debug Bridge (adb) を、Desktop Subsystem for Android™ (DSA) の Android に接続できます。
既定ではオフです。

adb は DSA には含まれていません。
Android SDK Platform-Tools から入手してください。

## adb を接続する

1. DSA Settings の **Network** (ネットワーク) ページを開き、**Developer mode** (開発者モード) をオンにします。
   または、`dsa settings set developer-mode on` を実行します。
2. Android が動いていない場合は、起動します: `dsa start --wait`。
3. 接続します。

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   Android が接続の許可を求めることはありません。
   このポートはこの PC 上でしか応答しないので、ほかのコンピューターからは接続できません。
4. 作業が終わったら、**Developer mode** を再びオフにするか、`dsa settings set developer-mode off` を実行します。

**Network** ページの **Connect adb** に正確なコマンドが表示されます。
`dsa settings` でも、`Developer mode` の行にそのコマンドが表示されます。

## 別のポートを使う

既定のポートは 58526 です。
変更するには、**Connect adb** の下の **Port** を編集するか、次のコマンドを実行します。

```
dsa settings set adb-port 5555
```

## 開発者モードで変わらないこと

DSA は、自分の仕事に adb を一切使いません。
ウィンドウ、入力、インストールは DSA 独自の制御チャネルを使うので、開発者モードがオフでもすべて同じように動きます。

Android のログを読むのに adb は必要ありません。
[トラブルシューティング](troubleshooting.md) を参照してください。

---

Android は Google LLC の商標です。
DSA は Google とは提携しておらず、同社による承認や後援も受けていません。
