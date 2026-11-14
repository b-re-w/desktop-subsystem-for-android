# Chế độ nhà phát triển và adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · [日本語](../ja/developer-mode.md) · [简体中文](../zh-CN/developer-mode.md) · [繁體中文](../zh-TW/developer-mode.md) · [Español](../es/developer-mode.md) · [Français](../fr/developer-mode.md) · [Deutsch](../de/developer-mode.md) · [Português (Brasil)](../pt-BR/developer-mode.md) · [Русский](../ru/developer-mode.md) · [Italiano](../it/developer-mode.md) · **Tiếng Việt** · [Bahasa Indonesia](../id/developer-mode.md) · [Türkçe](../tr/developer-mode.md)

Chế độ nhà phát triển cho phép Android Debug Bridge (adb) trên PC của bạn kết nối với Android
trong Desktop Subsystem for Android™ (DSA). Chế độ này tắt theo mặc định.

adb không phải là một phần của DSA. Hãy lấy nó từ Android SDK Platform-Tools.

## Kết nối adb

1. Mở trang **Network** (mạng) của DSA Settings và bật **Developer mode** (chế độ nhà phát triển).
   Hoặc chạy `dsa settings set developer-mode on`.
2. Khởi động Android, nếu nó chưa chạy: `dsa start --wait`.
3. Kết nối:

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   Android không yêu cầu bạn cho phép kết nối. Cổng này chỉ trả lời trên PC này, nên các máy tính
   khác không kết nối được.
4. Khi xong việc, hãy tắt lại **Developer mode**, hoặc chạy
   `dsa settings set developer-mode off`.

**Connect adb** (kết nối adb) trên trang **Network** hiển thị chính xác lệnh cần dùng, và
`dsa settings` in lệnh đó trên dòng `Developer mode`.

## Dùng cổng khác

Cổng mặc định là 58526. Để thay đổi, hãy sửa **Port** (cổng) trong **Connect adb**, hoặc chạy:

```
dsa settings set adb-port 5555
```

## Những gì chế độ nhà phát triển không thay đổi

DSA không bao giờ dùng adb cho công việc của chính nó. Cửa sổ, nhập liệu và cài đặt dùng kênh
điều khiển riêng của DSA, nên mọi thứ hoạt động như nhau khi chế độ nhà phát triển tắt.

Để đọc nhật ký Android, bạn không cần adb. Xem [Khắc phục sự cố](troubleshooting.md).

---

Android là nhãn hiệu của Google LLC. DSA không liên kết với Google, và không được Google xác nhận
hoặc tài trợ.
