# ORVIA — Track Header Cubase-inspired Compact UX

**Đề nghị chủ dự án:** dùng giao diện Track List từ ảnh Cubase làm chuẩn về mật độ, tổ chức và thao tác. Bỏ đề xuất mockup bo tròn lớn trước đó. Làm gần cảm giác Cubase, nhưng không sao chép asset/icon/logo hoặc nội dung proprietarily protected của hãng khác.

**Nền code:** ORVIA 4C.2 `feature/orvia-4c2-windows-portable-data`, PR #26 HEAD docs-only `b949af96d0c36df4766a960f0bf80a46ed6706dc`. Tách nhánh độc lập `feature/orvia-track-header-cubase-inspired`; PR Draft, không merge main/production.

## Code UI thực, không chỉ ảnh

- `crates/ui-egui/src/edit_window.rs`: kích thước cột Track Header vẫn **136px**, dùng `track.height.points()` nguyên vẹn cho cả Header và Timeline; **không tự sửa chiều cao MIDI/audio clip, Scroll, meter hoặc session TrackHeight**. Strip màu người dùng thu còn **5px** bên trái. Vùng nền xám đen trung tính `#252A2F`, Track selected `#3D4348`; Light dùng palette riêng. Track/clip color ở lane giữ nguyên.
- **Hàng 1:** nút **M/S** nhỏ liền kề **tên Track** và menu playlist. Tên dài chỉ ellipsis/clip hiển thị nhưng giữ nguyên giá trị `track.name`; tooltip toàn tên, double click rename; kéo tên reorder Track như cũ.
- **Hàng 2:** **Record (R-dot), Monitor (I), Edit (e, menu lệnh thật), Automation (A, toggle automation lane)**. Gắn đúng `mix.record_arm`, `mix.input_monitor`, `mix.mute`, `mix.solo`, `track.view`, `track_context_menu`. Điều khiển nén dạng hình chữ nhật bo góc 2px, không glow sáng.
- **Khả năng co giãn theo TrackHeight:** Micro 16 chỉ tên; Mini 24 tên + M/S; Small 46 hai hàng chính; Medium 88 bổ sung waveform/notes selector + freeze/dyn/automation read; Large 132 có cả timebase, plugin/elastic selector. Các lệnh khác giữ nguyên: Track height resize, folder toggle, playlist create/duplicate/select, freeze, inactive, automation mode, timebase, elastic, meter.
- **Nền tảng kiểm tra:** thêm Rust palette/layout unit tests và `tests/orvia-track-header-compact-contract.mjs` sáu nhóm source contracts; nối gate này vào `scripts/cloudflare-pages-build.sh` sau 4C.2 preflight trước Rust WASM compile.

## QA manual sau Cloudflare PASS (chưa chạy trực tiếp Chrome)

| Case | Viewport / tình huống | Mong đợi |
|---|---|---|
| TH-01 | 1366×768 và 1738×863, 8 Track | Tên/M/S hàng 1, Record/Monitor/e/A hàng 2, màu strip mỏng, không đè Timeline |
| TH-02 | Track Mini 24, Small 46, Medium 88, Large 132 | Thao tác resize bình thường, clip/audio MIDI lane cao đúng TrackHeader |
| TH-03 | Chọn Kick/Snare, CTRL click multi-track | Chỉ highlight nền xám, không đổi track colors, selection và reorder hoạt động |
| TH-04 | Click M/S/R/I, double click rename | Trạng thái phản ánh lệnh thật; Solo implies Mute; Undo/Redo theo engine nếu có |
| TH-05 | Click e/Edit, Automation A, view waveform/notes, dyn/read | Context menu mở được; automation lane bật/tắt; selectors thật hoạt động |
| TH-06 | Folder/Master/Aux/VCA, Playlist menu, menu chuột phải | Không hiện nút bị vô hiệu hóa; Folder toggle/Playlist không mất lệnh |
| TH-07 | Dark/Light/System, tên dài và tiếng Việt | Chữ đủ tương phản, tooltip tên gốc, không tràn vào Meter |
| TH-08 | Record/Playback, Save/Open, MIDI/Audio, Kontakt Desktop | Không thay audio engine; không tự nhận runtime Desktop/ASIO/VST3 PASS từ Cloudflare |

## Cổng triển khai

- [x] Đọc nhật ký trung tâm + repo trước sửa; đối chiếu 4C.2 và giao diện thật.
- [x] Chỉ sửa source Edit Track Header; không thay DSP/Session schema/track audio data.
- [x] Static compare các lệnh trong `track_header`: **13 lệnh** `track.height`, `track.folder_toggle`, `track.view`, `track.move`, 4 playlist actions, freeze, inactive, automation mode, timebase và elastic đều giữ nguyên. `mix.mute/solo/record_arm/input_monitor` giữ trong mảng command mapping mới.
- [x] Thêm Rust unit tests & source contract; Cloudflare script gắn test.
- [ ] Cloudflare check-run `completed/success` trên đúng HEAD của nhánh Track Header; mới dùng immutable Preview do bot công bố.
- [ ] Browser click QA / quan sát ảnh thực tế trên 1366/1738, audio MIDI regression, keyboard controls.
- [ ] Người dùng nghiệm thu UI thật.

**Giới hạn:** Không dùng ảnh mô phỏng làm bằng chứng code/Web build. Không merge production; không trigger Actions thủ công, dịch vụ phí, TinyFish hoặc CMD Windows. 4C.2 Desktop Native QA còn HOLD riêng.
