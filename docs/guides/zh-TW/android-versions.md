# Android 版本

[English](../android-versions.md) · [한국어](../ko/android-versions.md) · [日本語](../ja/android-versions.md) · [简体中文](../zh-CN/android-versions.md) · **繁體中文** · [Español](../es/android-versions.md) · [Français](../fr/android-versions.md) · [Deutsch](../de/android-versions.md) · [Português (Brasil)](../pt-BR/android-versions.md) · [Русский](../ru/android-versions.md) · [Italiano](../it/android-versions.md) · [Tiếng Việt](../vi/android-versions.md) · [Bahasa Indonesia](../id/android-versions.md) · [Türkçe](../tr/android-versions.md)

Desktop Subsystem for Android™（DSA）支援最新的兩個 Android 主要版本：目前是 Android 16 與 Android 17。
預設會執行最新的版本。
有些應用程式只能在其中一個版本上執行，因此您可以自行選擇。

## 查看執行中的版本

開啟 DSA Settings 的 **Updates**（更新）頁面。
在 **Android** 下會顯示版本、其組建、其安全性修補程式等級、它所跟隨的版本，以及其更新目前的狀態與下載進度。
**Check now**（立即檢查）會要求服務立即尋找更新。
或執行：

```
dsa android version   # 執行中的版本，以及可選擇的版本
```

## 變更版本

電腦上的每位使用者都執行相同的 Android 版本，因此只有系統管理員可以變更。

1. 開啟 DSA Settings 的 **Updates** 頁面。
2. 在 **Version to use**（要使用的版本）下，選擇一個版本或 **Always the newest**（一律使用最新版本）。
3. 選取 **Change**（變更），並核准系統管理員提示。

選擇的版本會被下載，並從 Android 下次啟動時開始使用。
從系統管理員終端機：

```
dsa android use 16
dsa android use latest
```

### 改用較舊的版本會清除資料

Android 無法讀取較新版本寫入的資料。
因此，改用較舊的主要版本會清除電腦上每位使用者的 Android 資料：他們安裝的應用程式、帳戶與所有應用程式資料。
DSA 的設定會保留。
此操作無法復原。

DSA Settings 會要求您以 **Erase data and change**（清除資料並變更）確認。
在命令列中，請加上 `--erase-data`：

```
dsa android use 16 --erase-data
```

改用較新的版本會保留您的應用程式與資料。

## 更新

- 選擇的版本會持續收到安全性更新。
- 當某個版本結束支援時，DSA 會改用下一個版本，並保留您的應用程式。
- DSA 絕不會自行改用較舊的主要版本。
- 每個映像在使用前都會以一份已簽署的清單核對。
  無法開機的映像會由上一個映像取代。

系統管理員可以在 DSA Settings 的 **Updates** 頁面上使用 **Update the Android image automatically**（自動更新 Android 映像），或執行 `dsa settings set image-updates off`，來關閉映像更新。

---

Android 是 Google LLC 的商標。
Windows 是 Microsoft 集團旗下公司的商標。
DSA 與 Google 或 Microsoft 沒有任何關聯，也未獲得其背書或贊助。
