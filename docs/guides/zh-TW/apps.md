# 應用程式與視窗

[English](../apps.md) · [한국어](../ko/apps.md) · [日本語](../ja/apps.md) · [简体中文](../zh-CN/apps.md) · **繁體中文** · [Español](../es/apps.md) · [Français](../fr/apps.md) · [Deutsch](../de/apps.md) · [Português (Brasil)](../pt-BR/apps.md) · [Русский](../ru/apps.md) · [Italiano](../it/apps.md) · [Tiếng Việt](../vi/apps.md) · [Bahasa Indonesia](../id/apps.md) · [Türkçe](../tr/apps.md)

每個 Android 應用程式工作都會開啟為獨立的桌面視窗，並有自己的工作列按鈕。
沒有圍住手機畫面的模擬器外框。
本頁說明 Windows 上的 Desktop Subsystem for Android™（DSA）。

## 開啟應用程式

- **開始功能表。** 每個已安裝的應用程式都有一個使用其名稱與圖示的開始功能表項目，因此 Windows 搜尋找得到它。
  Android 未執行時，開啟它會先啟動 Android。
- **DSA Settings。** 在 **Apps**（應用程式）頁面上，選取應用程式旁的 **Open**（開啟）。
- **命令列。** `dsa apps list` 會列出已安裝的應用程式及其套件。
  `dsa launch <package>` 會開啟其中一個，例如 `dsa launch com.android.settings`。

如果其他程式的開始功能表項目已經使用了該應用程式的名稱，DSA 會在名稱後加上套件，例如 `Clock (com.example.clock)`。
DSA 只會變更或移除它自己建立的項目。

## 桌面捷徑與跳躍清單

應用程式圖示的運作方式與 Windows 應用程式相同，而不是像 Android 主畫面那樣。

- **桌面捷徑。** 在 DSA Settings 的 **Apps** 頁面上，選取應用程式旁的 **Create desktop shortcut**（建立桌面捷徑）。
  您也可以在應用程式的工作列按鈕上按一下滑鼠右鍵，然後選取 **Create desktop shortcut**。
  從命令列：`dsa apps desktop-shortcut <package>`。
  捷徑開啟應用程式的方式與其開始功能表項目相同。
- **應用程式捷徑。** Android 應用程式提供的捷徑（例如通訊軟體的新聊天），會在您以滑鼠右鍵按一下應用程式的工作列按鈕或其開始功能表項目時顯示。
  選取其中一個，就會在視窗中開啟應用程式的該畫面；Android 未執行時會先啟動 Android。
  應用程式變更其捷徑時，清單也會跟著變更。
  從命令列：`dsa apps shortcut <package> <id>`；`dsa apps info <package> --json` 會列出 ID。
- **應用程式要求建立的捷徑。** 當應用程式要求將其某個捷徑加到您的桌面時，DSA 會先詢問您，只有在您同意時才會加入。

應用程式變更名稱時（例如更新之後），其開始功能表項目、桌面捷徑與工作列上的釘選都會跟著改用新名稱，並繼續正常運作。
您刪除的桌面捷徑不會再被建立。
解除安裝應用程式時，也會移除其桌面捷徑。

## 操作應用程式視窗

- 每個工作都有自己的視窗、工作列按鈕與 Alt+Tab 項目，並顯示應用程式的名稱與圖示。
  工作管理員會以每個應用程式自己的名稱列出它。
- 標題列是應用程式的一部分：應用程式的色彩延伸到視窗頂端，Windows 11 的標題列按鈕疊在上方。
  應用程式的圖示與名稱位於左側。
  當應用程式可以返回時，圖示會變成返回箭頭；選取它即可返回。
- 貼齊版面配置（將滑鼠停在最大化按鈕上）、拖曳標題列、按兩下標題列來最大化，以及即時調整大小都能使用。
  調整大小不會重新啟動應用程式。
  最大化的視窗會填滿螢幕的工作區域，並保持工作列可見。
- 沒有 Android 導覽列。
  應用程式的底部列位於視窗的底部邊緣，下方沒有空白帶。
