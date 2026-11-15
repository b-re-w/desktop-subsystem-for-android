# 記憶體與閒置行為

[English](../memory.md) · [한국어](../ko/memory.md) · [日本語](../ja/memory.md) · [简体中文](../zh-CN/memory.md) · **繁體中文** · [Español](../es/memory.md) · [Français](../fr/memory.md) · [Deutsch](../de/memory.md) · [Português (Brasil)](../pt-BR/memory.md) · [Русский](../ru/memory.md) · [Italiano](../it/memory.md) · [Tiếng Việt](../vi/memory.md) · [Bahasa Indonesia](../id/memory.md) · [Türkçe](../tr/memory.md)

Desktop Subsystem for Android™（DSA）會在 Android 執行期間將記憶體歸還給您的電腦，而不只是在它停止時。
這不需要任何設定。
本頁說明其中發生的事，以及如何查看。

## 查看 Android 使用多少記憶體

開啟 DSA Settings 的 **Memory**（記憶體）頁面，或執行 `dsa memory`。
兩者都會顯示：

- **On this PC**（在這台電腦上）：Android 虛擬機器、視窗管理員與每個應用程式視窗所佔用的記憶體。
- **Inside Android**（在 Android 內）：使用中的記憶體、快取、壓縮記憶體，以及最大的處理程序。
  Android 會在啟動幾分鐘後回報這些資訊。

## Doze，閒置狀態

當每個 Android 視窗都已關閉或最小化達 30 秒時，Android 的螢幕會關閉，Android 進入 Doze，就像放在口袋裡的手機一樣。

- 在 Doze 期間，通知、鬧鐘與小工具都會繼續運作。
  在這之間 CPU 會休息。
- `dsa status` 會顯示 `Dozing`。
  DSA Settings 的 **General**（一般）頁面會顯示 **Dozing (idle)**。
- 顯示任何 Android 視窗都會立即喚醒 Android。

Android 也會跟隨電腦的睡眠。
它會在電腦睡眠前儲存其狀態，並在電腦喚醒時設定時鐘。

## 凍結（預設為關閉）

凍結會停止 Android 中的每個處理程序。
此時它不使用 CPU，但通知、鬧鐘與小工具也會停止。
因此，除非您要求，DSA 絕不會凍結 Android。

- 立即凍結：在 **General** 頁面的 **Freeze Android now**（立即凍結 Android）下選取 **Freeze**（凍結），或執行 `dsa freeze`。
- 在 Doze 持續 30 分鐘後凍結：在 **Advanced**（進階）頁面上開啟 **Freeze Android when idle**（閒置時凍結 Android），或執行 `dsa settings set freeze on`。
- 開啟 Android 視窗，或在視窗中輸入，都會讓 Android 恢復執行。

## 記憶體如何歸還

- Android 虛擬機器只在 Android 第一次使用記憶體時，才向您的電腦取得記憶體。
- Android 釋放的記憶體會在 Android 執行期間歸還給您的電腦。
- 當應用程式的最後一個視窗關閉時，Android 會結束該應用程式的處理程序，因此其記憶體會在幾秒內歸還。
  在桌面上有小工具的應用程式，或仍在背景工作的應用程式（例如音樂播放器），則會繼續執行。
- 在沒有視窗、沒有通知、沒有聲音也沒有小工具的情況下仍讓背景服務持續執行的應用程式，會在五分鐘後被停止。
  如果 Android 一再重新啟動它，DSA 每次都會等待更久，最長一小時。
- 在沒有任何應用程式視窗開啟時，Android 會捨棄其檔案快取，並將閒置的記憶體移到壓縮記憶體，讓更多記憶體可以歸還。
- 視窗管理員會在最後一個應用程式視窗關閉 30 秒後，歸還它未使用的記憶體。

## 記憶體上限

上限預設為 **Automatic**（自動），也就是不設定固定上限。
Android 以 2 GB 啟動，並在執行期間成長，最多可達您電腦的記憶體減去 Windows 保留的部分（記憶體的 10% 或 2 GB，取較大者）。
Android 只佔用它所使用的記憶體。
若要設定上限，請使用 **Memory** 頁面上的 **Memory limit**（記憶體上限），或執行：

```
dsa settings set memory-limit 4g     # 4 GiB
dsa settings set memory-limit auto   # 回到自動
```

新的上限會在 Android 下次啟動時生效。

---

Android 是 Google LLC 的商標。
Windows 是 Microsoft 集團旗下公司的商標。
DSA 與 Google 或 Microsoft 沒有任何關聯，也未獲得其背書或贊助。
