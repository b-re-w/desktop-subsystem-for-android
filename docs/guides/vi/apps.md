# Ứng dụng và cửa sổ

[English](../apps.md) · [한국어](../ko/apps.md) · [日本語](../ja/apps.md) · [简体中文](../zh-CN/apps.md) · [繁體中文](../zh-TW/apps.md) · [Español](../es/apps.md) · [Français](../fr/apps.md) · [Deutsch](../de/apps.md) · [Português (Brasil)](../pt-BR/apps.md) · [Русский](../ru/apps.md) · [Italiano](../it/apps.md) · **Tiếng Việt** · [Bahasa Indonesia](../id/apps.md) · [Türkçe](../tr/apps.md)

Mỗi tác vụ (task) của ứng dụng Android mở thành một cửa sổ desktop riêng, với nút riêng trên
thanh tác vụ. Không có khung giả lập bao quanh một màn hình điện thoại. Trang này mô tả Desktop
Subsystem for Android™ (DSA) trên Windows.

## Mở một ứng dụng

- **Menu Bắt đầu.** Mỗi ứng dụng đã cài có một mục trong menu Bắt đầu với tên và biểu tượng
  riêng, nên tìm kiếm Windows tìm thấy nó. Mở mục đó sẽ khởi động Android trước nếu Android chưa
  chạy.
- **DSA Settings.** Trên trang **Apps** (ứng dụng), chọn **Open** (mở) bên cạnh ứng dụng.
- **Dòng lệnh.** `dsa apps list` liệt kê các ứng dụng đã cài cùng gói của chúng.
  `dsa launch <package>` mở một ứng dụng, ví dụ `dsa launch com.android.settings`.

Khi mục menu Bắt đầu của một chương trình khác đã có tên của ứng dụng, DSA thêm tên gói vào tên,
chẳng hạn `Clock (com.example.clock)`. DSA chỉ thay đổi hoặc xóa những mục do nó tạo ra.

## Lối tắt trên desktop và danh sách nhảy

Biểu tượng ứng dụng hoạt động theo cách của ứng dụng Windows, không theo cách của màn hình chính
Android.

- **Lối tắt trên desktop.** Trên trang **Apps** của DSA Settings, chọn **Create desktop shortcut**
  (tạo lối tắt trên desktop) bên cạnh ứng dụng. Bạn cũng có thể nhấp chuột phải vào nút của ứng
  dụng trên thanh tác vụ và chọn **Create desktop shortcut**. Từ dòng lệnh:
  `dsa apps desktop-shortcut <package>`. Lối tắt mở ứng dụng giống như mục menu Bắt đầu của nó.
- **Lối tắt của ứng dụng.** Các lối tắt mà một ứng dụng Android cung cấp, chẳng hạn cuộc trò
  chuyện mới của ứng dụng nhắn tin, hiện ra khi bạn nhấp chuột phải vào nút của ứng dụng trên
  thanh tác vụ hoặc mục menu Bắt đầu của nó. Chọn một lối tắt sẽ mở màn hình đó của ứng dụng
  trong một cửa sổ, khởi động Android trước nếu nó chưa chạy. Khi ứng dụng thay đổi lối tắt,
  danh sách cũng thay đổi theo. Từ dòng lệnh: `dsa apps shortcut <package> <id>`.
  `dsa apps info <package> --json` liệt kê các ID.
- **Lối tắt do ứng dụng yêu cầu.** Khi một ứng dụng yêu cầu thêm một lối tắt của nó vào desktop,
  DSA hỏi bạn trước và chỉ thêm nếu bạn đồng ý.

Khi một ứng dụng đổi tên, ví dụ sau khi cập nhật, mục menu Bắt đầu, các lối tắt trên desktop và
ghim trên thanh tác vụ của nó theo tên mới và vẫn tiếp tục hoạt động. Lối tắt trên desktop mà bạn
đã xóa sẽ không được tạo lại. Gỡ cài đặt một ứng dụng cũng xóa các lối tắt trên desktop của nó.

## Làm việc với cửa sổ ứng dụng

