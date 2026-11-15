# Cài đặt

[English](../settings.md) · [한국어](../ko/settings.md) · [日本語](../ja/settings.md) · [简体中文](../zh-CN/settings.md) · [繁體中文](../zh-TW/settings.md) · [Español](../es/settings.md) · [Français](../fr/settings.md) · [Deutsch](../de/settings.md) · [Português (Brasil)](../pt-BR/settings.md) · [Русский](../ru/settings.md) · [Italiano](../it/settings.md) · **Tiếng Việt** · [Bahasa Indonesia](../id/settings.md) · [Türkçe](../tr/settings.md)

**DSA Settings** là ứng dụng cài đặt của Desktop Subsystem for Android™ (DSA) trên Windows. Mở nó
từ menu Bắt đầu. Ứng dụng hiển thị bằng tiếng Anh hoặc tiếng Hàn, theo ngôn ngữ hiển thị của
Windows, và theo chủ đề Windows của bạn. Ứng dụng dùng được với bàn phím và với trình đọc màn hình.

Mọi thay đổi mà DSA Settings thực hiện đều đi qua lệnh `dsa`, nên dòng lệnh thấy cùng các giá trị:

```
dsa settings                        # mọi cài đặt và giá trị của chúng
dsa settings set <name> <value>     # đổi một cài đặt; công tắc nhận on hoặc off
```

Một số cài đặt áp dụng cho mọi người dùng của PC, nên chỉ quản trị viên mới có thể thay đổi chúng.
DSA Settings đánh dấu chúng bằng biểu tượng tấm khiên và yêu cầu quản trị viên chấp thuận khi bạn
thay đổi. Bên dưới, chúng được đánh dấu "(quản trị viên)".

## Các trang

| Trang | Những gì bạn tìm thấy ở đó | Dòng lệnh |
|---|---|---|
| **General** (chung) | Trạng thái Android, **Start**, **Stop**, **Freeze Android now**, sự cố gần nhất, **Open my log folder**, **Refresh** | `dsa status`, `dsa start`, `dsa stop`, `dsa freeze` |
| **Memory** (bộ nhớ) | **Memory limit**, kích thước VM của Android, mức dùng của từng phần | `memory-limit`, `dsa memory` |
| **Apps** (ứng dụng) | Các ứng dụng đã cài: **Open**, **App info**, **Uninstall**, **Create desktop shortcut**. Cho từng ứng dụng: **Resizable**, kích thước cửa sổ khi mở, **Scaling**, **Full screen request**, **When only the bars hide**, **Run as ARM**, **Root access**. Một hàng cho quyền root của **adb shell** | `dsa apps`, `dsa app-window`, `dsa apps run-as-arm`, `dsa apps root` |
| **Running in the background** (chạy nền) | Ứng dụng chạy không có cửa sổ: **Open**, **Stop**; có hỏi trước khi dừng hay không | `dsa background`, `stop-without-asking` |
| **Widgets** | Widget trên desktop của bạn, và **Add to desktop** | `dsa widgets` |
| **Display & GPU** (màn hình và GPU) | **Render GPU**, **GPU of the primary monitor**, **App full screen**, **Apps that hide their bars**, **Turn off Windows touch gestures in DSA app windows** (chỉ Windows 11, bật theo mặc định), **Restart the window manager** | `render-gpu`, `dsa gpus`, `app-fullscreen`, `app-immersive`, `touch-gestures-off` |
| **Device identity** (danh tính thiết bị) | Trước tiên là máy tính này, chỉ đọc: tên, nhà sản xuất và kiểu máy, và **Device specifications** với **Copy**. Sau đó là nhà sản xuất, kiểu máy và dấu vân tay bản dựng mà Android báo cho ứng dụng | `dsa settings device`, `device.<field>` |
| **Network** (mạng) | Cách Android kết nối mạng, địa chỉ của nó, ai trả lời tên miền cho nó (DNS) và cổng mạng, **Advanced networking**, **Use this PC's added root certificates**, **Developer mode** (adb), **Connect adb automatically** | `dsa network`, `advanced-networking`, `host-certificates`, `developer-mode`, `adb-port`, `adb-auto-connect` |
| **Updates** (cập nhật) | **Android**: phiên bản và bản dựng đang chạy, mức bản vá bảo mật, phiên bản mà nó theo dõi, bản cập nhật Android kèm tiến trình và **Check now** (quản trị viên), **Restart Android into updates by itself**, **Update the Android image automatically** (quản trị viên), **Version to use** (quản trị viên). **DSA**: **Update DSA automatically** (quản trị viên), **Get pre-release versions** (quản trị viên), **Install the newest version of DSA now** | `dsa android version`, `dsa android use`, `image-auto-restart`, `image-updates`, `program-updates`, `prerelease-updates`, `dsa update` |
| **Advanced** (nâng cao) | Bộ nhớ tạm, tệp, liên kết web, đóng băng khi nhàn rỗi, sao lưu, đặt lại, ứng dụng ARM, Google Play, nội dung được bảo vệ, quyền root, theo dõi (quản trị viên), thư mục nhật ký | xem bên dưới |
| **Logs** (nhật ký) | Nhật ký trực tiếp của Android, **Save diagnostics** | `dsa logcat`, `dsa logs collect` |
| **About** (giới thiệu) | Phiên bản, giấy phép, mã nguồn | |

