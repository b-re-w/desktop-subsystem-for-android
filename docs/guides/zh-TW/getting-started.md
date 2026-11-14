# 開始使用

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · [日本語](../ja/getting-started.md) · [简体中文](../zh-CN/getting-started.md) · **繁體中文** · [Español](../es/getting-started.md) · [Français](../fr/getting-started.md) · [Deutsch](../de/getting-started.md) · [Português (Brasil)](../pt-BR/getting-started.md) · [Русский](../ru/getting-started.md) · [Italiano](../it/getting-started.md) · [Tiếng Việt](../vi/getting-started.md) · [Bahasa Indonesia](../id/getting-started.md) · [Türkçe](../tr/getting-started.md)

Desktop Subsystem for Android™（DSA）在具有自己核心的輕量虛擬機器中執行 Android。
每個 Android 應用程式都會在您的桌面上以自己的視窗開啟。

## 各平台的狀態

| 主機 | 狀態 |
|---|---|
| Windows 11，x64 | 已發行。 |
| Windows 11，ARM64 | 已發行。Android 也是 ARM64，因此 ARM 應用程式不需轉譯即可執行。 |
| Linux，x64 與 ARM64 | 開發中。`.deb` 與 `.rpm` 套件可在 CI 中建置；尚未發行。 |
| macOS，Apple 晶片 | 開發中。`DSA.app`、其 `.pkg` 與 `.dmg` 可在 CI 中建置；尚未發行。 |

## 在 Windows 上安裝

您需要一台搭載 Windows 11 的 x64 或 ARM64 電腦，並開啟 Windows 功能「虛擬機器平台」。

1. 開啟「虛擬機器平台」。
   開啟「開啟或關閉 Windows 功能」，勾選「虛擬機器平台」，選取「確定」，然後重新啟動。
   或在系統管理員終端機中執行 `dism /online /enable-feature /featurename:VirtualMachinePlatform /all`。
   安裝程式不會替您開啟此功能：若功能未開啟，它會顯示訊息並停止。
2. 從[最新版本](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest)下載安裝程式與 `SHA256SUMS.txt`：
   x64 電腦請下載 `DSA-v<version>-win-x64-setup.exe`，ARM64 電腦請下載 `DSA-v<version>-win-arm64-setup.exe`。
3. 檢查下載的檔案。
   此指令輸出的雜湊值必須與 `SHA256SUMS.txt` 中的那一行相符：

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. 執行安裝程式，選取 **Install**（安裝），並核准系統管理員提示。
5. 等待第一個 Android 映像。
   安裝程式未內含映像時，DSA 服務會在背景下載。
   `dsa status` 會顯示下載進度。
6. 從開始功能表開啟應用程式，或從終端機檢查：

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## 安裝程式會安裝的項目

- `DsaHost` 服務，負責執行每位使用者的 Android。
- `dsa` 指令。
- 開始功能表中的 **DSA Settings**。
- 檔案總管瀏覽窗格中的 **Android**，可開啟 Android 的檔案。
- APK Installer，用來開啟 `.apk` 檔案。
- DSA 視窗管理員（`dsa-wm`）。
  它會在您登入時啟動，並同時啟動 Android，因此您開啟的第一個應用程式不必等待 Android 開機。

每位 Windows 使用者都有自己的 Android 與自己的資料磁碟 `%LOCALAPPDATA%\DSA\android.vhdx`。
其他使用者無法讀取它。

DSA 使用自己的核心與自己的虛擬機器。
它不會使用或變更您的 WSL 設定、核心或發行版。

## 更新

DSA 會更新自身與 Android 映像。
兩者都經過簽署，簽章、大小或雜湊值不符的內容，DSA 一律不會套用。

- **DSA 本身**會在沒有任何使用者的 Android 正在執行時更新。
  在那之前更新會等待；`dsa update` 會立即安裝。
  DSA 絕不會降到較低的版本。
- **發行通道。** 從 1.0 起，DSA 有穩定通道，以及供仍在完成中的版本使用的預先發行通道。
  您的通道取決於您使用的安裝程式，升級時會保留。
  系統管理員可在 DSA Settings 的 **Updates**（更新）頁面上以 **Get pre-release versions**（取得預先發行版本）變更。
  1.0 之前的每個版本都是預先發行版。
- **Android 映像**與通道無關。
  如果映像無法開機，DSA 會回到上一個映像，`dsa status` 也會說明這一點。

系統管理員可以在 DSA Settings 的 **Updates** 頁面上關閉更新。
所有開關請參閱[設定](settings.md)。

## Linux（開發中，尚未發行）

Linux 版 DSA 建置為 `.deb`（Debian、Ubuntu）與 `.rpm`（Fedora、openSUSE）。
它需要 KVM（`/dev/kvm`）與 Wayland 桌面。
Android 在 crosvm 虛擬機器中執行。

安裝套件時也會加入 DSA 已簽署的套件存放庫，因此之後的版本會透過 `apt`、`dnf`、`zypper` 或軟體中心送達。
在 Linux 上，DSA 沒有自己的更新程式。

- Debian 與 Ubuntu：`/etc/apt/sources.list.d/dsa.sources`，金鑰位於 `/usr/share/keyrings/dsa-archive-keyring.gpg`。
- Fedora 與 openSUSE：`/etc/yum.repos.d/dsa.repo`（有 zypper 時還有 `/etc/zypp/repos.d/dsa.repo`），金鑰位於 `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa`。

存放庫預計從 `https://thisisthepy.github.io/desktop-subsystem-for-android/` 提供。
目前尚未發布。

發行後，第一次安裝的方式如下：

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian、Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

套件會安裝 `dsa-hostd` 系統服務、`dsa-devices` 與 `dsa-compositor` 使用者服務、位於 `/usr/bin` 的 `dsa` 指令、**DSA Files** 功能表項目，以及 **Open in an Android app**、**Share with an Android app** 與 **Install in Android** 這幾個檔案動作。

## macOS（開發中，尚未發行）

macOS 版 DSA 在 Apple 晶片上以 Apple 的 Virtualization 架構執行 Android。
它建置為 `DSA.app`，並包裝成安裝程式套件（`DSA-<version>.pkg`）與磁碟映像（`DSA-<version>.dmg`）。
CI 建置採用臨時（ad hoc）簽署，因此只能在建置它的電腦上執行。
目前還沒有以 Developer ID 簽署的版本。

## 下一步

- [應用程式與視窗](apps.md)
- [安裝 APK](install-apks.md)

---

Android 是 Google LLC 的商標。
Windows 是 Microsoft 集團旗下公司的商標。
macOS 是 Apple Inc. 的商標。
DSA 與 Google、Microsoft 或 Apple 沒有任何關聯，也未獲得其背書或贊助。
