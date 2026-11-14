# 故障排除与日志

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · [日本語](../ja/troubleshooting.md) · **简体中文** · [繁體中文](../zh-TW/troubleshooting.md) · [Español](../es/troubleshooting.md) · [Français](../fr/troubleshooting.md) · [Deutsch](../de/troubleshooting.md) · [Português (Brasil)](../pt-BR/troubleshooting.md) · [Русский](../ru/troubleshooting.md) · [Italiano](../it/troubleshooting.md) · [Tiếng Việt](../vi/troubleshooting.md) · [Bahasa Indonesia](../id/troubleshooting.md) · [Türkçe](../tr/troubleshooting.md)

本页帮助您找出 Desktop Subsystem for Android™（DSA）无法正常工作的原因，
读取 Android 的日志，并提交问题报告。
本页适用于 Windows，标注之处也适用于 Linux。

## 先检查状态

```
dsa status          # Android 的状态、出现的问题以及待安装的更新
dsa start --wait    # 启动 Android 并等待其启动完成
```

DSA Settings 的 **General**（常规）页面会显示相同的状态。

| `dsa status` | DSA Settings | 含义 |
|---|---|---|
| `Active` | **Running** | Android 已启动，其中的 DSA 代理已连接 |
| `Dozing` | **Dozing (idle)** | Android 处于空闲状态。请参见[内存](memory.md) |
| `Frozen` | **Frozen** | Android 已冻结，直到您打开它的某个窗口为止 |
| `Stalled` | **Not ready** | Android 正在运行，但耗时过长。下一行会说明原因和处理方法 |

## 常见问题

| 问题 | 处理方法 |
|---|---|
| 安装程序提示虚拟机平台未开启 | 开启该 Windows 功能，重启电脑，然后重新运行安装程序。请参见[入门](getting-started.md) |
| 没有打开任何应用窗口 | 注销后重新登录。窗口管理器在登录时启动 |
| 应用打开时大小不对 | `dsa app-window <package> reset` |
| 映像更新后 Android 无法启动 | 无需任何操作。DSA 会自行回到上一个映像，`dsa status` 会提示这一点 |
| Android 无法联网 | 打开 DSA Settings 的 **Network**（网络）页面，或运行 `dsa network`。它会显示模式、地址、DNS 服务器以及出现的问题 |
| Android 出现故障，其他方法都无效 | 重置 Android 的数据（见下文） |

### 重置 Android 的数据

重置会清除您安装的所有应用、您的账号以及所有应用数据。
之后 Android 会像刚安装时那样启动。
Android 映像和 DSA 的设置会保留。

- 在 DSA Settings 中：**Advanced**（高级）页面 >
  **Reset Android's data**（重置 Android 的数据）> **Reset**（重置）。
- 在命令行中：`dsa reset-data` 会说明哪些内容将会丢失。
  `dsa reset-data --yes` 会执行清除。

## 读取 Android 的日志

Android 的日志保存在 Android 的内存中，从不写入文件。
只有在有程序读取时，它才会传到您的电脑。

- 在 DSA Settings 中：**Logs**（日志）页面。您可以按标签或文本筛选，也可以暂停。
- 在命令行中：

  ```
  dsa logcat                        # 持续跟踪日志；按 Ctrl+C 结束
  dsa logcat -d -b all *:W          # 日志当前保存的内容，警告及以上级别
  dsa logcat ActivityManager:I *:S  # 只看一个标签
  ```

## 提交问题报告

1. 保存诊断信息。在 **Logs** 页面上选择 **Save diagnostics**（保存诊断信息），或运行：

   ```
   dsa logs collect -o diag.zip
   ```

   这需要一分钟或更长时间。
2. 把该 zip 文件附加到一个
   [议题](https://github.com/thisisthepy/desktop-subsystem-for-android/issues)中。

该压缩包包含 Android 的完整日志、
其崩溃记录（tombstone、ANR 跟踪、dropbox、pstore）、
Android 系统服务的摘要，
以及这台电脑上与您相关的 DSA 日志（不包含其他用户的记录）。
压缩包中的 `summary.txt` 会说明哪些内容未能收集。

## Windows 上的日志位置

| 文件夹 | 文件 |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log`（窗口管理器）、`dsa-app-window.log`（应用窗口） |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log`（服务）、`android-<SID>.log`（每个用户的 Android 控制台，每次启动最多 2 MiB）、`setup-<version>.log`（自动更新） |

要打开这些文件夹，请使用 DSA Settings 的 **Advanced** 页面上的
**Open my log folder**（打开我的日志文件夹）和
**Open the service's log folder (administrators)**（打开服务的日志文件夹，需要管理员）。

## Linux 上的日志位置（尚未发布）

各服务会写入 systemd 日志：

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## 所有命令

不带参数运行 `dsa` 会输出所有命令。
加上 `--json` 可以得到程序能够读取的输出。

---

Android 是 Google LLC 的商标。
Windows 是 Microsoft 集团公司的商标。
DSA 与 Google 或 Microsoft 没有关联，也未获得二者的认可或赞助。
