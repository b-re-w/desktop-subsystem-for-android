# 开发者模式与 adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · [日本語](../ja/developer-mode.md) · **简体中文** · [繁體中文](../zh-TW/developer-mode.md) · [Español](../es/developer-mode.md) · [Français](../fr/developer-mode.md) · [Deutsch](../de/developer-mode.md) · [Português (Brasil)](../pt-BR/developer-mode.md) · [Русский](../ru/developer-mode.md) · [Italiano](../it/developer-mode.md) · [Tiếng Việt](../vi/developer-mode.md) · [Bahasa Indonesia](../id/developer-mode.md) · [Türkçe](../tr/developer-mode.md)

开发者模式让您电脑上的 Android 调试桥（adb）
连接到 Desktop Subsystem for Android™（DSA）中的 Android。
它默认关闭。

adb 不属于 DSA。请从 Android SDK Platform-Tools 获取它。

## 连接 adb

1. 打开 DSA Settings 的 **Network**（网络）页面，
   开启 **Developer mode**（开发者模式）。
   也可以运行 `dsa settings set developer-mode on`。
2. 如果 Android 未运行，请启动它：`dsa start --wait`。
3. 连接：

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   Android 不会要求您允许此连接。
   该端口只在这台电脑上响应，因此其他计算机无法连接。
4. 用完后，请再次关闭 **Developer mode**，
   或运行 `dsa settings set developer-mode off`。

**Network** 页面上的 **Connect adb**（连接 adb）会显示确切的命令，
`dsa settings` 也会在 `Developer mode` 一行中输出该命令。

## 使用其他端口

默认端口为 58526。
如需更改，请编辑 **Connect adb** 下的 **Port**（端口），或运行：

```
dsa settings set adb-port 5555
```

## 开发者模式不会改变的内容

DSA 自身的工作从不使用 adb。
窗口、输入和安装都使用 DSA 自己的控制通道，
所以即使关闭开发者模式，一切也都照常工作。

读取 Android 的日志不需要 adb。请参见[故障排除](troubleshooting.md)。

---

Android 是 Google LLC 的商标。
DSA 与 Google 没有关联，也未获得其认可或赞助。