- Mỗi tác vụ có cửa sổ, nút trên thanh tác vụ và mục Alt+Tab riêng, với tên và biểu tượng của
  ứng dụng. Trình quản lý Tác vụ liệt kê mỗi ứng dụng dưới tên riêng của nó.
- Thanh tiêu đề là một phần của ứng dụng: màu của ứng dụng kéo dài đến tận mép trên cửa sổ, và
  các nút tiêu đề của Windows 11 nằm phía trên. Biểu tượng và tên ứng dụng nằm bên trái. Khi ứng
  dụng có thể quay lại, biểu tượng chuyển thành mũi tên quay lại. Chọn nó để quay lại.
- Snap layouts (di chuột lên nút phóng to), kéo thanh tiêu đề, nhấp đúp vào nó để phóng to và
  thay đổi kích thước trực tiếp đều hoạt động. Thay đổi kích thước không khởi động lại ứng dụng.
  Cửa sổ phóng to lấp đầy vùng làm việc của màn hình và vẫn để lộ thanh tác vụ.
- Không có thanh điều hướng Android. Thanh dưới cùng của ứng dụng nằm sát mép dưới cửa sổ, không
  có dải trống bên dưới.
- Chuột (có hover), cảm ứng, bút (lực nhấn, tẩy và nút trên thân bút) và bàn phím đều hoạt động.
  Văn bản đến từ bộ gõ Windows của bạn, kể cả tiếng Hàn và các ngôn ngữ khác. Phần đang soạn hiện
  ngay trong ô nhập của Android, và cửa sổ gợi ý nằm cạnh con trỏ. Không có ứng dụng bàn phím
  Android.
- Kéo tệp từ File Explorer thả vào cửa sổ ứng dụng, hoặc kéo tệp từ ứng dụng ra File Explorer.
- Các trình đọc màn hình như Trình tường thuật (Narrator) và NVDA đọc ứng dụng qua UI Automation.
- Đóng một cửa sổ sẽ yêu cầu Android kết thúc tác vụ đó.

## Menu cửa sổ

Nhấp chuột phải vào thanh tiêu đề, chọn biểu tượng của ứng dụng, hoặc nhấn Alt+Space. Sau các mục
thông thường của Windows, menu có:

- **Full screen** (toàn màn hình): lấp đầy toàn bộ màn hình, giống F11. Chọn lại mục này, hoặc
  nhấn F11, để thoát.
- **New window** (cửa sổ mới): mở thêm một cửa sổ của ứng dụng, cho các ứng dụng hỗ trợ nhiều cửa
  sổ.
- **Screenshot** (chụp màn hình): sao chép màn hình của ứng dụng, không có thanh tiêu đề, vào bộ
  nhớ tạm và lưu vào thư mục Screenshots trong thư mục Ảnh của bạn.
- **Aspect ratio** (tỷ lệ khung hình): mở cài đặt tỷ lệ khung hình của Android cho ứng dụng, dành
  cho các ứng dụng trông bị kéo giãn trong cửa sổ lớn.
- **Open by default** (mở theo mặc định): chọn xem các liên kết mà ứng dụng hỗ trợ sẽ mở trong ứng
  dụng hay trong trình duyệt của bạn.
- **App info** (thông tin ứng dụng): mở mục của ứng dụng trên trang **Apps** của DSA Settings.

## Toàn màn hình

Khi một ứng dụng yêu cầu toàn màn hình, cửa sổ của nó hiển thị theo một trong hai cách:

- **Trên toàn bộ màn hình**: cửa sổ lấp đầy màn hình và thanh tiêu đề ẩn đi. Đưa con trỏ chạm mép
  trên màn hình để hiện thanh này, với nút thu nhỏ và đóng. Nút ở giữa thoát chế độ toàn màn
  hình.
- **Bên trong cửa sổ**: cửa sổ giữ nguyên kích thước và thanh tiêu đề, còn ứng dụng lấp đầy cửa
  sổ.

Ứng dụng yêu cầu theo hai kiểu, và mỗi kiểu có mặc định riêng:

