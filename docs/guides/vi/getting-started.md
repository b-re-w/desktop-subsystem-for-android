# Bắt đầu

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · [日本語](../ja/getting-started.md) · [简体中文](../zh-CN/getting-started.md) · [繁體中文](../zh-TW/getting-started.md) · [Español](../es/getting-started.md) · [Français](../fr/getting-started.md) · [Deutsch](../de/getting-started.md) · [Português (Brasil)](../pt-BR/getting-started.md) · [Русский](../ru/getting-started.md) · [Italiano](../it/getting-started.md) · **Tiếng Việt** · [Bahasa Indonesia](../id/getting-started.md) · [Türkçe](../tr/getting-started.md)

Desktop Subsystem for Android™ (DSA) chạy Android trong một máy ảo nhẹ với kernel riêng. Mỗi ứng
dụng Android mở trong một cửa sổ riêng trên desktop của bạn.

## Trạng thái theo nền tảng

| Máy chủ | Trạng thái |
|---|---|
| Windows 11, x64 | Đã phát hành. |
| Windows 11, ARM64 | Đã phát hành. Android cũng là ARM64, nên ứng dụng ARM chạy không cần dịch. |
| Linux, x64 và ARM64 | Đang phát triển. Các gói `.deb` và `.rpm` được xây dựng trong CI, chưa phát hành. |
| macOS, Apple silicon | Đang phát triển. `DSA.app`, gói `.pkg` và `.dmg` của nó được xây dựng trong CI, chưa phát hành. |

## Cài đặt trên Windows

Bạn cần Windows 11 trên PC x64 hoặc ARM64, với tính năng Windows **Nền tảng Máy ảo** (Virtual
Machine Platform) đã bật.

1. Bật Nền tảng Máy ảo. Mở **Bật hoặc tắt tính năng Windows**, đánh dấu **Nền tảng Máy ảo**, chọn
   **OK** và khởi động lại. Hoặc chạy
   `dism /online /enable-feature /featurename:VirtualMachinePlatform /all` trong một terminal quản
   trị viên. Trình cài đặt không tự bật tính năng này cho bạn: nó dừng lại kèm thông báo nếu tính
   năng đang tắt.
2. Tải chương trình cài đặt và `SHA256SUMS.txt` từ
   [bản phát hành mới nhất](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest):
   `DSA-v<version>-win-x64-setup.exe` trên PC x64, `DSA-v<version>-win-arm64-setup.exe` trên PC
   ARM64.
3. Kiểm tra tệp đã tải. Mã băm mà lệnh này in ra phải khớp với dòng trong `SHA256SUMS.txt`:

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. Chạy trình cài đặt, chọn **Install** (cài đặt) và chấp thuận lời nhắc quản trị viên.
5. Chờ ảnh Android đầu tiên. Khi trình cài đặt không mang sẵn ảnh, dịch vụ DSA tải nó xuống ở chế
   độ nền. `dsa status` hiển thị tiến trình tải.
6. Mở một ứng dụng từ menu Bắt đầu, hoặc kiểm tra từ terminal:

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## Trình cài đặt cài những gì

- Dịch vụ `DsaHost`, dịch vụ chạy Android của từng người dùng.
- Lệnh `dsa`.
- **DSA Settings** trong menu Bắt đầu.
- **Android** trong ngăn điều hướng của File Explorer, mục này mở tệp của Android.
- APK Installer (trình cài APK), chương trình mở tệp `.apk`.
- DSA Window Manager (`dsa-wm`). Nó khởi động khi bạn đăng nhập và khởi động Android ngay lúc đó,
  nên ứng dụng đầu tiên bạn mở không phải chờ Android khởi động.

Mỗi người dùng Windows có Android riêng và đĩa dữ liệu riêng, `%LOCALAPPDATA%\DSA\android.vhdx`.
Người dùng khác không đọc được nó.

