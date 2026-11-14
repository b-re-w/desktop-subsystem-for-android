# 開發人員模式與 adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · [日本語](../ja/developer-mode.md) · [简体中文](../zh-CN/developer-mode.md) · **繁體中文** · [Español](../es/developer-mode.md) · [Français](../fr/developer-mode.md) · [Deutsch](../de/developer-mode.md) · [Português (Brasil)](../pt-BR/developer-mode.md) · [Русский](../ru/developer-mode.md) · [Italiano](../it/developer-mode.md) · [Tiếng Việt](../vi/developer-mode.md) · [Bahasa Indonesia](../id/developer-mode.md) · [Türkçe](../tr/developer-mode.md)

開發人員模式讓您電腦上的 Android Debug Bridge（adb）能連線到 Desktop Subsystem for Android™（DSA）中的 Android。
此模式預設為關閉。

adb 不屬於 DSA。
請從 Android SDK Platform-Tools 取得。

## 連線 adb

1. 開啟 DSA Settings 的 **Network**（網路）頁面，然後開啟 **Developer mode**（開發人員模式）。
   或執行 `dsa settings set developer-mode on`。
2. 如果 Android 未執行，請啟動它：`dsa start --wait`。
3. 連線：

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   Android 不會要求您允許此連線。
   此連接埠只在這台電腦上回應，因此其他電腦無法連線。
4. 使用完畢後，請再次關閉 **Developer mode**，或執行 `dsa settings set developer-mode off`。

**Network** 頁面上的 **Connect adb**（連線 adb）會顯示確切的指令，`dsa settings` 也會在 `Developer mode` 那一行輸出它。

## 使用其他連接埠

預設連接埠為 58526。
若要變更，請編輯 **Connect adb** 下的 **Port**（連接埠），或執行：

```
dsa settings set adb-port 5555
```

## 開發人員模式不會改變的部分

DSA 本身的運作絕不使用 adb。
視窗、輸入與安裝都使用 DSA 自己的控制通道，因此即使關閉開發人員模式，一切運作也都相同。

閱讀 Android 的記錄不需要 adb。
請參閱[疑難排解](troubleshooting.md)。

---

Android 是 Google LLC 的商標。
DSA 與 Google 沒有任何關聯，也未獲得其背書或贊助。
