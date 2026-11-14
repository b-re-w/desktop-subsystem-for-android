# 安裝 APK

[English](../install-apks.md) · [한국어](../ko/install-apks.md) · [日本語](../ja/install-apks.md) · [简体中文](../zh-CN/install-apks.md) · **繁體中文** · [Español](../es/install-apks.md) · [Français](../fr/install-apks.md) · [Deutsch](../de/install-apks.md) · [Português (Brasil)](../pt-BR/install-apks.md) · [Русский](../ru/install-apks.md) · [Italiano](../it/install-apks.md) · [Tiếng Việt](../vi/install-apks.md) · [Bahasa Indonesia](../id/install-apks.md) · [Türkçe](../tr/install-apks.md)

Desktop Subsystem for Android™（DSA）不隨附應用程式商店。
您可以使用 APK Installer 或 `dsa` 指令，從 APK 檔案安裝應用程式。
本頁說明 Windows 上的 DSA。

## 使用 APK Installer 安裝

1. 在檔案總管中按兩下 `.apk` 檔案。
   它會在 **APK Installer**（APK 安裝程式）中開啟。
2. 確認 APK 的內容。
   視窗會在不啟動 Android 的情況下顯示：
   - 應用程式的名稱、圖示、套件與版本；
   - 發行者，取自簽署憑證，因為 APK 本身不會註明發行者；
   - 來源：下載檔案的主機，或檔案所在的資料夾；
   - 在 **Capabilities**（功能）下，它要求的前幾項權限。
     **More**（更多）會顯示所有權限、它適用的 Android 版本、大小、處理器類型，以及其憑證的 SHA-256。

   當 APK 只有其他處理器類型的程式碼時，視窗會警告您。
3. 選取 **Install**（安裝）。
   應用程式已經安裝時，按鈕會顯示為 **Update**（更新）或 **Install again**（重新安裝）。
   Android 未執行時會先啟動 Android。
4. 勾選 **Launch when ready**（就緒時啟動，預設值）時，應用程式安裝完成後就會開啟，安裝程式也會關閉。
   否則請選取 **Launch**（啟動），或稍後再從開始功能表開啟應用程式。

Android 執行期間，視窗也會說明該檔案是新安裝、更新、相同版本的重新安裝，還是 Android 會拒絕的項目：較舊的版本，或以其他金鑰簽署的同一個應用程式。

您也可以將 APK 檔案拖放到 APK Installer、DSA Settings 或應用程式的視窗上。
每個 APK 都會在自己的安裝程式視窗中開啟。

## 從命令列安裝

```
dsa apk-info app.apk            # APK 的內容；不啟動 Android 即可讀取檔案
dsa install app.apk             # 安裝；Android 未執行時會先啟動
dsa install app.apk --progress  # 同時輸出每個步驟
```

Android 拒絕 APK 時，`dsa install` 會以代碼 1 結束，其訊息會以 `not installed (<reason>):` 開頭。

## 安裝遭拒的原因

| 原因 | 意義 | 處理方式 |
|---|---|---|
| `incompatible` | 應用程式需要其他處理器類型、較新的 Android 版本，或此裝置缺少的功能 | 請參閱[僅為 ARM 建置的應用程式](apps.md#僅為-arm-建置的應用程式)或 [Android 版本](android-versions.md) |
| `conflicting-signature` | 應用程式已經安裝，但以其他金鑰簽署 | 先解除安裝已安裝的應用程式 |
| `downgrade` | 已安裝較新版本的應用程式 | 先解除安裝已安裝的應用程式 |
| `invalid-apk` | 該檔案不是 Android 能讀取的已簽署 APK | 重新取得 APK |
| `insufficient-storage` | Android 沒有空間安裝該應用程式 | 釋放 Android 中的空間 |
| `blocked` | 此 Android 的原則不允許該應用程式 | |
| `other` | 其他任何情況；後面會接著 Android 本身的訊息 | |

APK 最大可達 2 GiB。
安裝一次只會執行一個：第二個會等待第一個完成。

## 解除安裝應用程式

可以使用下列任一方式：

- Windows 的「設定」>「應用程式」>「已安裝的應用程式」。
- DSA Settings 的 **Apps**（應用程式）頁面：應用程式旁的 **Uninstall**（解除安裝）。
- `dsa apps uninstall <package>`。

屬於 Android 一部分的應用程式無法解除安裝。

---

Android 是 Google LLC 的商標。
Windows 是 Microsoft 集團旗下公司的商標。
DSA 與 Google 或 Microsoft 沒有任何關聯，也未獲得其背書或贊助。