- 滑鼠（含停留）、觸控、手寫筆（壓力、橡皮擦與筆身按鈕）與鍵盤都能使用。
  文字來自您的 Windows 輸入法，包括韓文與其他語言；組字會直接顯示在 Android 編輯器中，選字視窗就在游標旁邊。
  沒有 Android 鍵盤應用程式。
- 可以將檔案從檔案總管拖曳到應用程式的視窗上，或將檔案從應用程式拖曳到檔案總管中。
- 朗讀程式（Narrator）與 NVDA 等螢幕閱讀程式會透過 UI Automation（使用者介面自動化）朗讀應用程式。
- 關閉視窗會要求 Android 結束該工作。

## 視窗功能表

以滑鼠右鍵按一下標題列、選取應用程式的圖示，或按 Alt+Space。
在一般的 Windows 項目之後，功能表還有：

- **Full screen**（全螢幕）：與 F11 一樣填滿整個螢幕。
  再次選取它或按 F11 即可離開。
- **New window**（新視窗）：開啟應用程式的另一個視窗，適用於支援多個視窗的應用程式。
- **Screenshot**（螢幕擷取畫面）：將應用程式的畫面（不含標題列）複製到剪貼簿，並儲存在「圖片」中的「螢幕擷取畫面」資料夾。
- **Aspect ratio**（長寬比）：開啟 Android 中該應用程式的長寬比設定，適用於在大視窗中看起來被拉長的應用程式。
- **Open by default**（預設開啟）：選擇應用程式支援的連結要在應用程式中開啟，還是在瀏覽器中開啟。
- **App info**（應用程式資訊）：在 DSA Settings 的 **Apps** 頁面上開啟該應用程式的項目。

## 全螢幕

當應用程式要求全螢幕時，其視窗會以下列兩種方式之一顯示：

- **在整個螢幕上**：視窗填滿螢幕，標題列會隱藏。
  以指標碰觸螢幕頂端邊緣即可顯示標題列，其中有最小化與關閉；中間的按鈕會離開全螢幕。
- **在視窗內**：視窗保持其大小與標題列，由應用程式填滿視窗。

應用程式提出要求的方式有兩種，每一種都有自己的預設值：

| 要求的種類 | 範例 | 預設值 |
|---|---|---|
| 明確的全螢幕要求 | 影片播放器的全螢幕按鈕 | 在整個螢幕上 |
| 只隱藏各列 | 短影片動態 | 在視窗內 |

您可以在 DSA Settings 的 **Display & GPU**（顯示與 GPU）頁面上，於 **App full screen**（應用程式全螢幕）與 **Apps that hide their bars**（隱藏各列的應用程式）下變更預設值；或在 **Apps** 頁面上，於 **Full screen request**（全螢幕要求） 與 **When only the bars hide**（僅隱藏列時）下為單一應用程式變更。
應用程式自己的選擇優先。
不論設定為何，F11 與視窗功能表中的 **Full screen** 隨時都能切換整個螢幕的全螢幕。

從命令列：