| Kiểu yêu cầu | Ví dụ | Mặc định |
|---|---|---|
| Yêu cầu toàn màn hình rõ ràng | Nút toàn màn hình của trình phát video | Trên toàn bộ màn hình |
| Chỉ ẩn các thanh | Một luồng video ngắn | Bên trong cửa sổ |

Đổi các mặc định trong **App full screen** và **Apps that hide their bars** trên trang **Display &
GPU** (màn hình và GPU) của DSA Settings, hoặc cho một ứng dụng trong **Full screen** và **Bars
hidden** trên trang **Apps**. Lựa chọn riêng của ứng dụng được ưu tiên. F11 và **Full screen**
trong menu cửa sổ luôn bật hoặc tắt toàn màn hình trên toàn bộ màn hình, bất kể cài đặt là gì.

Từ dòng lệnh:

```
dsa settings set app-fullscreen screen|window           # mặc định cho yêu cầu rõ ràng
dsa settings set app-immersive screen|window            # mặc định cho việc ẩn thanh
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## Đặt kích thước cửa sổ của ứng dụng

Trên trang **Apps** của DSA Settings:

- Tắt **Resizable** (có thể đổi kích thước) để giữ các cửa sổ của ứng dụng ở kích thước lúc chúng
  mở.
- Chọn kích thước mà cửa sổ của ứng dụng mở ra: kích thước Android chọn, hoặc kích thước điện
  thoại hay máy tính bảng theo chiều dọc hoặc chiều ngang.

Thay đổi áp dụng cho cửa sổ tiếp theo mà ứng dụng mở. Từ dòng lệnh:

```
dsa app-window <package> --fixed-size on        # không thể đổi kích thước hay phóng to
dsa app-window <package> --start-size 1280x800  # mở ở kích thước này; auto để Android chọn
dsa app-window <package> --dpi-scaling system   # cách co giãn trên màn hình khác; xem bên dưới
dsa app-window <package> reset                  # quay về mặc định
```

## Cho ứng dụng bắt đầu bên dưới thanh tiêu đề

Ứng dụng vẽ đến tận mép trên của cửa sổ, bên dưới thanh tiêu đề. Vì vậy một số ứng dụng có
thanh trên cùng bị ép lại hoặc bị thanh tiêu đề che mất một phần. Với ứng dụng như vậy, hãy bật
**Start below the title bar** (bắt đầu bên dưới thanh tiêu đề) trên trang **Apps** của DSA Settings. Khi đó nội dung của
ứng dụng bắt đầu bên dưới thanh tiêu đề, và thanh tiêu đề lấy màu mà ứng dụng hiển thị ở đó.
Mặc định tùy chọn này tắt, và áp dụng từ lần mở cửa sổ ứng dụng tiếp theo. Từ dòng lệnh:

```
dsa app-window <package> --below-title-bar on   # off quay về mặc định
```

## Chuyển ứng dụng sang màn hình có tỷ lệ khác

Ứng dụng mở ở tỷ lệ của màn hình nơi cửa sổ của nó mở, ngay từ khung hình đầu tiên, nên nó không
bị tạo lại ngay sau khi mở. Khi bạn chuyển một cửa sổ sang màn hình có tỷ lệ khác, ứng dụng giữ
nguyên kích thước trên màn hình. Mỗi ứng dụng làm việc này theo một trong các cách sau, do bạn
chọn trong **Scaling on another monitor** (co giãn trên màn hình khác) trên trang **Apps**, hoặc
bằng `dsa app-window <package> --dpi-scaling`:

- **Automatic** (`auto`, mặc định): **System** cho các ứng dụng có trong danh sách tích hợp sẵn
  của DSA gồm những ứng dụng bị lỗi khi tỷ lệ thay đổi lúc đang chạy, và **App** cho mọi ứng dụng
  khác. Danh sách này có YouTube: nó giữ bố cục theo tỷ lệ lúc khởi động, nên sau khi chuyển màn
  hình, các thẻ dưới cùng và trình phát thu nhỏ của nó sẽ bị lỗi cho đến khi khởi động lại. Cài
  đặt hiển thị mặc định của ứng dụng như vậy là **Scaling: system (recommended for this app)**.
  Lựa chọn riêng của bạn cho ứng dụng được ưu tiên.
- **App** (`app`): khi bạn kéo cửa sổ, DSA co giãn nó theo màn hình mới. Khi bạn thả ra, ứng dụng
  vẽ lại ở tỷ lệ của màn hình mới, và chữ vẫn sắc nét: Android tạo lại màn hình của ứng dụng một
  lần và khôi phục trạng thái (vị trí cuộn, văn bản đã gõ), nên ứng dụng vẽ lại trong chốc lát
  sau mỗi lần chuyển, giống như một chương trình Windows với chế độ co giãn "Application". Điều
  này đúng cả với ứng dụng tự nhận là xử lý được thay đổi tỷ lệ, vì một số ứng dụng đó chỉ vẽ lại
  một phần màn hình ở tỷ lệ mới.
- **System** (`system`): ứng dụng giữ nguyên tỷ lệ, và DSA co giãn cửa sổ của nó theo màn hình
  mới. Ứng dụng không bao giờ bị tạo lại. Khi phóng to, chữ có thể trông hơi mờ.
- **System (forced)** (`system-forced`): ứng dụng nhận tỷ lệ mới mà không khởi động lại, kể cả khi
  nó không nói là xử lý được. Chưa có: nó cần một thay đổi trong Android mà phiên bản này chưa
  có, nên cho đến lúc đó nó hoạt động như **System**.

Chữ trong ứng dụng theo cỡ chữ của Windows (Cài đặt > Trợ năng > Cỡ chữ).

## Thông báo

Thông báo Android hiện thành thông báo Windows, với tên và biểu tượng của ứng dụng.

- Chọn một thông báo để mở ứng dụng.
- Khi bạn đóng thông báo, nó cũng bị đóng trong Android, trừ khi ứng dụng giữ nó hiển thị.
- Thông báo vẫn tiếp tục đến khi Android ở trạng thái Doze. Xem [Bộ nhớ](memory.md).

## Ứng dụng chạy nền

Một ứng dụng tiếp tục chạy sau khi cửa sổ đóng, chẳng hạn trình phát nhạc, hiện thành biểu tượng
trong vùng thông báo. Chú thích (tooltip) của biểu tượng là thông báo của ứng dụng.

- Chọn biểu tượng để mở ứng dụng.
- Nhấp chuột phải vào biểu tượng để có **Open**, các thao tác riêng của thông báo, và **Stop**
  (dừng).
- **Stop** hỏi trước. Đánh dấu **Do not ask again** (không hỏi lại) để bỏ qua câu hỏi từ đó về
  sau. Để được hỏi lại, bật **Ask before stopping an app from the notification area** trên trang
  **Running in the background** (chạy nền) của DSA Settings.
- Ứng dụng phát đa phương tiện cũng hoạt động với phím đa phương tiện và điều khiển đa phương tiện
  của Windows.

Trang **Running in the background** của DSA Settings liệt kê các ứng dụng này, với **Open** và
**Stop** cho từng ứng dụng. Từ dòng lệnh: `dsa background`, `dsa background open <package>` và
`dsa background stop <package>`.

## Widget trên desktop

Một widget Android có thể nằm trên desktop của bạn trong một cửa sổ riêng. Nó luôn nằm dưới các
cửa sổ khác, vẫn còn khi bạn hiện desktop, và không có trên thanh tác vụ hay trong Alt+Tab.

1. Mở trang **Widgets** của DSA Settings.
2. Trong **Widgets your apps offer** (widget mà ứng dụng của bạn cung cấp), chọn **Add to desktop**
   (thêm vào desktop) bên cạnh một widget. Nếu ứng dụng yêu cầu bạn thiết lập widget, phần thiết
   lập sẽ mở trong một cửa sổ.
3. Để di chuyển widget, nhấn vào bất kỳ đâu trên widget và giữ yên nửa giây. Widget nhấc lên, với
   một đường viền và một tay cầm ở mỗi cạnh có thể đổi kích thước. Kéo widget để di chuyển, hoặc
   kéo một tay cầm để đổi kích thước. Một lần kéo nhanh vẫn thuộc về widget, nên danh sách của nó
   vẫn cuộn được. Bạn cũng có thể kéo một cạnh của widget để đổi kích thước mà không cần nhấc nó
   lên trước. Chỉ những cạnh có thể đổi kích thước mới làm được điều này. DSA ghi nhớ vị trí và
   kích thước bạn đặt.
4. Nhấp chuột phải vào widget, hoặc thả ra sau khi nhấn giữ, để có **Widget settings** (cài đặt
   widget, khi widget có thể thiết lập lại), **Resize** (đổi kích thước) để chọn một trong các kích
   thước nó cho phép, và **Remove** (gỡ). Phím menu và Shift+F10 cũng mở menu này. Các phím mũi
   tên, Enter và Esc hoạt động trong menu này, và một chữ cái chọn một mục: R cho **Resize** và M
   cho **Remove**.

Từ dòng lệnh: `dsa widgets list`, `dsa widgets add <provider>` và `dsa widgets remove <id>`.

## Liên kết và bộ nhớ tạm

- Liên kết web bạn chạm vào trong một ứng dụng Android sẽ mở trong trình duyệt mặc định của bạn.
  Liên kết mà một ứng dụng đã cài xác minh là của mình sẽ mở trong ứng dụng đó.
- Để mở mọi liên kết web trong ứng dụng Android, hãy bật **Open web links in Android apps** trên
  trang **Advanced** (nâng cao) của DSA Settings (`dsa settings set links-in-android on`).
- Văn bản bạn sao chép được chia sẻ hai chiều giữa Windows và Android. Để ngừng việc này, tắt
  **Share the clipboard with Android** trên trang **Advanced**
  (`dsa settings set clipboard off`).

## Google Play và video được bảo vệ

DSA không đi kèm phần mềm nào của Google. Nếu bạn muốn, DSA tải nó từ nguồn cho bạn, sau khi cho
bạn thấy nguồn và điều khoản. Cả hai đều có hiệu lực từ lần khởi động Android tiếp theo, và cả
hai đều có thể gỡ bỏ lại.

- **Google Play.** Trên trang **Advanced** của DSA Settings, chọn **Install** trong **Google
  Play**, hoặc chạy `dsa gapps install`. DSA tải gói công khai LiteGapps Core (Google Play
  services, Play Store và đồng bộ tài khoản) cho phiên bản Android của bạn. Android này không được
  Google chứng nhận, nên Google có thể từ chối cho bạn đăng nhập cho đến khi bạn đăng ký thiết bị:
  **Register this device with Google** hiển thị Android ID và mở trang đăng ký. `dsa gapps status`
  hiển thị trạng thái, và `dsa gapps remove` gỡ các ứng dụng của Google, kể cả khi Android đang
  dừng.
- **Widevine.** Ứng dụng phát trực tuyến cần mô-đun Widevine của Google để phát video được bảo vệ.
  Trên trang **Advanced**, chọn **Get** trong **Protected content**, hoặc chạy
  `dsa drm widevine install`. DSA tải ảnh trình giả lập Android của Google cho phiên bản Android
  của bạn (khoảng 2 GB), chỉ giữ lại mô-đun Widevine, không chỉnh sửa, và xóa phần còn lại.
  Widevine ở đây có cấp bảo mật L3, nên các dịch vụ phát trực tuyến gửi tối đa độ nét tiêu chuẩn.
  Để xem độ nét cao, hãy dùng trình duyệt hoặc ứng dụng Windows. `dsa drm widevine remove` gỡ nó.

## Ứng dụng chỉ dành cho ARM

Trên PC x64, ứng dụng chỉ có mã cho bộ xử lý ARM chạy qua một trình dịch ARM. Android trên PC
x64 đi kèm một trình dịch, trình dịch mã nguồn mở Digitalis, và dùng nó ngay từ lần khởi động đầu
tiên. Bạn có thể dùng trình dịch khác:

- **Trình dịch ARM của Google.** Trên trang **Advanced**, chọn **Get** bên cạnh **Google's ARM
  translator**, hoặc chạy `dsa arm-translation fetch google`. Trước tiên DSA cho biết nó đến từ
  đâu và điều khoản của Google
  ([Android SDK License](https://developer.android.com/studio/terms)), và không tải gì cho đến khi
  bạn chấp nhận. Sau đó PC của bạn tải ảnh trình giả lập Android mà Google cung cấp cho phiên bản
  Android của bạn (khoảng 2 GB), chỉ giữ lại trình dịch và xóa phần còn lại. DSA không đi kèm hay
  phân phối lại trình dịch của Google.
- **Một gói bạn tự xây dựng hoặc lấy từ nơi khác**, chẳng hạn gói được tạo từ trình dịch mã nguồn
  mở Digitalis: `dsa arm-translation install <folder>`.

Sau đó khởi động lại Android: dừng nó trên trang **General** (chung) của DSA Settings, rồi mở một
ứng dụng.

Để quay lại trình dịch đi kèm Android, chọn **Use** bên cạnh **Use the default translator** trên
trang **Advanced**, hoặc chạy `dsa arm-translation use-default`. Lệnh này gỡ gói bạn đã cài, từ
lần khởi động Android tiếp theo.

`dsa arm-translation status` cho biết Android đang dùng trình dịch nào. Công tắc **Run apps built
only for ARM** trên trang **Advanced** bật hoặc tắt việc dịch mà không gỡ gì. Với ứng dụng có cả
mã ARM và x64, **Run as ARM** trên trang **Apps** (`dsa apps run-as-arm <package> on`) chạy mã ARM
của nó. Chỉ ứng dụng ARM 64-bit (`arm64-v8a`) chạy được. Trên PC ARM64, Android tự chạy ứng dụng
ARM và không cần trình dịch.

## Quyền root

Một số ứng dụng, chẳng hạn công cụ sao lưu, cần quyền root. DSA cấp quyền này theo cách `sudo` làm
trên PC Linux: chỉ khi một quản trị viên của PC chấp thuận, từng yêu cầu một. Tính năng này tắt
theo mặc định, và ứng dụng không được phép hoàn toàn không thấy `su`.

1. Quản trị viên bật **Let Android apps ask for root** trong **Root access** (quyền root) trên
   trang **Advanced** của DSA Settings (`dsa settings set root-elevation on`). Thiết lập có hiệu lực
   từ lần khởi động Android tiếp theo.
2. Trên trang **Apps**, đặt **Root access** cho ứng dụng:
   - **Root: not allowed** (mặc định): ứng dụng không có `su`.
   - **Root: ask each time**: mỗi lần ứng dụng dùng `su`, Windows hiện lời nhắc quản trị viên
     (UAC) dưới tên và biểu tượng của chính ứng dụng. Người dùng tiêu chuẩn nhập mật khẩu của
     quản trị viên. Một lần chấp thuận kéo dài 15 phút cho ứng dụng đó.
   - **Root: remember (no asking)**: ứng dụng dùng `su` mà không hỏi. Chọn mục này cần quản trị
     viên chấp thuận.

   Từ dòng lệnh: `dsa apps root <package> off|ask|remember`.

Khi bật chế độ nhà phát triển, `adb shell` cũng có thể dùng `su`. Nó có hàng riêng trên trang
**Apps**, **adb shell**, và mặc định hỏi mỗi lần (`dsa apps root adb ask`). Lời nhắc của nó có tên
**adb shell** và hiển thị lệnh.

Khi chính một phần của Android yêu cầu, lời nhắc nêu tên phần đó: **Android system**, **Android
phone services**, **Android Bluetooth**, **Android NFC** hoặc **Android system service**. Các ứng
dụng dùng chung một ID người dùng được chấp thuận như một nhóm, và lời nhắc liệt kê tất cả.

Mọi yêu cầu đều được lưu trong nhật ký dịch vụ cùng với ứng dụng, thời gian, lệnh và việc nó có
được chấp thuận hay không. Tắt tính năng này sẽ ẩn `su` khỏi mọi ứng dụng ngay lập tức, không cần
khởi động lại.

---

Android là nhãn hiệu của Google LLC. Windows là nhãn hiệu của nhóm công ty Microsoft. DSA không
liên kết với Google hay Microsoft, và không được các công ty này xác nhận hoặc tài trợ.
