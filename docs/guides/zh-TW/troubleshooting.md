# 疑難排解與記錄

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · [日本語](../ja/troubleshooting.md) · [简体中文](../zh-CN/troubleshooting.md) · **繁體中文** · [Español](../es/troubleshooting.md) · [Français](../fr/troubleshooting.md) · [Deutsch](../de/troubleshooting.md) · [Português (Brasil)](../pt-BR/troubleshooting.md) · [Русский](../ru/troubleshooting.md) · [Italiano](../it/troubleshooting.md) · [Tiếng Việt](../vi/troubleshooting.md) · [Bahasa Indonesia](../id/troubleshooting.md) · [Türkçe](../tr/troubleshooting.md)

本頁協助您找出 Desktop Subsystem for Android™（DSA）無法運作的原因、閱讀 Android 的記錄，以及送出問題回報。
內容涵蓋 Windows，註明之處也涵蓋 Linux。

## 先檢查狀態

```
dsa status          # Android 的狀態、任何問題，以及任何待處理的更新
dsa start --wait    # 啟動 Android 並等到開機完成
```

DSA Settings 的 **General**（一般）頁面會顯示相同的狀態。

| `dsa status` | DSA Settings | 意義 |
|---|---|---|
| `Active` | **Running** | Android 已開機，其中的 DSA 代理程式已連線 |
| `Dozing` | **Dozing (idle)** | Android 處於閒置狀態。請參閱[記憶體](memory.md) |
| `Frozen` | **Frozen** | Android 已凍結，直到您開啟它的某個視窗為止 |
| `Stalled` | **Not ready** | Android 正在執行，但花費的時間太長。下一行會說明原因與處理方式 |

## 常見問題

| 問題 | 處理方式 |
|---|---|
| 安裝程式表示「虛擬機器平台」未開啟 | 開啟此 Windows 功能、重新啟動，然後再次執行安裝程式。請參閱[開始使用](getting-started.md) |
| 沒有開啟任何應用程式視窗 | 登出後再重新登入。視窗管理員會在登入時啟動 |
| 應用程式以不正確的大小開啟 | `dsa app-window <package> reset` |
| 映像更新後 Android 無法開機 | 不需要做任何事。DSA 會自行回到上一個映像，`dsa status` 也會說明這一點 |
| Android 沒有網路 | 開啟 DSA Settings 的 **Network**（網路）頁面，或執行 `dsa network`。它會顯示模式、位址、DNS 伺服器以及任何問題 |
| Android 損壞，其他方法都無效 | 重設 Android 的資料（見下文） |

### 重設 Android 的資料

重設會清除您安裝的每個應用程式、您的帳戶與所有應用程式資料。
接著 Android 會以剛安裝時的狀態啟動。
Android 映像與 DSA 的設定會保留。

- 在 DSA Settings 中：**Advanced**（進階）頁面 > **Reset Android's data**（重設 Android 的資料）> **Reset**（重設）。
- 在命令列中：`dsa reset-data` 會說明將會遺失哪些內容。
  `dsa reset-data --yes` 會將其清除。

## 閱讀 Android 的記錄

Android 的記錄保存在 Android 的記憶體中，絕不會寫入檔案。
只有在有程式讀取時，記錄才會傳到您的電腦。

- 在 DSA Settings 中：**Logs**（記錄）頁面。
  您可以依標籤或文字篩選，也可以暫停。
- 在命令列中：

  ```
  dsa logcat                        # 持續追蹤記錄；按 Ctrl+C 結束
  dsa logcat -d -b all *:W          # 記錄目前的內容，警告以上層級
  dsa logcat ActivityManager:I *:S  # 只顯示一個標籤
  ```

## 送出問題回報

1. 儲存診斷資料。
   在 **Logs** 頁面上選取 **Save diagnostics**（儲存診斷資料），或執行：

   ```
   dsa logs collect -o diag.zip
   ```

   這需要一分鐘或更久。
2. 將 zip 檔案附加到 [issue](https://github.com/thisisthepy/desktop-subsystem-for-android/issues) 中。

此封存檔包含 Android 的完整記錄、其當機記錄（tombstones、ANR 追蹤、dropbox、pstore）、Android 系統服務的摘要，以及這台電腦上與您有關的 DSA 記錄，但不包含其他使用者的記錄行。
封存檔中的 `summary.txt` 會說明哪些內容無法收集。

## Windows 上的記錄位置

| 資料夾 | 檔案 |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log`（視窗管理員）、`dsa-app-window.log`（應用程式視窗） |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log`（服務）、`android-<SID>.log`（每位使用者的 Android 主控台，每次開機最多 2 MiB）、`setup-<version>.log`（自我更新） |

若要開啟這些資料夾，請使用 DSA Settings 的 **Advanced** 頁面上的 **Open my log folder**（開啟我的記錄資料夾）與 **Open the service's log folder (administrators)**（開啟服務的記錄資料夾，限系統管理員）。

## Linux 上的記錄位置（尚未發行）

服務會寫入 systemd 日誌：

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## 所有指令

不加引數執行 `dsa` 會輸出所有指令。
加上 `--json` 可取得程式能讀取的輸出。

---

Android 是 Google LLC 的商標。
Windows 是 Microsoft 集團旗下公司的商標。
DSA 與 Google 或 Microsoft 沒有任何關聯，也未獲得其背書或贊助。