## Trang Advanced

| Cài đặt | Mặc định | Dòng lệnh |
|---|---|---|
| **Share the clipboard with Android** | Bật | `clipboard` |
| **Share Windows folders with Android** | Bật | `file-sharing` |
| **Shared folders** | Cả sáu | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`, `dsa share` |
| **Freeze Android when idle** | Tắt | `freeze` |
| **Open web links in Android apps** | Tắt | `links-in-android` |
| **Back up apps' data to this PC**, **Back up now**, **Restore** | Tắt | `dsa backup on`, `dsa backup now`, `dsa backup restore` |
| **Recovery key**, **Use a recovery key** | | `dsa backup key show`, `dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator** (không có trên ARM64) | | `dsa arm-translation fetch google` |
| **Use the default translator** (không có trên ARM64) | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**, **Remove Google Play** | Chưa cài | `dsa gapps install`, `dsa gapps remove` |
| **Widevine** (trong **Protected content**) | Chưa cài | `dsa drm widevine install`, `dsa drm widevine remove` |
| **Let Android apps ask for root** (quản trị viên) | Tắt | `root-elevation` |
| **Input trace** (quản trị viên) | Tắt | `input-trace` |
| **Resize trace** (quản trị viên) | Tắt | `resize-trace` |

Sao lưu giữ một bản sao được mã hóa của dữ liệu ứng dụng trong thư mục người dùng của bạn, với một
khóa mà Windows giữ cho bạn. Để khôi phục trên PC khác, hoặc sau khi cài lại Windows, bạn cần khóa
khôi phục, vì vậy hãy ghi nó ra ở nơi khác ngoài PC. Google Play, Widevine và quyền root được mô tả
trong [Ứng dụng và cửa sổ](apps.md).

## Khi nào một thay đổi có hiệu lực

- **Render GPU**: khi trình quản lý cửa sổ khởi động lại, vào lần đăng nhập tiếp theo hoặc khi bạn
  chọn **Restart the window manager**.
- **Memory limit**, **Device identity**, **Advanced networking**, **Run apps built only for ARM**,
  Google Play, Widevine, việc bật quyền root, và phiên bản Android: vào lần khởi động Android tiếp
  theo. Để khởi động lại Android, hãy dừng nó trên trang **General**, rồi mở một ứng dụng.
- Cài đặt cửa sổ của một ứng dụng: từ cửa sổ tiếp theo mà ứng dụng mở.
- Mọi thứ khác, kể cả việc tắt quyền root: ngay lập tức.

## Cách Android tìm tên miền

Máy tính của bạn trả lời mọi tên miền mà ứng dụng Android tra cứu, bằng cùng trình phân giải mà
ứng dụng Windows dùng (trên Linux và macOS là trình phân giải của hệ thống). Vì vậy tệp hosts, quy
tắc tên miền của công ty bạn, máy chủ DNS riêng của VPN và bộ lọc DNS áp dụng cho ứng dụng Android
giống như cho máy tính. Android không tự hỏi một máy chủ DNS nào.

- Điều này vẫn đúng khi bật **Advanced networking**: máy tính của bạn ở cùng mạng và biết các tên
  miền của mạng đó, và VPN hay bộ lọc DNS trên máy vẫn được áp dụng.
