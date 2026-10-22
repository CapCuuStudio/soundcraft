# ORVIA Web — Giai đoạn 3C: Instrument & FX Rack QA

**Ngày:** 23/10/2026  
**Baseline:** Stage 3B branch `feature/orvia-3b` (Cloudflare confirmed code build for `07a569d`; subsequent changes only technical docs).  
**Branch:** `feature/orvia-3c-plugin-rack`.

## Tính năng triển khai

- Nâng cấp **nhánh Browser bên phải** đang hoạt động thành rack nhạc cụ & hiệu ứng, sử dụng trực tiếp registry `soundcraft_dsp::plugins()` và engine commands.
- Chọn chính xác Track cần xử lý, Instrument Track mới có thể được tạo ngay trong Browser (bằng `track.new`).
- Nhạc cụ built-in có thật: Subtractive Synth, Drum Synth, Sampler (Sampler yêu cầu nạp sample trước khi có tiếng); giữ các mặc định khác và hỗ trợ mở cửa sổ thông số.
- Effects built-in: tìm tên/ID, lọc theo category, **chọn Insert slot còn trống**, `mix.insert`, mở cửa sổ thông số thật, Bypass `mix.insert_bypass`, xóa `mix.insert_remove`.
- Không ghi đè Insert slot đang chứa hiệu ứng từ thao tác **Thêm**. Nếu đầy 10 slot, không hiện thao tác thêm được; nút xóa cho phép giải phóng chỗ.
- Instrument hiện tại khi được chọn lại chỉ **mở tham số**, không reset settings về default qua `mix.instrument`.
- Mọi plugin hiển thị bởi Browser lấy từ `soundcraft_dsp` built-in registry (không tạo ID không tồn tại).
- Trình duyệt không tuyên bố VST3/CLAP/AU/Kontakt native có thể host trong WASM; menu native desktop hiện hữu không bị xóa. Browser không tải thư viện Kontakt/NKI vào Cloudflare.

## QA Gates

1. **Cloudflare compile/deploy** từ Draft PR riêng đúng SHA GĐ3C, phải có status “Deploy successful” và URL Preview thực tế; không đoán trước URL.
2. **Rust tests**: `stage3c_catalog_is_real_builtin_and_classified`, `stage3c_track_type_and_insert_free_slot_safety`, các bài engine của `mix.instrument`, `mix.insert`, `mix.insert_remove`. Chưa có Cargo tại sandbox, phải đánh dấu *pending* cho tới khi chạy thật.
3. **Browser UX trên Chrome 1366×768**: Project Window → Browser → Nhạc cụ, chọn Instrument Track, chọn Subtractive Synth rồi Drum Synth, kiểm tra đúng Track và `Undo`; mở bảng tham số và chỉnh mức/âm.
4. **Sampler**: chọn Sampler, thông báo cần sample; nạp WAV thật bằng cửa sổ Sampler hoặc kéo thả hợp lệ, sau đó mới kỳ vọng có âm. Không nhầm Sampler không có sample thành lỗi phát.
5. **Effects**: Audio Track → Hiệu ứng → chọn EQ/Compressor/Reverb → Insert 01, sau đó thêm một plugin khác vào Insert 02. Chạy phát nhạc thực; kiểm tra Bypass ON/OFF và Undo, đổi tham số, xóa riêng slot 01 không ảnh hưởng slot 02. Lưu/mở `.scraftz` giữ plugin ID/slot/bypass/params.
6. **Tính an toàn**: Track MIDI (không phải Instrument) không nhận instrument; Track không có đường audio không gán FX; 10 slots đầy không overwrite âm thầm; chuyển Track thì Rack phản ánh Track mới.
7. **Hồi quy**: timeline 2C (copy/split/delete/undo/save), MixConsole 3A, Piano Roll/Velocity/CC 3B, Play và MIDI export còn hoạt động.
8. **Native claims**: Browser web chỉ hiện built-in. Host VST3/Kontakt native phải được kiểm chứng trên desktop Windows ở giai đoạn riêng, không thuộc QA Web 3C.

**Giới hạn:** không merge main, không cập nhật production, không tự chạy GitHub Actions/phí, không TinyFish, không yêu cầu CMD Windows. Không tuyên bố nghiệm thu browser/audio khi người dùng chưa kiểm thử.
