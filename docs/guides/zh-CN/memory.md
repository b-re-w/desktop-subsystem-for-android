# 内存与空闲行为

[English](../memory.md) · [한국어](../ko/memory.md) · [日本語](../ja/memory.md) · **简体中文** · [繁體中文](../zh-TW/memory.md) · [Español](../es/memory.md) · [Français](../fr/memory.md) · [Deutsch](../de/memory.md) · [Português (Brasil)](../pt-BR/memory.md) · [Русский](../ru/memory.md) · [Italiano](../it/memory.md) · [Tiếng Việt](../vi/memory.md) · [Bahasa Indonesia](../id/memory.md) · [Türkçe](../tr/memory.md)

Desktop Subsystem for Android™（DSA）在 Android 运行期间就会把内存归还给您的电脑，
而不只是在它停止时。
您无需为此进行任何设置。
本页说明其中发生了什么，以及如何查看。

## 查看 Android 使用了多少内存

打开 DSA Settings 的 **Memory**（内存）页面，或运行 `dsa memory`。两者都会显示：

- **On this PC**（在这台电脑上）：Android 虚拟机、窗口管理器和每个应用窗口占用的内存。
- **Inside Android**（在 Android 内部）：已用内存、缓存、压缩内存以及占用最多的进程。
  Android 会在启动几分钟后报告这些信息。

## Doze，即空闲状态

当所有 Android 窗口都已关闭或最小化 30 秒后，
Android 的屏幕会关闭，Android 进入 Doze，就像放在口袋里的手机一样。

- 在 Doze 中，通知、闹钟和小部件仍然正常工作。其间 CPU 会休息。
- `dsa status` 会显示 `Dozing`。
  DSA Settings 的 **General**（常规）页面会显示 **Dozing (idle)**。
- 显示任何一个 Android 窗口都会立即唤醒 Android。

Android 还会跟随电脑的睡眠。
它会在电脑睡眠前保存状态，并在电脑唤醒时校准时钟。

## 冻结（默认关闭）

冻结会停止 Android 中的所有进程。
此时它不占用 CPU，但通知、闹钟和小部件也会停止。
因此，除非您要求，DSA 绝不会冻结 Android。

- 立即冻结：使用 **General** 页面上 **Freeze Android now**（立即冻结 Android）下的
  **Freeze**，或运行 `dsa freeze`。
- 在 Doze 中持续 30 分钟后冻结：在 **Advanced**（高级）页面上开启
  **Freeze Android when idle**（空闲时冻结 Android），
  或运行 `dsa settings set freeze on`。
- 打开一个 Android 窗口，或在窗口中进行输入，都会让 Android 恢复运行。

## 内存如何归还

- Android 虚拟机只在 Android 第一次使用某块内存时才从电脑获取它。
- Android 释放的内存会在 Android 运行期间归还给电脑。
- 当应用的最后一个窗口关闭时，Android 会结束该应用的进程，
  因此其内存会在几秒内归还。
  在桌面上有小部件的应用，或仍在后台工作的应用（例如音乐播放器），会继续运行。
- 如果某个应用在没有窗口、没有通知、没有声音、也没有小部件的情况下
  一直保持后台服务运行，它会在五分钟后被停止。
  如果 Android 一再重新启动它，DSA 每次都会等待更长时间，最长一小时。
- 没有打开任何应用窗口时，Android 会丢弃其文件缓存，并把空闲内存移入压缩内存，
  以便归还更多内存。
- 最后一个应用窗口关闭 30 秒后，窗口管理器会归还它不用的内存。

## 内存上限

上限默认为 **Automatic**（自动），即不设固定上限。
Android 启动时使用 2 GB，并在运行中逐渐增长，
最多可用到电脑内存减去 Windows 保留的部分
（内存的 10% 或 2 GB，取较大者）。
Android 只占用它实际使用的内存。
如需设置上限，请使用 **Memory** 页面上的 **Memory limit**（内存上限），或：

```
dsa settings set memory-limit 4g     # 4 GiB
dsa settings set memory-limit auto   # 恢复为自动
```

新的上限在 Android 下一次启动时生效。

---

Android 是 Google LLC 的商标。
Windows 是 Microsoft 集团公司的商标。
DSA 与 Google 或 Microsoft 没有关联，也未获得二者的认可或赞助。