- **Private DNS** của chính Android tắt theo mặc định, vì nó sẽ hỏi một máy chủ DNS mà bỏ qua máy
  tính của bạn. Nếu bạn chọn một máy chủ DNS riêng trong cài đặt của Android, Android sẽ dùng nó.
- Nếu DSA không trả lời được trong một lúc, Android hỏi các máy chủ DNS của mạng cho đến khi DSA
  trả lời được trở lại.
- Hàng **DNS** của trang **Network**, và `dsa network`, cho biết bên nào đang trả lời: trình phân
  giải của máy tính bạn, hay các máy chủ DNS của mạng, được liệt kê ở đó.

## Các giá trị dòng lệnh nhận

- `render-gpu`: `display` (GPU của màn hình chính, mặc định), `integrated`, `discrete`, hoặc một
  phần tên GPU như `dsa gpus` liệt kê.
- `memory-limit`: `auto` (mặc định: không có giới hạn cố định, tối đa bằng bộ nhớ của PC trừ phần
  Windows giữ lại), hoặc một kích thước như `4g` hay `3072` (MiB).
- `shared-folders`: `all`, `none`, hoặc các tên ngăn cách bằng dấu phẩy, chẳng hạn
  `Documents,Pictures`.
- `adb-port`: một cổng TCP từ 1 đến 65535. Mặc định là 58526.
- `app-fullscreen`: `screen` (mặc định) hoặc `window`, cho yêu cầu toàn màn hình rõ ràng của ứng
  dụng. `app-immersive`: `screen` hoặc `window` (mặc định), cho ứng dụng chỉ ẩn các thanh của
  mình. Xem [Toàn màn hình](apps.md#toàn-màn-hình).
- `touch-gestures-off`: `on` (mặc định) tắt cử chỉ chạm ba và bốn ngón của Windows 11 khi một cửa
  sổ ứng dụng DSA ở phía trước, để ứng dụng nhận được đến mười ngón, và trả lại cài đặt của bạn
  khi một cửa sổ khác lên phía trước. Nếu Windows từ chối, cài đặt này không dùng được và ứng dụng
  cài đặt cho biết lý do. Bạn vẫn có thể tắt các cử chỉ cho mọi ứng dụng trong Cài đặt Windows, tại
  Bluetooth và thiết bị, Cảm ứng.
- `host-certificates`: `on` (mặc định) cho phép ứng dụng Android tin cậy các chứng chỉ gốc được
  thêm vào PC này, chẳng hạn chứng chỉ của công ty bạn hoặc của một chương trình bảo mật.
- `adb-auto-connect`: `on` (mặc định) hiển thị Android trong `adb devices` và Android Studio khi
  chế độ nhà phát triển đang bật, không cần `adb connect`.
- `image-auto-restart`: `on` (mặc định) khởi động lại Android vào bản cập nhật ảnh đã tải ngay lập
  tức khi không có cửa sổ ứng dụng nào đang mở. Nếu không, DSA hỏi trước, hoặc chờ cho đến khi bạn
  đóng chúng.
- `prerelease-updates`: `on` nhận cả bản phát hành trước lẫn bản ổn định, `off` chỉ nhận bản ổn
  định. Giá trị ban đầu theo trình cài đặt bạn đã dùng: trình cài đặt bản phát hành trước bật nó,
  trình cài đặt bản ổn định để tắt, và các lần nâng cấp giữ nguyên lựa chọn của bạn. Mọi bản phát
  hành trước 1.0 đều là bản phát hành trước. Tắt nó không bao giờ đưa DSA về phiên bản cũ hơn: DSA
  chờ một bản ổn định mới hơn bản đã cài.
- `root-elevation`: `on` cho phép những ứng dụng bạn cho phép trên trang **Apps** yêu cầu quyền
  root. Mức của từng ứng dụng được đặt bằng `dsa apps root <package> off|ask|remember`.
- `device.<field>`: `manufacturer`, `brand`, `model`, `device`, `name`, `characteristics`,
  `fingerprint`, hoặc một phần của dấu vân tay như `fingerprint.release`.
  `dsa settings reset device` đặt lại mọi giá trị.

---

Android là nhãn hiệu của Google LLC. Windows là nhãn hiệu của nhóm công ty Microsoft. DSA không
liên kết với Google hay Microsoft, và không được các công ty này xác nhận hoặc tài trợ.
