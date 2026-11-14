# Khắc phục sự cố và nhật ký

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · [日本語](../ja/troubleshooting.md) · [简体中文](../zh-CN/troubleshooting.md) · [繁體中文](../zh-TW/troubleshooting.md) · [Español](../es/troubleshooting.md) · [Français](../fr/troubleshooting.md) · [Deutsch](../de/troubleshooting.md) · [Português (Brasil)](../pt-BR/troubleshooting.md) · [Русский](../ru/troubleshooting.md) · [Italiano](../it/troubleshooting.md) · **Tiếng Việt** · [Bahasa Indonesia](../id/troubleshooting.md) · [Türkçe](../tr/troubleshooting.md)

Trang này giúp bạn tìm ra lý do Desktop Subsystem for Android™ (DSA) không hoạt động, đọc nhật ký
của Android và gửi báo cáo sự cố. Trang này dành cho Windows, và cho Linux ở những chỗ có ghi chú.

## Kiểm tra trạng thái trước

```
dsa status          # trạng thái Android, sự cố nếu có, và bản cập nhật đang chờ nếu có
dsa start --wait    # khởi động Android và chờ đến khi khởi động xong
```

Trang **General** (chung) của DSA Settings hiển thị cùng trạng thái đó.

| `dsa status` | DSA Settings | Ý nghĩa |
|---|---|---|
| `Active` | **Running** | Android đã khởi động xong và agent của DSA bên trong đã kết nối |
| `Dozing` | **Dozing (idle)** | Android đang nhàn rỗi. Xem [Bộ nhớ](memory.md) |
| `Frozen` | **Frozen** | Android bị đóng băng cho đến khi bạn mở một cửa sổ của nó |
| `Stalled` | **Not ready** | Android đang chạy nhưng đã mất quá nhiều thời gian. Dòng tiếp theo cho biết lý do và việc cần làm |

## Các sự cố thường gặp

| Sự cố | Việc cần làm |
|---|---|
| Trình cài đặt báo Nền tảng Máy ảo đang tắt | Bật tính năng Windows đó, khởi động lại và chạy lại trình cài đặt. Xem [Bắt đầu](getting-started.md) |
| Không có cửa sổ ứng dụng nào mở | Đăng xuất rồi đăng nhập lại. Trình quản lý cửa sổ khởi động khi đăng nhập |
| Một ứng dụng mở với kích thước không đúng | `dsa app-window <package> reset` |
| Android không khởi động sau khi cập nhật ảnh | Không cần làm gì. DSA tự quay lại ảnh trước đó, và `dsa status` cho biết điều đó |
| Android không có mạng | Mở trang **Network** (mạng) của DSA Settings, hoặc chạy `dsa network`. Nó hiển thị chế độ, địa chỉ, máy chủ DNS và sự cố nếu có |
| Android bị hỏng và không cách nào khác giúp được | Đặt lại dữ liệu Android (bên dưới) |

### Đặt lại dữ liệu Android

Đặt lại sẽ xóa mọi ứng dụng bạn đã cài, tài khoản của bạn và toàn bộ dữ liệu ứng dụng. Sau đó
Android khởi động như lúc mới cài. Ảnh Android và cài đặt của DSA vẫn được giữ.

- Trong DSA Settings: trang **Advanced** (nâng cao) > **Reset Android's data** > **Reset**.
- Trên dòng lệnh: `dsa reset-data` cho biết những gì sẽ mất. `dsa reset-data --yes` xóa chúng.

## Đọc nhật ký Android

Nhật ký Android nằm trong bộ nhớ bên trong Android và không bao giờ được ghi ra tệp. Nó chỉ đến
PC của bạn khi có thứ gì đó đọc nó.

- Trong DSA Settings: trang **Logs** (nhật ký). Bạn có thể lọc theo thẻ hoặc văn bản, và tạm
  dừng.
- Trên dòng lệnh:

  ```
  dsa logcat                        # theo dõi nhật ký; Ctrl+C để kết thúc
  dsa logcat -d -b all *:W          # nội dung nhật ký hiện có, từ mức cảnh báo trở lên
  dsa logcat ActivityManager:I *:S  # chỉ một thẻ
  ```

## Gửi báo cáo sự cố

1. Lưu thông tin chẩn đoán. Trên trang **Logs**, chọn **Save diagnostics** (lưu chẩn đoán), hoặc
   chạy:

   ```
   dsa logs collect -o diag.zip
   ```

   Việc này mất một phút hoặc lâu hơn.
2. Đính kèm tệp zip vào một
   [issue](https://github.com/thisisthepy/desktop-subsystem-for-android/issues).

Tệp lưu trữ chứa toàn bộ nhật ký của Android, các bản ghi sự cố (tombstones, ANR traces, dropbox,
pstore), bản tóm tắt các dịch vụ hệ thống của Android, và các nhật ký DSA trên PC này liên quan
đến bạn, không kèm các dòng của người dùng khác. `summary.txt` trong tệp lưu trữ cho biết những gì
không thu thập được.

## Nhật ký nằm ở đâu trên Windows

| Thư mục | Tệp |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log` (trình quản lý cửa sổ), `dsa-app-window.log` (cửa sổ ứng dụng) |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log` (dịch vụ), `android-<SID>.log` (console Android của từng người dùng, tối đa 2 MiB mỗi lần khởi động), `setup-<version>.log` (tự cập nhật) |

Để mở các thư mục này, hãy dùng **Open my log folder** và **Open the service's log folder
(administrators)** trên trang **Advanced** của DSA Settings.

## Nhật ký nằm ở đâu trên Linux (chưa phát hành)

Các dịch vụ ghi vào systemd journal:

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## Mọi lệnh

`dsa` không có đối số sẽ in ra mọi lệnh. Thêm `--json` để có đầu ra mà chương trình đọc được.

---

Android là nhãn hiệu của Google LLC. Windows là nhãn hiệu của nhóm công ty Microsoft. DSA không
liên kết với Google hay Microsoft, và không được các công ty này xác nhận hoặc tài trợ.
