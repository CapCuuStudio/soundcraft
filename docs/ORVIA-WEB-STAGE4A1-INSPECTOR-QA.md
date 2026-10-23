# ORVIA Web — Giai đoạn 4A.1: Inspector UI cleanup / QA

**Ngày:** 23/10/2026. **Branch:** `feature/orvia-4a1-inspector-layout`. **Base:** GĐ3D branch `feature/orvia-3d-track-automation` (người dùng xác nhận “xong rồi”, Cloudflare PASS cho commit `6f192bc`; các commit sau chỉ là tài liệu). Không merge main/production.

## Yêu cầu / lỗi quan sát được
Ảnh Chrome 1738×863 do người dùng cung cấp: Inspector chỉ ~200px; tiêu đề dài và nút Automation quá sát nhau, Volume/Pan thiếu khoảng ngang, danh sách Insert tốn nhiều chiều cao. Người dùng phê duyệt 4A.1 nhằm nâng Inspector, không làm lại toàn bộ UI.

## Thay đổi thực hiện
- `docking.rs`: default Inspector **222 → 270px**, dải kéo ngang từ min **190px** đến max **410px**; giữ nguyên các panel Timeline/MIDI/Rack khác và không can thiệp bố cục người dùng đã lưu. Một workspace đã lưu từ trước có thể cần kéo vạch chia hoặc dùng **Reset Layout** nếu người dùng đồng ý đặt lại bố cục.
- `panels.rs`: tiêu đề Track chữ lớn và bọc dòng, thanh màu Track; Mute/Solo, Ghi âm/Monitor thành nút 2 cột, hiển thị đúng trạng thái; Đổi tên và Thêm Track rõ ràng hơn.
- Volume/Pan được tách thông số và thanh slider toàn chiều ngang; dữ liệu điều khiển vẫn là `mix.volume`, `mix.pan`, hợp nhất Undo như trước, không thay đổi giá trị hoặc công thức engine.
- Các mục Routing, Instrument, Inserts, Sends và Automation được gom nhóm; Inserts thu lại lúc đầu để tiết kiệm chỗ nhưng vẫn mở được đủ các nút; Menu cho plugin/bus vẫn gọi engine hiện hữu.
- `panels/orvia_automation.rs`: chuyển nút “Hiện đường trên Timeline”, thêm điểm, ghi vùng chọn và slider sang hàng riêng, bật xuống dòng cho hướng dẫn; vẫn dùng `track.view`, `automation.set_point`, `automation.clear`, `automation.write_range` với dữ liệu thật.
- Rust unit test mặc định độ rộng Inspector và khả năng co giãn.
- Không đổi DSP, playback, MIDI, project/session, plugin host, Build CI/CD, MixConsole, Piano Roll và Rack.

## Cổng QA (không được đánh dấu PASS trước khi có bằng chứng)
1. PR branch preview Cloudflare `Deploy successful` cho **đúng SHA mã 4A1** và URL bất biến; nếu build failed đọc lỗi và sửa trên nhánh 4A1.
2. Rust Cargo UI unit tests gồm `orvia_4a1_inspector_default_is_readable_but_resizable`. Chưa tự chạy vì sandbox chưa có Cargo, chỉ Cloudflare WASM build không tương đương PASS unit tests.
3. Chrome mới tạo workspace ở 1366×768, 1440×900: Inspector ~270px, tên dài xuống dòng, nút M/S/Record/Monitor dễ bấm, slider Volume/Pan không cắt; cuộn dọc đến Automation và nút Thêm Track.
4. Chrome workspace đã lưu của user: kiểm tra Inspector **không bị reset bất ngờ**. Nếu sidebar vẫn hẹp, kéo vạch chia Inspector để mở rộng; Reset Layout chỉ khi người dùng tự muốn khôi phục default.
5. Track Audio/Instrument/MIDI: chọn đúng Track, Mute/Solo/Record/Monitor, Volume/Pan, đổi tên, chuyển Output/Bus, mở/đổi Insert, hiệu ứng Send. Đảm bảo không tạo bản sao dữ liệu và Undo/Redo hoạt động.
6. Automation 3D: thêm/sửa/xóa 2 điểm Volume, đổi Pan, bật lại clip view, Save/Open. Piano Roll 3B, MixConsole 3A, Rack 3C, Play/Stop và Export không bị ảnh hưởng.
7. Ghi nhật ký repo và Drive. Không TinyFish, GitHub Actions tự phát, dịch vụ phí, CMD Windows, không merge main/production.

**Trạng thái ban đầu:** mã đã push / Chrome UAT và Rust tests còn chờ.