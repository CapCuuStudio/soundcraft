# ORVIA Desktop — hướng chuyển nền tảng Zrythm (đề xuất đã được người dùng đồng ý)

**Ngày:** 20/10/2026
**Sản phẩm:** ORVIA thuộc LUNOR MUSIC, **ứng dụng DAW Windows thực**.
**Upstream nguồn tham khảo/ứng viên:** https://github.com/zrythm/zrythm (mã nguồn C++23, Qt/QML, JUCE)
**Nguồn xác minh:** README.md, CMakeLists.txt, TRADEMARKS.md, LICENSES/LicenseRef-ZrythmLicense.txt trên nhánh mặc định của Zrythm ngày 20/10/2026.

## Quyết định dự án

1. **Đích cuối là ORVIA Windows desktop**, không phải một DAW WebAssembly.
2. Cloudflare Pages chỉ là **trang theo dõi tạm thời**: mục tiêu, trạng thái, các thay đổi, link commit, ảnh chụp/video thật của bản native khi có. Không trình bày ảnh demo hoặc mô phỏng thành phần mềm đã chạy.
3. Giữ **SoundCraft/ORVIA web cũ** nguyên vẹn như phương án dự phòng, không tự merge PR cũ, không xóa repository, không động `main` hoặc LUNOR production.
4. Chưa chuyển mã nguồn sang Zrythm thật cho đến khi có repository/fork Zrythm do chủ sở hữu kiểm soát, build Windows baseline và đánh giá đủ các yêu cầu về giấy phép.
5. Không cần CMD máy người dùng; không tự dùng GitHub Actions tính phí/quota hay cloud trả phí; yêu cầu xác nhận trước nếu bất kỳ dịch vụ mới có thể phát sinh tiền.

## Kiến trúc và thực tế đã xác minh

- Zrythm hiện dùng **C++23, Qt/QML, JUCE**, CMake trên Windows; **không cần chuyển sang WASM** trong hướng này.
- README upstream hướng dẫn **Visual Studio 2022 với Desktop development with C++ + Qt**, cấu hình CMake, build bằng Visual Studio; Qt phải đồng bộ compiler/config.
- CMake upstream yêu cầu Qt 6.11. Kiểm tra phiên bản thực trên runner trước khi chọn công cụ build.
- README v2 liệt kê Piano Roll+Velocity, MIDI/audio record, automation, mixer, plugin browser và VST3; **một số tính năng từ v1 còn 'Not yet ported from v1'**. Không giả vờ Zrythm v2 đầy đủ ngay lập tức.
- Native Windows có thể dùng VST3 theo khả năng host; **không đảm bảo Kontakt hoặc mọi plugin sẽ hoạt động** trước khi QA thực tế.
- **AGPL-3.0-or-later** cùng điều khoản bổ sung, license của phụ thuộc cần rà; phải công khai mã nguồn tương ứng và ghi công theo giấy phép khi phân phối. **Không** chuyển Zrythm source sang MIT, không trộn vào SoundCraft MIT cũ mà chưa đánh giá tác động pháp lý.
- Trademarks Zrythm không được dùng cho sản phẩm chỉnh sửa mang tên ORVIA khi phân phối; dùng thương hiệu riêng và giữ ghi công nguồn gốc cùng các legal notices theo điều khoản.

## Kế hoạch và điều kiện nghiệm thu

### S0 — Repo độc lập & kiểm tra pháp lý
- Tạo fork/repository riêng từ **toàn bộ mã Zrythm cùng lịch sử Git** (không copy một ít mã vào SoundCraft rồi tự gọi là fork).
- Chốt upstream ref/tag cố định; đối chiếu LICENSES/TRADEMARKS và các phụ thuộc được đóng gói.
- Đánh dấu rõ trong preview: repository đã có/chưa có, chưa build khi chưa có bằng chứng.

### S1 — Windows baseline build (KHÔNG đổi thương hiệu vội)
- Biên dịch ứng dụng Zrythm gốc theo quy trình Windows của upstream, tách output build khỏi repo.
- Khi có runner hợp lệ, lưu log đầy đủ hoặc liên kết build success/failure và ảnh chụp từ chương trình native thực.
- Chỉ khi chạy được mới bắt đầu xác định module muốn đổi thương hiệu.

### S2 — Đổi tên/visual ORVIA nhưng giữ engine
- Thay tên, icon, About, titlebar, launcher, installer; loại toàn bộ dấu hiệu thương hiệu không được phép, giữ thông báo pháp lý và attribution.
- Giữ nguyên Project/MIDI engine, plugin host, save/open/export trong lần đầu; có test hồi quy.

### S3 — Chức năng thực + kiểm chứng
- Trên Windows: test mở project, MIDI editor, velocity, piano audition, Audio/WAV, sound, transport, mixer, save/reload, VST3, xuất audio.
- So sánh chính xác với nhu cầu đã thống nhất. Không phát hành phần mềm nếu nhiều đường chức năng chưa hoạt động.

### S4 — Bản đóng gói Windows và nghiệm thu
- Xuất gói Windows có bằng chứng build, hash, hướng dẫn kiểm thử ngắn gọn.
- Chủ dự án nghiệm thu. Sau đó gỡ Cloudflare Preview tạm; không tự xóa trước khi được đồng ý.

## Giới hạn của trang web theo dõi
Cloudflare Pages xem **tiến độ/tài liệu/ảnh hoặc video ghi từ desktop native**; không có thao tác trực tiếp trên DAW desktop. Để người dùng tự bấm Play, chỉnh MIDI trước khi tải về phải có môi trường Windows chạy từ xa, không có sẵn miễn phí trong phương án này. Không thêm hình ảnh/screenshot giả.

## Tình trạng ngày 20/10/2026
- Đã đọc technical log LUNOR MUSIC và ORVIA; đã khảo sát upstream README/CMake/trademark/license.
- **Chưa fork** Zrythm vào tài khoản GitHub chủ dự án: kết nối GitHub hiện có không cung cấp chức năng tạo fork/repository.
- **Chưa có native Windows build/ảnh từ bản chạy thật.**
- Nhánh `feature/orvia-zrythm-desktop-transition` của repo SoundCraft được dùng **chỉ để chuẩn bị trang tiến độ tạm**, không phải repository mã nguồn Zrythm và không phát hành sản phẩm.
