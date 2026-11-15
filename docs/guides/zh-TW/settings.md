# 設定

[English](../settings.md) · [한국어](../ko/settings.md) · [日本語](../ja/settings.md) · [简体中文](../zh-CN/settings.md) · **繁體中文** · [Español](../es/settings.md) · [Français](../fr/settings.md) · [Deutsch](../de/settings.md) · [Português (Brasil)](../pt-BR/settings.md) · [Русский](../ru/settings.md) · [Italiano](../it/settings.md) · [Tiếng Việt](../vi/settings.md) · [Bahasa Indonesia](../id/settings.md) · [Türkçe](../tr/settings.md)

**DSA Settings** 是 Windows 上 Desktop Subsystem for Android™（DSA）的設定應用程式。
請從開始功能表開啟它。
它會依您的 Windows 顯示語言以英文或韓文顯示，並遵循您的 Windows 佈景主題。
它可搭配鍵盤與螢幕閱讀程式使用。

DSA Settings 所做的每項變更都會經由 `dsa` 指令進行，因此命令列看到的值完全相同：

```
dsa settings                        # 每項設定及其值
dsa settings set <name> <value>     # 變更一項；開關使用 on 或 off
```

有些設定會套用到電腦上的每位使用者，因此只有系統管理員可以變更。
DSA Settings 會以盾牌圖示標示這些設定，並在您變更其中一項時要求系統管理員核准。
下文中會以「（系統管理員）」標示。

## 頁面

| 頁面 | 內容 | 命令列 |
|---|---|---|
| **General**（一般） | Android 的狀態、**Start**、**Stop**、**Freeze Android now**、最近一次的問題、**Open my log folder**、**Refresh** | `dsa status`、`dsa start`、`dsa stop`、`dsa freeze` |
| **Memory**（記憶體） | **Memory limit**、Android VM 的大小、各部分的用量 | `memory-limit`、`dsa memory` |
| **Apps**（應用程式） | 已安裝的應用程式：**Open**、**App info**、**Uninstall**、**Create desktop shortcut**。每個應用程式：**Resizable**、視窗開啟時的大小、**Scaling**、**Full screen request**、**When only the bars hide**、**Run as ARM**、**Root access**。另有一列用於 **adb shell** 的 root 存取權 | `dsa apps`、`dsa app-window`、`dsa apps run-as-arm`、`dsa apps root` |
| **Running in the background**（在背景執行） | 沒有視窗但仍在執行的應用程式：**Open**、**Stop**；停止前是否詢問 | `dsa background`、`stop-without-asking` |
| **Widgets**（小工具） | 您桌面上的小工具，以及 **Add to desktop** | `dsa widgets` |
| **Display & GPU**（顯示與 GPU） | **Render GPU**、**GPU of the primary monitor**、**App full screen**、**Apps that hide their bars**、**Turn off Windows touch gestures in DSA app windows**（僅限 Windows 11，預設開啟）、**Restart the window manager** | `render-gpu`、`dsa gpus`、`app-fullscreen`、`app-immersive`、`touch-gestures-off` |
| **Device identity**（裝置識別） | 最上方是這台電腦，唯讀：其名稱、製造商與型號，以及含 **Copy** 的 **Device specifications**。接著是 Android 回報給應用程式的製造商、型號與組建指紋 | `dsa settings device`、`device.<field>` |
| **Network**（網路） | Android 連上網路的方式、其位址、由誰回應其名稱查詢（DNS）與其閘道、**Advanced networking**、**Use this PC's added root certificates**、**Developer mode**（adb）、**Connect adb automatically** | `dsa network`、`advanced-networking`、`host-certificates`、`developer-mode`、`adb-port`、`adb-auto-connect` |
| **Updates**（更新） | **Android**：執行中的版本與組建、其安全性修補程式等級、它所跟隨的版本、Android 更新及其進度與 **Check now**（系統管理員）、**Restart Android into updates by itself**、**Update the Android image automatically**（系統管理員）、**Version to use**（系統管理員）。**DSA**：**Update DSA automatically**（系統管理員）、**Get pre-release versions**（系統管理員）、**Install the newest version of DSA now** | `dsa android version`、`dsa android use`、`image-auto-restart`、`image-updates`、`program-updates`、`prerelease-updates`、`dsa update` |
| **Advanced**（進階） | 剪貼簿、檔案、網頁連結、閒置時凍結、備份、重設、ARM 應用程式、Google Play、受保護的內容、root 存取權、追蹤（系統管理員）、記錄資料夾 | 請參閱下文 |
| **Logs**（記錄） | Android 的即時記錄、**Save diagnostics** | `dsa logcat`、`dsa logs collect` |
| **About**（關於） | 版本、授權、原始碼 | |

## Advanced 頁面

| 設定 | 預設值 | 命令列 |
|---|---|---|
| **Share the clipboard with Android** | 開啟 | `clipboard` |
| **Share Windows folders with Android** | 開啟 | `file-sharing` |
| **Shared folders** | 全部六個 | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`、`dsa share` |
| **Freeze Android when idle** | 關閉 | `freeze` |
| **Open web links in Android apps** | 關閉 | `links-in-android` |
| **Back up apps' data to this PC**、**Back up now**、**Restore** | 關閉 | `dsa backup on`、`dsa backup now`、`dsa backup restore` |
| **Recovery key**、**Use a recovery key** | | `dsa backup key show`、`dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator**（ARM64 上沒有） | | `dsa arm-translation fetch google` |
| **Use the default translator**（ARM64 上沒有） | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**、**Remove Google Play** | 未安裝 | `dsa gapps install`、`dsa gapps remove` |
| **Widevine**（位於 **Protected content** 下） | 未安裝 | `dsa drm widevine install`、`dsa drm widevine remove` |
| **Let Android apps ask for root**（系統管理員） | 關閉 | `root-elevation` |
| **Input trace**（系統管理員） | 關閉 | `input-trace` |
| **Resize trace**（系統管理員） | 關閉 | `resize-trace` |

