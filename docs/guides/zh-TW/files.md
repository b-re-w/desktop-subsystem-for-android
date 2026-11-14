# 檔案

[English](../files.md) · [한국어](../ko/files.md) · [日本語](../ja/files.md) · [简体中文](../zh-CN/files.md) · **繁體中文** · [Español](../es/files.md) · [Français](../fr/files.md) · [Deutsch](../de/files.md) · [Português (Brasil)](../pt-BR/files.md) · [Русский](../ru/files.md) · [Italiano](../it/files.md) · [Tiếng Việt](../vi/files.md) · [Bahasa Indonesia](../id/files.md) · [Türkçe](../tr/files.md)

使用 Desktop Subsystem for Android™（DSA），您可以在檔案總管中開啟 Android 的檔案、將您的 Windows 資料夾分享給 Android 應用程式，以及將檔案傳送給 Android 應用程式。
本頁說明 Windows 上的 DSA。

## 在檔案總管中開啟 Android 的檔案

可以使用下列任一方式。
Android 未執行時，每種方式都會啟動 Android。

- 檔案總管瀏覽窗格中的 **Android**，就在 WSL 的 **Linux** 正下方。
- DSA Settings 的 **Advanced**（進階）頁面上的 **Android files**（Android 檔案）。
- `dsa files open`。
  `dsa files` 會說明儲存空間的位置，但不會開啟它。

您看到的是 Android 的共用儲存空間：Android 應用程式所儲存的檔案，例如相片與下載的檔案。
您可以像操作任何資料夾一樣將檔案複製進出。
檔案總管透過 Windows 的 WebDAV 用戶端存取此儲存空間。
如果儲存空間沒有回應，DSA 會顯示錯誤並說明原因。

## 將您的 Windows 資料夾分享給 Android 應用程式

您的「文件」、「下載」、「圖片」、「音樂」、「影片」與「桌面」資料夾，會顯示在 Android 共用儲存空間的 `Host` 下，例如 `Host/Pictures`。
Android 應用程式可以在那裡開啟與儲存檔案。
應用程式必須先取得 Android 的儲存空間權限，而 Windows 只允許它做您自己的帳戶能做的事。

分享預設為開啟。
若要變更，請開啟 DSA Settings 的 **Advanced** 頁面：

- 關閉 **Share Windows folders with Android**（與 Android 分享 Windows 資料夾），就不會分享任何資料夾。
- 在 **Shared folders**（共用資料夾）下，取消勾選某個資料夾即可停止分享它。
  它會立即從 Android 中消失。

從命令列：

```
dsa settings set shared-folders Documents,Pictures   # 只分享這些；也可以是 all 或 none
dsa settings set file-sharing off                     # 不分享任何資料夾
```

較早的版本使用名為 `Windows` 的資料夾，而不是 `Host`。
舊資料夾清空後，Android 會將它移除，因此您存在那裡的檔案都會保留。

## 將檔案傳送給 Android 應用程式

- **拖放。** 將檔案從檔案總管拖曳到應用程式的視窗上。
  檔案會傳給該應用程式。
  APK 則會改為傳給 APK Installer。
  資料夾不會被傳送。
- **右鍵功能表。** 以滑鼠右鍵按一下檔案，然後選取 **Open in an Android app**（在 Android 應用程式中開啟）或 **Share with an Android app**（分享給 Android 應用程式）。
  在 Windows 11 上，這些項目位於「顯示其他選項」下。
  接著 Android 會詢問要由哪個應用程式接收檔案。
- **命令列。** `dsa open <file>` 與 `dsa share <file>`。

這些方式可傳送最大 64 MiB 的檔案。
較大的檔案會在傳送任何內容之前就被拒絕。
若要將較大的檔案交給應用程式，請改為將它放在共用資料夾中。

## 從 Android 取得檔案

- 在 Android 中將檔案儲存在 `Host/<folder>` 下。
  它就會出現在您電腦上的該資料夾中。
- 或在檔案總管中開啟 **Android**，將檔案複製出來。

---

Android 是 Google LLC 的商標。
Windows 是 Microsoft 集團旗下公司的商標。
DSA 與 Google 或 Microsoft 沒有任何關聯，也未獲得其背書或贊助。
