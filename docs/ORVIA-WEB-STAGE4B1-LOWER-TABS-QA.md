# ORVIA Web — 4B.1 Lower Zone Tabs QA

Ngày: 2026-10-24. Base: `feature/orvia-4a4-track-timeline` (owner accepted; code Cloudflare PASS `e383ebc`, docs-only head `4eb63d8`).
Branch: `feature/orvia-4b1-lower-tabs`.

## Phạm vi đã thay đổi (UI-only)

- `crates/ui-egui/src/edit_window.rs`: footer tab strip 22 -> 32 logical px; 12pt tab labels trong đúng vùng tab.
- Four real tab actions: MixConsole, Key Editor, Memory Locations, Undo History. Các label được trình bày bằng chữ dễ đọc thay vì hàng chữ in hoa; engine command IDs và logic open/close không đổi.
- Graphite/Cyan selected background + cyan underline 2 px; hover/focus visible; bảng màu Dark/Light riêng, không ép Dark.
- Nhóm tab được cuộn ngang trong vùng riêng khi viewport hẹp, nút `Đặt lại bố cục` luôn nằm bên phải và gọi nguyên `ui.dock.reset`.
- Thêm 2 Rust unit tests cho palette/tab height. Không đổi Mixer, MIDI Editor, session, DSP, playback, audio/video import-export, VST/CLAP, plugin list, quyền truy cập hoặc saved docking layout.

## Cổng kiểm thử và nghiệm thu

1. Cloudflare Pages **Deploy successful** đúng commit 4B.1; cung cấp immutable Preview chỉ sau khi xác minh.
2. Mở Chrome tại 1366×768, 1440×900, 1738×863 và hẹp hơn: footer tab 32px, label không cắt, tab không tràn lên nút đặt lại bố cục. Scroll ngang phải truy cập được đủ bốn tab.
3. Click MixConsole -> mở Mixer thật; click Key Editor -> đóng MixConsole, mở MIDI Editor thật; click tab đang mở -> đóng; double-click clip MIDI vẫn mở Key Editor.
4. Memory Locations/Undo History toggle đúng panel hiện có; reset layout thật chỉ khi click; trạng thái active và màu không giả chức năng.
5. Dark/Light/System: chữ, viền focus, underline/hover đủ rõ, không ghi đè theme lưu của người dùng.
6. Hồi quy: 4A.4 Track Header/Timeline, 4A.3 Studio Rack, Inspector 4A.1, Toolbar 4A.2, Undo/Redo, Save/Open, audio và MIDI trên browser.
7. Rust Cargo format/clippy/unit tests + Chromium audio/MIDI: phải xác minh độc lập; Cloudflare WASM build không thay thế QA.

## Không được suy diễn

- Commit code mới chỉ xác nhận đã lưu nguồn trên GitHub, chưa đồng nghĩa build WASM thành công.
- Nếu Cloudflare đang in progress hoặc failed, không giao preview 4B.1 như đã hoàn thành.
- Không chạy GitHub Actions thủ công khi chưa có chủ trương; không merge main/production, không dùng TinyFish, dịch vụ trả phí, CMD Windows người dùng.
