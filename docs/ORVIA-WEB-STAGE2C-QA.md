# ORVIA Web — Giai đoạn 2C: Kiểm thử tổng thể và sửa lỗi an toàn

**Ngày:** 2026-10-22  
**Nhánh kiểm thử:** `feature/orvia-web-stage2c-qa`  
**Nguồn:** 2B (`feature/orvia-web-stage2b-timeline-edit`, PR #8), kế thừa 2A (Be Vietnam Pro, PR #7).  
**Tuyên bố chính xác:** Đây là **static source audit + mã sửa lỗi + các regression tests đã ghi**, không phải chứng nhận QA runtime; môi trường hiện không có Cargo/rustc và không kết nối đến Cloudflare Pages qua Chromium. Không sửa production, không dùng GitHub Actions/TinyFish, dịch vụ tính phí, hoặc CMD trên Windows của chủ dự án.

## Cổng nghiệm thu & bằng chứng

| Cổng | Bằng chứng | Kết quả 22/10 |
|---|---|---|
| A. Mã nguồn GĐ2C riêng biệt | Nhánh `feature/orvia-web-stage2c-qa`, PR kiểm thử trên GitHub | Source committed |
| B. Cloudflare deploy đúng HEAD 2C | GitHub Checks / Pages bot phải báo `success` ở đúng SHA | **PENDING** — không suy diễn từ 2A |
| C. Font Việt trong DAW web | Open Chrome/Edge 1366×768, xem “THÔNG TIN CLIP”, “Độ dài”, “NHẠC CỤ & HIỆU ỨNG” | **UNVERIFIED** — có code/asset và 2A đã từng báo deploy thành công |
| D. Tool/Timeline | Move/Trim/Pencil/Grid snap khi kéo clip; Cmd/Ctrl đảo Snap; không có toolbar trùng | **UNVERIFIED runtime**, source có các handler |
| E. Info Line Bars/Beats/Time | Chọn clip Keys, xem Bars/Beats, sửa Start/Length; Time-lock phải vô hiệu hóa Start/Length | **FIXED IN SOURCE + tests pending** |
| F. Duplicate an toàn | Chuột phải clip MIDI → Nhân bản; bản gốc y nguyên; clip phía sau dịch chuyển, không bị ghi đè | **FIXED IN SOURCE + tests pending** |
| G. Split đúng một clip | Đặt Playhead trong clip → Chia; clips ở Track khác và lân cận không thay đổi | **FIXED IN SOURCE + tests pending** |
| H. Locks, Undo/Redo | Cấm duplicate/split clip locked; từ chối split ở mép/ngoài; Undo + Redo khôi phục chính xác | **REGRESSION WRITTEN — NOT EXECUTED** |
| I. Inspector/MixConsole | Tạo Send tới Bus có thật → chỉnh Send level/mute; đổi Output Main/bus; M/S/Fader có tác dụng | **UNVERIFIED runtime**, source đã nối lệnh |
| J. Web audio | Play/Stop 9 tracks; nghe Kick/Snare/Hats/Keys; chỉnh Mute/Fader; không mất tiếng sau edit/Undo | **PARTIAL EVIDENCE:** người dùng đã xác nhận Play có tiếng ở GĐ1; GĐ2C chưa test |
| K. Lưu/đọc project | Save .scraftz → reload → open → notes/velocity, clips, bus/routes/volume được bảo toàn | **REGRESSION WRITTEN (MIDI clips), NOT EXECUTED** |
| L. Import/Export & offline file | WAV/MIDI import, bounce WAV, SMF roundtrip; so play/duration/CC/velocity | **UNVERIFIED runtime**; chưa đóng GĐ1 |
| M. Layout laptop | Panel 1366×768 và 1920×1080 không che timeline, Info Line 31px, có splitter | **UNVERIFIED runtime** |

## Lỗi phát hiện và sửa trong 2C

1. **BUG-2C-01 (P1, code):** `clip.duplicate_one`/`clip.split_one` dùng precondition `has_selection` dù nút chuột phải đã chuyển **ClipId** rõ ràng. Khi không có Track được chọn, lệnh bị chặn. Sửa thành precondition `always` với kiểm tra nghiêm ngặt `clip` ID, lock, time boundary trong handler. Kiểm thử `orvia_stage2c_explicit_clip_does_not_require_track_selection`.
2. **BUG-2C-02 (P1, safety):** Start/Length ở Info Line chỉ khóa theo `edit_locked` mà bỏ qua `time_locked`. Sửa UI tắt điều khiển vị trí/độ dài khi có bất kỳ khóa nào; Gain/Mute vẫn tách biệt theo semantics SoundCraft.
3. **REG-2C-03:** Test `orvia_stage2c_invalid_split_does_not_change_project` với vị trí 2 đầu clip, id không tồn tại và clip edit-locked.
4. **REG-2C-04:** Test `orvia_stage2c_duplicate_preserves_adjacent_clips_and_honours_downstream_lock` dựng 3 clips MIDI liền nhau. Kiểm tra thiếu chỗ vì clip phía sau bị lock sẽ từ chối không thay đổi dữ liệu; sau khi mở lock, copy chuyển clip kế tiếp đúng độ dài, Undo/Redo khôi phục dữ liệu.
5. **REG-2C-05:** Test `orvia_stage2c_duplicate_split_bundle_roundtrip_preserves_midi` tạo bản sao và chia clip Keys, đảm bảo Track Snare không đổi, serialize `.scraftz` → deserialize và kiểm tra dữ liệu clip giống phiên đang chỉnh.

**Lưu ý:** không thể gọi các bài kiểm thử là PASS cho đến khi chạy cargo test trên nguồn này. `git commit`, bot Cloudflare success và audio browser QA là ba cột độc lập.

## Các bước kiểm thử browser cần làm khi preview sẵn sàng

1. Vào đúng **immutable URL** được Cloudflare Pages tạo cho commit GĐ2C; xác nhận DAW Rust/WASM xuất hiện và không màn hình đen. Nếu WebGPU lỗi thử `?webgl`.
2. Tại Track Keys (MIDI), chọn clip và xem Info Line: “Nhịp 5|1|000”, Start, Length; kiểm tra giá trị phản ánh clip thực. Chọn clip Audio Kick và kiểm tra Gain xuất hiện; khóa clip rồi xác minh nút Start/Length vô hiệu hóa.
3. Trên Keys: nhấn chuột phải → Nhân bản clip; kiểm tra clip phía sau không bị mất; Ctrl+Z → Ctrl+Shift+Z; đặt Playhead ở giữa clip → Chia, rồi Undo; nhấp Grid/Snap, kéo clip và Trim mép phải.
4. Mở Inspector Track Kick, thử Fader/Mute/Solo và Sends tới Bus có thật, sau đó hoàn tác; kiểm tra chỉ track tương ứng thay đổi.
5. Phát demo sau mỗi thao tác. Dùng Save để tải `.scraftz`; refresh, Open file vừa lưu và so sánh nốt MIDI, clips, volumes.
6. Import tệp WAV/MIDI có âm thật, chỉnh và Bounce WAV, nghe WAV đầu ra. Ghi rõ các lỗi, không chỉ trả lời “ok”.
7. 1366×768 + 1920×1080: nhãn Việt đầy đủ dấu, Info Line có thể cuộn ngang khi nhiều thuộc tính, Timeline vẫn đủ rộng, Side Panels resize.

## Cổng dừng trước Giai đoạn 3

- Cloudflare build/deploy HEAD 2C phải thành công, không có compile warnings/errors nghiêm trọng.
- Cargo test phần engine và UI phải chạy PASS hoặc có kết quả FAIL kèm lỗi đã xử lý.
- Chuỗi Browser QA mục D–M trên có bằng chứng; chưa đủ bằng chứng thì **không coi GĐ2 hoàn tất**.
- Không phát hành production, không chuyển GĐ3 khi còn lỗi mất clip/không undo/lưu project lỗi.

## Tài liệu tham chiếu kỹ thuật

- PR #7 GĐ2A, #8 GĐ2B và nhánh GĐ2C.
- `crates/engine/src/cmd/clip.rs` (targeted clip ops + tests);
  `crates/ui-egui/src/orvia_project_bar.rs` (Info Line, BBT and locks);
  `crates/ui-egui/src/edit_window.rs` (Timeline gestures & right-click);
  `apps/soundcraft-web/src/web.rs` + `apps/soundcraft-web/index.html` (Save/Open & unsaved guard);
  `crates/ui-egui/src/panels.rs` (Inspector).
- Công việc chưa nghiệm thu ở GĐ1 được giữ nguyên, không viết lại lịch sử bằng trạng thái PASS sai.
