# 解除安裝

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · [日本語](../ja/uninstall.md) · [简体中文](../zh-CN/uninstall.md) · **繁體中文** · [Español](../es/uninstall.md) · [Français](../fr/uninstall.md) · [Deutsch](../de/uninstall.md) · [Português (Brasil)](../pt-BR/uninstall.md) · [Русский](../ru/uninstall.md) · [Italiano](../it/uninstall.md) · [Tiếng Việt](../vi/uninstall.md) · [Bahasa Indonesia](../id/uninstall.md) · [Türkçe](../tr/uninstall.md)

## Windows

您可以用下列任一方式解除安裝 Desktop Subsystem for Android™（DSA）：

- Windows 的「設定」>「應用程式」>「已安裝的應用程式」：找到 Desktop Subsystem for Android，然後選取「解除安裝」。
- 再次執行安裝程式，然後選取 **Uninstall**（解除安裝）。

解除安裝會為電腦上的每位使用者（無論是否已登入）移除下列項目：

- 程式、`DsaHost` 服務、`dsa` 指令、DSA Settings 與 APK Installer；
- 已下載的 Android 映像與更新；
- Android 應用程式的開始功能表項目，以及它們在「已安裝的應用程式」中的項目；
- 檔案總管中的 **Android**、`.apk` 檔案關聯，以及右鍵功能表項目；
- DSA 的設定與記錄。

它會**保留**每位使用者的 Android 磁碟 `%LOCALAPPDATA%\DSA\android.vhdx`，因此日後重新安裝時，就能再次找到您的應用程式與資料。
若也要移除您的資料，請在解除安裝前執行 `dsa reset-data --yes`，或在解除安裝後刪除該檔案。

如果資料夾重新導向移動了某位使用者的開始功能表，該使用者的 Android 應用程式項目會保留下來，因為解除安裝程式只會在預設位置尋找。
請手動刪除這些項目。

您的 WSL 設定不會受到影響，因為 DSA 從不變更它。

## Linux（尚未發行）

```
sudo apt remove desktop-subsystem-for-android    # Debian、Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

移除套件時，也會一併移除它所加入的套件存放庫項目。

---

Android 是 Google LLC 的商標。
Windows 是 Microsoft 集團旗下公司的商標。
DSA 與 Google 或 Microsoft 沒有任何關聯，也未獲得其背書或贊助。