DSA dùng kernel riêng và máy ảo riêng. Nó không dùng hay thay đổi cài đặt WSL, kernel hay các bản
phân phối WSL của bạn.

## Cập nhật

DSA tự cập nhật chính nó và ảnh Android. Cả hai đều được ký, và DSA không áp dụng gì nếu chữ ký,
kích thước hoặc mã băm không khớp.

- **Bản thân DSA** được cập nhật khi không có Android của người dùng nào đang chạy. Cho đến lúc
  đó, bản cập nhật chờ. `dsa update` cài nó ngay lập tức. DSA không bao giờ xuống phiên bản thấp
  hơn.
- **Kênh phát hành.** Từ 1.0, DSA có một kênh ổn định và một kênh phát hành trước cho các phiên
  bản vẫn đang được hoàn thiện. Kênh của bạn bắt đầu theo trình cài đặt bạn đã dùng, và các lần
  nâng cấp giữ nguyên nó. Quản trị viên đổi kênh bằng **Get pre-release versions** (nhận phiên bản
  phát hành trước) trên trang **Updates** (cập nhật) của DSA Settings. Mọi bản phát hành trước 1.0
  đều là bản phát hành trước.
- **Ảnh Android** không phụ thuộc vào kênh. Nếu một ảnh không khởi động được, DSA quay lại ảnh
  trước đó, và `dsa status` cho biết điều đó.

Quản trị viên có thể tắt cập nhật trên trang **Updates** của DSA Settings. Xem
[Cài đặt](settings.md) để biết mọi công tắc.

## Linux (đang phát triển, chưa phát hành)

DSA cho Linux được xây dựng dưới dạng `.deb` (Debian, Ubuntu) và `.rpm` (Fedora, openSUSE). Nó cần
KVM (`/dev/kvm`) và một desktop Wayland. Android chạy trong một máy ảo crosvm.

Cài một gói cũng sẽ thêm kho gói có ký của DSA, nên các phiên bản sau đến qua `apt`, `dnf`,
`zypper` hoặc trung tâm phần mềm. Trên Linux, DSA không có trình cập nhật riêng.

- Debian và Ubuntu: `/etc/apt/sources.list.d/dsa.sources`, với khóa trong
  `/usr/share/keyrings/dsa-archive-keyring.gpg`.
- Fedora và openSUSE: `/etc/yum.repos.d/dsa.repo` (và `/etc/zypp/repos.d/dsa.repo` nếu có zypper),
  với khóa trong `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa`.

Kho gói sẽ được phục vụ từ `https://thisisthepy.github.io/desktop-subsystem-for-android/`. Nó chưa
được xuất bản.

Sau khi phát hành, lần cài đầu tiên trông như sau:

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian, Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

Gói này cài dịch vụ hệ thống `dsa-hostd`, các dịch vụ người dùng `dsa-devices` và
`dsa-compositor`, lệnh `dsa` trong `/usr/bin`, mục menu **DSA Files**, và các thao tác tệp
**Open in an Android app**, **Share with an Android app** và **Install in Android**.

## macOS (đang phát triển, chưa phát hành)

DSA cho macOS chạy Android trên Virtualization framework của Apple, trên Apple silicon. Nó được
xây dựng thành `DSA.app`, trong một gói cài đặt (`DSA-<version>.pkg`) và một ảnh đĩa
(`DSA-<version>.dmg`). Các bản dựng CI được ký tạm thời (ad hoc), nên chúng chỉ chạy trên máy đã
xây dựng chúng. Chưa có bản phát hành nào được ký bằng Developer ID.

## Tiếp theo

- [Ứng dụng và cửa sổ](apps.md)
- [Cài đặt APK](install-apks.md)

---

Android là nhãn hiệu của Google LLC. Windows là nhãn hiệu của nhóm công ty Microsoft. macOS là
nhãn hiệu của Apple Inc. DSA không liên kết với Google, Microsoft hay Apple, và không được các công
ty này xác nhận hoặc tài trợ.