```
dsa settings set app-fullscreen screen|window           # 明確要求的預設值
dsa settings set app-immersive screen|window            # 隱藏各列的預設值
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## 設定應用程式的視窗大小

在 DSA Settings 的 **Apps** 頁面上：

- 關閉 **Resizable**（可調整大小），讓應用程式的視窗保持開啟時的大小。
- 選擇應用程式視窗開啟時的大小：由 Android 決定的大小，或直向或橫向的手機或平板大小。

變更會套用到應用程式開啟的下一個視窗。
從命令列：

```
dsa app-window <package> --fixed-size on        # 無法調整大小或最大化
dsa app-window <package> --start-size 1280x800  # 以此大小開啟；auto 由 Android 決定
dsa app-window <package> --dpi-scaling system   # 在其他螢幕上的縮放方式；請參閱下文
dsa app-window <package> reset                  # 回到預設值
```

## 讓應用程式從標題列下方開始

應用程式會一直繪製到視窗頂端，也就是標題列下面。因此有些應用程式的頂端列會被擠壓，或有一部分被標題列遮住。
對於這樣的應用程式，請在 DSA Settings 的 **Apps** 頁面開啟 **Start below the title bar**（從標題列下方開始）。應用程式的內容隨後從標題列下方開始，
標題列則採用應用程式在那裡顯示的顏色。預設為關閉，下次開啟該應用程式的視窗時生效。在命令列中：

```
dsa app-window <package> --below-title-bar on   # off 回到預設值
```

## 將應用程式移到縮放比例不同的螢幕

Android 以主螢幕的縮放比例啟動，因此應用程式在主螢幕上會以正確的大小開啟。
將視窗移到縮放比例不同的螢幕時，應用程式在螢幕上的大小會保持不變。
每個應用程式會以下列其中一種方式處理，您可以在 **Apps** 頁面的 **Scaling**（縮放）下選擇，或使用 `dsa app-window <package> --dpi-scaling`：

- **Automatic**（自動，`auto`，預設值）：對於 DSA 內建清單中那些執行時變更縮放比例就會出問題的應用程式使用 **System**，其他應用程式則使用 **App**。
  這份清單包含 YouTube：它會維持開始時的縮放比例所對應的版面配置，因此移動之後，其底部索引標籤與迷你播放器會一直顯示異常，直到重新啟動為止。
  Settings 會將這類應用程式的預設值顯示為 **Automatic (system, recommended for this app)**。
  您為該應用程式所做的選擇優先。
- **App**（應用程式，`app`）：拖曳視窗時，DSA 會將其縮放到新的螢幕。
  放開後，應用程式會以新螢幕的縮放比例重新繪製，文字保持清晰：Android 會重新建立其畫面一次並還原其狀態（捲動位置、已輸入的文字），因此每次移動後會短暫重新繪製，就像 Windows 程式使用「應用程式」縮放時一樣。
  即使應用程式聲稱能自行處理縮放比例的變更，也是如此，因為其中有些應用程式只會以新的縮放比例重新繪製部分畫面。
- **System**（系統，`system`）：應用程式保持其縮放比例，由 DSA 將其視窗縮放到新的螢幕。
  應用程式永遠不會被重新建立。
  放大後，文字可能看起來稍微模糊。
- **System (forced)**（系統（強制），`system-forced`）：在不重新啟動的情況下，將新的縮放比例提供給應用程式，即使它沒有聲稱能處理。
  尚未提供：這需要對 Android 進行此版本尚未包含的變更，因此在那之前，其運作方式與 **System** 相同。

應用程式中的文字會遵循 Windows 的文字大小（設定 > 協助工具 > 文字大小）。

## 通知

Android 通知會以應用程式的名稱與圖示顯示為 Windows 通知。

- 選取通知即可開啟應用程式。
- 關閉通知時，Android 中的通知也會一併關閉，除非應用程式讓它持續顯示。
- Android 處於 Doze 時，通知仍會持續送達。
  請參閱[記憶體](memory.md)。

## 在背景執行的應用程式

視窗關閉後仍繼續執行的應用程式（例如音樂播放器），會在通知區域中顯示為圖示。
其工具提示就是該應用程式的通知。

- 選取圖示即可開啟應用程式。
- 以滑鼠右鍵按一下圖示，可使用 **Open**、通知本身的動作，以及 **Stop**（停止）。
- **Stop** 會先詢問。
  勾選 **Do not ask again**（不要再詢問），之後就會略過此問題。
  若要再次詢問，請在 DSA Settings 的 **Running in the background**（在背景執行）頁面上開啟 **Ask before stopping an app from the notification area**（從通知區域停止應用程式前先詢問）。
- 播放媒體的應用程式也支援媒體鍵與 Windows 媒體控制項。

DSA Settings 的 **Running in the background** 頁面會列出這些應用程式，每個都有 **Open** 與 **Stop**。
從命令列：`dsa background`、`dsa background open <package>` 與 `dsa background stop <package>`。

## 桌面上的小工具

Android 小工具可以放在您的桌面上，擁有自己的視窗。
它會停留在其他視窗下方，在您顯示桌面時仍然存在，而且不會出現在工作列或 Alt+Tab 中。

1. 開啟 DSA Settings 的 **Widgets**（小工具）頁面。
2. 在 **Widgets your apps offer**（應用程式提供的小工具）下，選取小工具旁的 **Add to desktop**（新增到桌面）。
   如果應用程式要求您設定小工具，其設定畫面會在視窗中開啟。
3. 若要移動小工具，請按住它的任何位置並保持不動半秒。
   它會浮起，並顯示邊框，在可以調整大小的每個邊緣上都有控點。
   拖曳它即可移動，拖曳控點即可調整大小。
   快速拖曳仍由小工具本身處理，因此其清單依然可以捲動。
   您也可以直接拖曳小工具的邊緣來調整大小，不必先讓它浮起；只有可以調整大小的邊緣才能這樣做。
   DSA 會記住您放置的位置與大小。
4. 以滑鼠右鍵按一下小工具，或在長按後放開，即可使用 **Widget settings**（小工具設定，當小工具可以重新設定時）、**Resize**（調整大小，從它允許的大小中選擇一個）以及 **Remove**（移除）。
   功能表鍵與 Shift+F10 也能開啟此功能表。
   方向鍵、Enter 與 Esc 都能在此功能表中使用，按下字母則會選取項目：R 代表 **Resize**，M 代表 **Remove**。

從命令列：`dsa widgets list`、`dsa widgets add <provider>` 與 `dsa widgets remove <id>`。

## 連結與剪貼簿

- 在 Android 應用程式中點選的網頁連結，會在您的預設瀏覽器中開啟。
  已安裝的應用程式驗證為自己所有的連結，則會在該應用程式中開啟。
- 若要改為在 Android 應用程式中開啟所有網頁連結，請在 DSA Settings 的 **Advanced**（進階）頁面上開啟 **Open web links in Android apps**（在 Android 應用程式中開啟網頁連結）（`dsa settings set links-in-android on`）。
- 您複製的文字會在 Windows 與 Android 之間雙向共用。
  若要停止，請在 **Advanced** 頁面上關閉 **Share the clipboard with Android**（與 Android 共用剪貼簿）（`dsa settings set clipboard off`）。

## Google Play 與受保護的影片

DSA 不隨附任何 Google 軟體。
如果您需要，DSA 會先向您顯示來源與條款，再從來源替您下載。
兩者都會從 Android 下次啟動時生效，也都可以再移除。

- **Google Play。** 在 DSA Settings 的 **Advanced** 頁面上，選取 **Google Play** 下的 **Install**（安裝），或執行 `dsa gapps install`。
  DSA 會為您的 Android 版本下載公開的 LiteGapps Core 套件（Google Play 服務、Play 商店與帳戶同步）。
  此 Android 未經 Google 認證，因此在您註冊裝置之前，Google 可能會拒絕讓您登入：**Register this device with Google**（向 Google 註冊此裝置）會顯示 Android ID 並開啟註冊頁面。
  `dsa gapps status` 會顯示狀態，`dsa gapps remove` 則會移除 Google 的應用程式，即使 Android 已停止也可以。
- **Widevine。** 串流應用程式需要 Google 的 Widevine 模組才能播放受保護的影片。
  在 **Advanced** 頁面上，選取 **Protected content**（受保護的內容）下的 **Get**（取得），或執行 `dsa drm widevine install`。
  DSA 會下載適用於您 Android 版本的 Google Android 模擬器映像（約 2 GB），只保留未經修改的 Widevine 模組，並刪除其餘部分。
  這裡的 Widevine 為安全性等級 L3，因此串流服務最多只提供標準畫質；若要觀看高畫質，請使用 Windows 的瀏覽器或應用程式。
  `dsa drm widevine remove` 會將其移除。

## 僅為 ARM 建置的應用程式

在 x64 電腦上，只有 ARM 處理器程式碼的應用程式會透過 ARM 轉譯器執行。
x64 電腦上的 Android 隨附一個轉譯器，也就是開放原始碼轉譯器 Digitalis，並從第一次啟動就使用它。
您也可以改用其他轉譯器：

- **Google 的 ARM 轉譯器。** 在 **Advanced** 頁面上，選取 **Google's ARM translator**（Google 的 ARM 轉譯器）旁的 **Get**，或執行 `dsa arm-translation fetch google`。
  DSA 會先顯示它的來源與 Google 的條款（[Android SDK License](https://developer.android.com/studio/terms)），在您接受之前不會下載任何東西。
  接著，您的電腦會下載 Google 為您的 Android 版本提供的 Android 模擬器映像（約 2 GB），只保留轉譯器，並刪除其餘部分。
  DSA 不會隨附或轉發 Google 的轉譯器。
- **您自行建置或從其他地方取得的套件**，例如以開放原始碼轉譯器 Digitalis 製作的套件：`dsa arm-translation install <folder>`。

接著重新啟動 Android：在 DSA Settings 的 **General**（一般）頁面上將它停止，然後開啟一個應用程式。

若要回到 Android 隨附的轉譯器，請在 **Advanced** 頁面上選取 **Use the default translator**（使用預設轉譯器）旁的 **Use**（使用），或執行 `dsa arm-translation use-default`。
這會從 Android 下次啟動時起，移除您安裝的套件。

`dsa arm-translation status` 會說明 Android 使用的是哪個轉譯器。
**Advanced** 頁面上的 **Run apps built only for ARM**（執行僅為 ARM 建置的應用程式）開關，可在不移除任何東西的情況下開啟或關閉轉譯。
對於同時具有 ARM 與 x64 程式碼的應用程式，**Apps** 頁面上的 **Run as ARM**（以 ARM 執行）（`dsa apps run-as-arm <package> on`）會改為執行其 ARM 程式碼。
只能執行 64 位元 ARM 應用程式（`arm64-v8a`）。
在 ARM64 電腦上，Android 會自行執行 ARM 應用程式，不需要轉譯器。

## Root 存取權

有些應用程式（例如備份工具）需要 root。
DSA 提供 root 的方式就像 Linux 電腦上的 `sudo`：只有在電腦的系統管理員核准時才提供，而且一次一個要求。
此功能預設為關閉，未獲允許的應用程式完全看不到 `su`。

1. 系統管理員在 DSA Settings 的 **Advanced** 頁面上，於 **Root access**（root 存取權）下開啟 **Let Android apps ask for root**（允許 Android 應用程式要求 root）（`dsa settings set root-elevation on`）。
   此設定從 Android 下次啟動時生效。
2. 在 **Apps** 頁面上，設定應用程式的 **Root access**：
   - **Root: not allowed**（不允許，預設值）：應用程式沒有 `su`。
   - **Root: ask each time**（每次詢問）：每次應用程式使用 `su` 時，Windows 都會以應用程式自己的名稱與圖示顯示系統管理員提示（UAC，使用者帳戶控制）。
     標準使用者需輸入系統管理員的密碼。
     核准的效力對該應用程式持續 15 分鐘。
   - **Root: remember (no asking)**（記住，不詢問）：應用程式使用 `su` 時不會詢問。
     選擇此項需要系統管理員核准。

   從命令列：`dsa apps root <package> off|ask|remember`。

開啟開發人員模式時，`adb shell` 也可以使用 `su`。
它在 **Apps** 頁面上有自己的一列 **adb shell**，預設為每次詢問（`dsa apps root adb ask`）。
其提示的名稱為 **adb shell**，並會顯示指令。

當 Android 本身的某個部分提出要求時，提示會顯示其名稱：**Android system**、**Android phone services**、**Android Bluetooth**、**Android NFC** 或 **Android system service**。
共用同一個使用者 ID 的應用程式會以一個群組核准，提示會列出所有這些應用程式。

每一次要求都會連同應用程式、時間、指令以及是否核准，記錄在服務記錄中。
關閉此功能會立即對所有應用程式隱藏 `su`，不需重新啟動。

---

Android 是 Google LLC 的商標。
Windows 是 Microsoft 集團旗下公司的商標。
DSA 與 Google 或 Microsoft 沒有任何關聯，也未獲得其背書或贊助。
