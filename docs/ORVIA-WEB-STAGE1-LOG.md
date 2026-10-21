# ORVIA Web — Giai đoạn 1: Nền tảng, âm thanh, dữ liệu

**Ngày bắt đầu:** 22/10/2026  
**Nhánh:** feature/orvia-soundcraft-rebuild — Draft PR #6  
**Mục đích:** Một ứng dụng DAW thật (Rust/egui/WASM), test chuỗi Add Track → MIDI → Play/Mixer → Import → Save/Open → Export. Không thêm thành phần giao diện giả, không đụng production.

## Trạng thái phân biệt rõ
- **SOURCE IMPLEMENTED:** code đã được commit, không đồng nghĩa đã chạy hay QA pass.
- **BUILD CONFIRMED:** Cloudflare bot trả 'Deploy successful' ở **đúng commit**.
- **RUNTIME VERIFIED:** có kết quả thao tác + âm thanh trên Chrome/Edge; chưa đạt ở lượt này.
- **UNIT TEST VERIFIED:** cargo test đã chạy pass; **chưa có kết quả cho các test mới**.

## Các sửa lỗi / bài kiểm thử đầu tiên
1. apps/soundcraft-web/src/web.rs: người dùng chọn hoặc kéo thả session .scraft/.scraftz trong khi project hiện tại còn dirty sẽ **không thay project**. Thay vì âm thầm mất dữ liệu, ứng dụng báo cần lưu/tải project trước; cả file picker và drag/drop đều đi qua nhánh guard này.
2. apps/soundcraft-web/src/web.rs + apps/soundcraft-web/index.html: trạng thái dirty lấy từ SoundCraft engine được phản ánh sang trang web; beforeunload chỉ yêu cầu xác nhận khi dirty. Tính năng này phụ thuộc trình duyệt và có thể không hiện nếu người dùng chưa tương tác.
3. crates/engine/src/io.rs: bài test **orvia_web_instrument_four_notes_round_trip_with_velocity_and_mixer**: tạo Instrument Track bằng track.new, tạo MIDI Clip và bốn Note với velocity 35/66/92/120; set fader -9 dB; đóng gói session_bundle và open_session_bytes; kiểm tra định danh plugin synth, MIDI pitch/start/velocity, volume và clean state.
4. Cùng bài test có offline mixer sound check render_range (peak > 0.0001) và kiểm tra mix.mute (peak < 0.0001). Tránh tuyên bố nghe thấy tiếng khi chưa chạy test.
5. Các test đã có trước đó trong SoundCraft: io.rs bundle_round_trips_session_and_audio; MIDI create_clip_add_edit_move_delete, engine demo_builds_and_renders_sound. Không nhận là test mới.

## Kiểm thử thực tế phải đạt, theo thứ tự
- [ ] Cloudflare Pages Deploy successful đúng HEAD chứa cả bảo vệ beforeunload + 4-note regression; WASM asset có thật.
- [ ] Mở immutable preview bằng Chrome/Edge desktop, không màn hình đen; thử WebGL2 khi WebGPU lỗi.
- [ ] Chọn Instrument Track → tên ORVIA Piano → built-in subtractive_synth → vẽ 4 nốt trong Key Editor.
- [ ] Nhấn Play: có tiếng, playhead chạy; fader -9 dB nghe nhỏ hơn; Mute im tiếng; Stop dừng.
- [ ] Add 2 Audio/MIDI Track, Undo/Redo, lưu .scraftz và mở lại: nốt + volume + instrument không mất.
- [ ] Import WAV và MIDI thật, kéo clip, save/open media kèm audio trong bundle không bị missing.
- [ ] Bounce WAV; thử file xuất và playback.
- [ ] Test không mất dữ liệu: mở một project khác trong khi đang có thay đổi; reload tab phải nhận cảnh báo. Thử dự án đã lưu không hiện cảnh báo.
- [ ] Test 1366×768 laptop và desktop; console errors, load, frame drops, audio silent-clock.
- [ ] Chạy cargo test -p soundcraft-engine, cargo test -p soundcraft-ui-egui ở môi trường Rust phù hợp và ghi chính xác kết quả.

**Giới hạn:** Môi trường ChatGPT hiện không có cargo/rustc, không truy cập trực tiếp URL Cloudflare để tương tác; do đó các hộp trên chưa được đánh dấu đạt. Chỉ GitHub/Cloudflare bot có thể chứng minh build, không chứng minh runtime audio. Không VST3/Kontakt Windows native trên web.

**Quy tắc:** Mỗi lỗi nêu file/commit/tác động/test, không tự phát hành production, không GitHub Actions tốn quota, không paid services, không TinyFish, không CMD máy người dùng.