備份會在您的使用者資料夾中保存應用程式資料的加密副本，並使用 Windows 替您保管的金鑰。
若要在其他電腦上，或在重新安裝 Windows 之後還原，您需要復原金鑰，因此請將它記下來，並存放在電腦以外的地方。
Google Play、Widevine 與 root 存取權的說明請參閱[應用程式與視窗](apps.md)。

## 變更何時生效

- **Render GPU**：在視窗管理員重新啟動時，也就是您下次登入時，或您選取 **Restart the window manager** 時。
- **Memory limit**、**Device identity**、**Advanced networking**、**Run apps built only for ARM**、Google Play、Widevine、開啟 root 存取權，以及 Android 版本：在 Android 下次啟動時。
  若要重新啟動 Android，請在 **General** 頁面上將它停止，然後開啟一個應用程式。
- 應用程式的視窗設定：從該應用程式開啟的下一個視窗起。
- 其他所有設定，包括關閉 root 存取權：立即生效。

## Android 如何查詢名稱

Android 應用程式查詢的每個名稱都由您的電腦回應，使用的是 Windows 應用程式所用的同一個解析程式（在 Linux 與 macOS 上則是系統的解析程式）。
因此，您的 hosts 檔案、公司的名稱規則、VPN 自己的 DNS 伺服器與 DNS 篩選器，對 Android 應用程式的作用就和對電腦一樣。
Android 本身不會向 DNS 伺服器查詢。

- 開啟 **Advanced networking** 時也是如此：您的電腦位於同一個網路並知道其中的名稱，電腦上的 VPN 或 DNS 篩選器也仍然有效。
- Android 自己的 **Private DNS**（私人 DNS）預設為關閉，因為它會繞過您的電腦去查詢 DNS 伺服器。
  如果您在 Android 的設定中選擇了私人 DNS 伺服器，Android 就會使用它。
- 如果 DSA 暫時無法回應，Android 會改向網路的 DNS 伺服器查詢，直到 DSA 能再次回應為止。
- **Network** 頁面的 **DNS** 列與 `dsa network` 會說明目前由誰回應：您電腦的解析程式，或網路的 DNS 伺服器（並列出這些伺服器）。

## 命令列接受的值

- `render-gpu`：`display`（主要螢幕的 GPU，預設值）、`integrated`、`discrete`，或 `dsa gpus` 所列 GPU 名稱的一部分。
- `memory-limit`：`auto`（預設值：沒有固定上限，最多可用到電腦的記憶體減去 Windows 保留的部分），或 `4g`、`3072`（MiB）等大小。
- `shared-folders`：`all`、`none`，或以逗號分隔的名稱，例如 `Documents,Pictures`。
- `adb-port`：1 到 65535 之間的 TCP 連接埠。
  預設值為 58526。
- `app-fullscreen`：`screen`（預設值）或 `window`，用於應用程式明確的全螢幕要求。
  `app-immersive`：`screen` 或 `window`（預設值），用於只隱藏各列的應用程式。
  請參閱[全螢幕](apps.md#全螢幕)。
- `touch-gestures-off`：`on`（預設值）會在 DSA 應用程式視窗位於前景時關閉 Windows 11 的三指與四指觸控手勢，讓應用程式最多能接收十指，並在其他視窗來到前景時還原您自己的設定。
  如果 Windows 拒絕，就無法使用此設定，設定應用程式會說明原因；您仍可在 Windows 的「設定」中，於「藍牙與裝置」>「觸控」下為所有應用程式關閉這些手勢。
- `host-certificates`：`on`（預設值）讓 Android 應用程式信任加入這台電腦的根憑證，例如公司或安全性程式的憑證。
- `adb-auto-connect`：`on`（預設值）會在開發人員模式開啟時，讓 Android 出現在 `adb devices` 與 Android Studio 中，不需執行 `adb connect`。
- `image-auto-restart`：`on`（預設值）會在沒有任何應用程式視窗開啟時，立即重新啟動 Android 以套用已下載的映像更新。
  否則 DSA 會先詢問，或等到您關閉這些視窗。
- `prerelease-updates`：`on` 會同時取得預先發行版與穩定版，`off` 則只取得穩定版。
  其初始值取決於您使用的安裝程式：預先發行版的安裝程式會將它開啟，穩定版的安裝程式則保持關閉，而升級時會保留您的選擇。
  1.0 之前的每個版本都是預先發行版。
  將它關閉絕不會讓 DSA 退回較舊的版本；DSA 會等待比已安裝版本更新的穩定版。
- `root-elevation`：`on` 讓您在 **Apps** 頁面上允許的應用程式可以要求 root。
  每個應用程式的等級以 `dsa apps root <package> off|ask|remember` 設定。
- `device.<field>`：`manufacturer`、`brand`、`model`、`device`、`name`、`characteristics`、`fingerprint`，或指紋的一部分，例如 `fingerprint.release`。
  `dsa settings reset device` 會將所有值還原。

---

Android 是 Google LLC 的商標。
Windows 是 Microsoft 集團旗下公司的商標。
DSA 與 Google 或 Microsoft 沒有任何關聯，也未獲得其背書或贊助。
