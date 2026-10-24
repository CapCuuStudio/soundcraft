# ORVIA Design Language v1.0 — Bộ quy chuẩn giao diện

**Ngày:** 2026-10-24 · **Trạng thái:** Quy chuẩn v1.0, chờ người dùng xem và duyệt hình thức trước khi áp dụng đại trà  
**Hướng thiết kế người dùng đã chọn:** *Thay đổi vừa phải — riêng biệt nhưng quen thuộc với người dùng DAW*  
**Phạm vi:** Rust/egui Web & Desktop; **đây là tài liệu thiết kế, chưa phải bản giao diện đã được sửa hoặc build**.

## 1. Tuyên ngôn thiết kế

**ORVIA Studio**: “Focus on sound” — gọn, chính xác, nhịp nhàng, có chiều sâu; ít viền và nút dày đặc hơn giao diện workstation truyền thống. Giữ bố cục quen thuộc **Toolbar trên – Inspector trái – Timeline trung tâm – Browser/Rack phải – Lower Zone dưới**. Ưu tiên hiển thị thông tin âm nhạc hơn trang trí; ưu tiên Timeline hơn chrome của ứng dụng.

- Không tái tạo 1:1 Cubase, Pro Tools, logo/icon, kiểu phối màu hay tài sản thương hiệu của phần mềm khác.
- **Không đổi engine, DSP, MIDI, DAW commands, project/session schema, VST host, Save/Open/Export hoặc workflow nhạc cụ hiện có.**
- Làm khác bằng ngôn ngữ màu, typography, mật độ bố cục, panel header, states và hình học nút — **không phá bố cục hoặc cách thao tác đã quen**.
- Tất cả màu UI theo **semantic tokens**. Màu track/clip trong dự án là dữ liệu người dùng; không tự viết đè, đổi màu nhạc cụ hoặc làm thay đổi waveform/MIDI.

## 2. Bộ màu chuẩn ORVIA Graphite / Cyan (Dark, sRGB)

Các giá trị sau là nguồn chuẩn cho **bản thiết kế v1** và file machine-readable đi kèm `docs/ORVIA-DESIGN-TOKENS-V1.json`. Tên là **design token**, không đồng nghĩa các trường Rust `Tokens` đã có được đổi ngay.

| Design token | Hex | Dùng cho |
|---|---|---|
| `color.canvas` | `#101820` | Nền khung ứng dụng |
| `color.panel` | `#17212B` | Inspector, Rack và pane nội dung |
| `color.raised` | `#202D39` | Toolbar group, nút và khối nổi |
| `color.hover` | `#263A48` | Hover trên vùng tương tác |
| `color.timeline` | `#141D26` | Timeline trống, không phủ clip |
| `color.border` | `#354653` | Đường ranh panel, 1px |
| `color.grid` | `#293744` | Grid phụ trong Timeline |
| `color.barLine` | `#405363` | Đường bar chính (nhỉnh hơn grid) |
| `color.textPrimary` | `#E5EDF4` | Tiêu đề, nhãn chính |
| `color.textSecondary` | `#AAB9C6` | Nhãn phụ / mô tả |
| `color.textHint` | `#8FA9B8` | Gợi ý, metadata |
| `color.accent` | `#47D7D1` | Active/focus, Play đang phát, selection rail |
| `color.accentInk` | `#0B2530` | Chữ/icon trên nền accent sáng |
| `color.selection` | `#1B424C` | Hàng/panel đang chọn (nền) |
| `color.record` | `#F27476` | Record armed/recording |
| `color.solo` | `#F2B86B` | Solo, cảnh báo vừa |
| `color.success` | `#97E1B2` | Monitor/ready/Automation Read |
| `color.disabledText` | `#8094A4` | Text vô hiệu hóa, kèm tooltip lý do |

**Quy tắc sử dụng:** accent chỉ chiếm ~5–10% diện tích các nút/đường nhấn, không tô xanh cyan toàn bộ panel; đỏ chỉ dành Record/error; vàng chỉ dành Solo/cảnh báo; không dùng xanh active và đỏ Record cho cùng một ý nghĩa. Các màu này **không thay track palette**.

**Tương phản đã tính ở dạng sRGB:** Primary trên Panel = 13.77:1; Secondary trên Panel = 8.12:1; Hint trên Panel = 6.62:1; Accent trên Panel = 9.24:1; AccentInk trên Accent = 9.01:1. Quy định chữ thường đạt tối thiểu WCAG AA 4.5:1, trừ nhãn thực sự disabled; chỉ báo trạng thái phải có icon/text bổ sung, không dựa độc quyền vào màu. Mọi lần cập nhật palette tiếp theo cần kiểm tra lại tỷ lệ thật.

**Light/System theme:** Hệ thống đã hỗ trợ Dark/Light/System. Trong 4A.3 ưu tiên kiểm tra Dark, **không xóa Light/System hoặc áp Dark mặc định ghi đè thiết lập người dùng**. Bộ màu Light phiên bản riêng sẽ được định nghĩa khi triển khai Light; không tự suy ra “đảo ngược màu” từ bảng Dark.

## 3. Chữ và số (Typography)

| Vai trò | Cỡ egui points | Weight | Áp dụng |
|---|---:|---|---|
| Brand ORVIA | 17 | semibold/bold | Header thương hiệu riêng, không che menu |
| Panel title | 13 | bold | Inspector, Rack, Mixer section |
| Body / action | 12.5–13 | regular | Nút, menu, thông tin Track |
| Secondary | 11.5–12 | regular | Subtitle, helper, routing |
| Numeric DAW readout | 13–15 | mono/tabular | BPM, volume dB, pan, thời gian |
| Main transport counter | 23–26 | mono/tabular | Counter lớn |
| Compact metadata | 11 | regular | Giá trị phụ, hint; không dưới 11pt nếu cần đọc |

- **Web:** dùng **Be Vietnam Pro** đã được bundling (OFL) trong `crates/ui-egui/src/fonts.rs`; không thêm font mới. **Desktop:** giữ font hệ thống đã được loader chọn; `FontFamily::Proportional` / `FontFamily::Name("bold")` / `FontFamily::Monospace` như hiện tại.
- Tránh viết HOA TOÀN BỘ ở mọi panel; dùng Title Case/tiếng Việt ngắn gọn. Chữ có dấu phải đủ line-height và không clip.
- Không rút ngắn tên Track do người dùng đặt một cách mất dữ liệu; header cho phép xuống dòng hoặc ellipsis kèm tooltip hiện đủ tên.
- Ngôn ngữ mặc định UI ở các section ORVIA mới là tiếng Việt nhất quán; các thuật ngữ DAW phổ biến vẫn có thể giữ: Track, MIDI, Grid, Snap, Insert, Solo, Mute, Tempo.

## 4. Không gian và hình khối

| Quy tắc | Chuẩn v1 | Ghi chú |
|---|---:|---|
| Spacing grid | 4px | Các khoảng cách 4 / 8 / 12 / 16 / 24 |
| Panel padding | 12px | Nội dung Rack/Inspector, có thể 8px khi hẹp |
| Row gap | 6–8px | Tránh vùng điều khiển quá sít |
| Panel radius | 8px | Khối riêng, không bo Timeline/clip quá mức |
| Small control radius | 5px | Button/slider/selector |
| Card radius | 8px | Item plugin, nhóm tính năng |
| Panel border | 1px | `color.border`, không thêm viền 3D nhiều lớp |
| Button height | 30–32px | Nút chính/secondary, đủ bấm |
| Compact icon button | 28–32px | Icon 16–18px, touch hitbox ≥28px |
| Slider height | 24–28px | Track/volume/pan |
| Focus ring | 2px | Accent; visible keyboard focus |

- Hạn chế các đường chia ngang dài và gradient nặng; ưu tiên surface separation + padding.
- Dành vùng Timeline tối đa. **Giữ logic responsive toolbar hotfix 4A.2**: >=1550 logical px một hàng cao 78px; màn hình hẹp hai hàng cao 140px có scroll độc lập. Đây là baseline đã được người dùng chốt, **không tự đổi breakpoint ở bước design language**.
- Inspector: default 270px, resize 190–410px; Rack mặc định ~260px và resize theo docking hiện hữu; **không reset saved workspace**.

## 5. Components chuẩn

### 5.1 Nút
- **Primary:** mặt `color.accent`, icon/chữ `color.accentInk`, dùng rất ít (hành động chính như Gán/Thêm khi hợp lệ). Height 32, radius 5.
- **Secondary:** nền `color.raised`, chữ `color.textPrimary`, hover `color.hover`, height 30.
- **Ghost/toolbar:** nền trong suốt hoặc `color.panel`, hover `color.hover`, active chỉ báo một vạch cyan 2px + nền selection; không lạm dụng nền cyan.
- **Destructive:** chỉ đỏ ở icon/text/tình trạng cụ thể, có xác nhận khi xóa dữ liệu quan trọng. Gỡ Insert riêng giữ đúng engine command.
- **Disabled:** độ sáng giảm + lý do trong tooltip ("Track Audio không gán nhạc cụ", "Sampler cần sample"), **không giả vờ nút đã hoạt động**.
- Hover, pressed, selected, focus, disabled phải khác nhau; disabled không nhận click; các nút state Mute/Solo/Record vẫn ưu tiên màu ý nghĩa tương ứng.

### 5.2 Slider / Fader
- Track tối màu `color.raised`, fill Accent, thumb 10–12px; giá trị số luôn nhìn thấy cạnh hoặc trên slider.
- Volume hiển thị **dB**; Pan hiển thị **L / C / R** hoặc giá trị ±; khoảng giá trị/chuyển đổi chỉ dùng engine hiện tại. Không đổi calibration, pan law, clip gain hoặc MIDI velocity.
- Drag phải giữ **Undo merge** hiện tại; double-click/reset chỉ thêm sau khi xác nhận hành vi không phá dữ liệu.

### 5.3 Panel / Accordion
- Header 30–34px, icon nhỏ ở đầu, tiêu đề 13pt; nút thu gọn rõ. Default Inspector: Track controls, Volume/Pan mở; Routing/Sends/Inserts có thể thu gọn; Automation có thể nhớ trạng thái.
- Không tạo vách ngăn giả hoặc panel con 3D. Tiêu đề nhóm phải ngắn, mô tả trợ giúp xuống dòng, không cắt tiếng Việt.
- Giữ cấu trúc thực Inspector–Timeline–Rack và persisted dock layout.

### 5.4 Tabs
- Height 32px, text 12pt; active là chữ sáng + underline accent 2px, hover nền nhẹ. Không dùng hàng nút nổi 3D.
- Tab Rack `Nhạc cụ / Hiệu ứng`; Lower Zone `MixConsole / Key Editor / Memory / Undo` phải phản ánh trạng thái dock thật và keyboard focus.

### 5.5 Track header / Timeline
- Track header gọn, height không cố tăng; tên Track 13pt, màu Track giữ nguyên theo session. Bên trái dùng rail màu Track dày ~3px, đang chọn có viền cyan **không phủ lên waveform**.
- Các nút Mute/Solo/Arm/Monitor icon 16px + tooltip. Không đổi lệnh hay phím tắt.
- Clip waveform/MIDI: giữ hình sóng, dữ liệu note, clip colors, tempo, markers, grid và playhead. Chỉ điều chỉnh typography, selection outline, spacing và opacity sau QA.
- Grid: nét phụ `color.grid` 1px, bar line `color.barLine` 1px; playhead và vùng edit selection có tương phản đủ, không đổi logic snap.

### 5.6 Browser / Instrument & FX Rack (đích áp dụng đầu tiên)
- Header `NHẠC CỤ & HIỆU ỨNG` với hai tabs, tìm kiếm có icon kính lúp. Chọn Track đích ngay dưới tabs, hiển thị đúng Audio/Instrument type.
- Plugin item height 42–52px, tên 12.5–13pt và mô tả 11.5pt; có icon theo nhóm (DSP built-in, Dynamics/EQ, Instrument…), không lấy logo bên thứ ba. Một item = một action `Gán / Thêm / Chỉnh` **thật** và đủ độ rộng.
- FX Rack hiện đúng các slot đã dùng, nhãn On/Bypass và nút xóa; vị trí slot trống phải rõ ràng và không ghi đè plugin hiện hữu.
- Audio Track: ưu tiên UX tab Hiệu ứng; Instrument Track: tab Nhạc cụ. Không chuyển tab âm thầm khi việc này khiến các thay đổi chưa lưu bị mất; không đổi track đang chọn nếu người dùng chưa yêu cầu.
- **Web chỉ mô tả DSP built-in thật**, không ngụ ý host Kontakt/VST3 native trong trình duyệt. Sampler chưa có sample phải nêu rõ tình trạng.

## 6. Nhận diện thương hiệu & biểu tượng

- Header ORVIA: wordmark bằng chữ + tối đa **một** dấu nhấn cyan, không thêm slogan dài trong vùng Workspace. Tên dự án vẫn là thông tin chính.
- Icon mới là nét đơn giản 1.6–1.8px (cỡ 16/18); giữ icons Rust procedural hiện tại `crates/ui-egui/src/icons.rs` và mở rộng theo nguyên tắc clean-room khi cần. Không copy icon/ảnh/âm thanh/font từ Pro Tools, Cubase, Steinberg.
- Font hiện hữu Be Vietnam Pro đã bundling, không đưa thêm tệp font lên repo nếu chưa cần; mọi asset mới phải có giấy phép phù hợp và khai báo ATTRIBUTION theo `AGENTS.md`.
- Màu Record đỏ, Solo vàng, Play/Focus cyan, trạng thái monitoring sẵn sàng xanh lá; trạng thái âm thanh phải có biểu tượng/nhãn chứ không chỉ màu.

## 7. Bản đồ triển khai sang code hiện có

| Token / component v1 | Vị trí hiện có | Nguyên tắc |
|---|---|---|
| Palette Dark `color.*` | `crates/ui-egui/src/theme.rs` → `Tokens::DARK` | Tạo mapping rõ ràng; không đổi LIGHT/SYSTEM bừa bãi |
| Typography | `theme.rs` + `fonts.rs` | Reuse Be Vietnam Pro web / system fonts desktop |
| Button/Icon | `widgets.rs`, `icons.rs` | Giữ hàm xử lý click và icon semantics |
| Transport/tool palette | `toolbar.rs`, `edit_window.rs` | Bảo toàn responsive 78/140px và command wiring |
| Inspector | `panels.rs`, `panels/orvia_automation.rs` | Chỉnh presentation, không thay commands |
| Rack | `panels/orvia_rack.rs` | **Ứng dụng đầu tiên ở 4A.3**, chỉ plugin registry thật |
| Track header/clip | `edit_window.rs` + helpers `theme.rs` | Giữ nguyên clip/note content và màu do user chọn |
| Dock/tabs | `docking.rs` / shared dock | Không reset layout cũ |
| MixConsole | `mix_window.rs` | Áp sau 4A.3 và QA, không đụng engine |

## 8. Kiểm thử và thứ tự áp dụng

**4A.3 (ưu tiên):** Áp design tokens vào **Rack** trên nhánh riêng, UI presentation trước, test chọn Track/Instrument/FX, Add, Bypass, Remove, params, Save/Open. Giữ dark Light/System prefs.
**4A.4:** Track header + selected states, Timeline polish nhẹ; so sánh ảnh trước/sau tại 1366×768, 1440×900, 1738×863.
**4B UI polish:** Bottom panel/tabs, icon, brand wordmark; review accessibility + keyboard focus.
**QA gate:** Mỗi thay đổi code cần Cloudflare deploy đúng SHA + Chrome UAT (audio, MIDI, Save/Open, Undo/Redo); Cargo tests trên runner được đánh dấu độc lập. **Tài liệu v1 này không tạo bản web mới, không có link preview cần xác minh.**

### Definition of Done cho quy chuẩn v1
- [x] Bộ màu và contrast, font/cỡ chữ, spacing, radius, buttons, sliders, panels, tabs, track header, Rack, state được định nghĩa có số cụ thể.
- [x] Chỉ rõ mapping sang Rust/egui và những điều không được sửa.
- [x] Không yêu cầu mua license, tải font hoặc dùng CMD.
- [ ] Người dùng xem và duyệt bảng màu/hướng nhìn trực quan trước khi áp mã rộng.
- [ ] Cloudflare/Chrome nghiệm thu từng triển khai UI sau này.

**Không tự merge main/production, không tự chạy GitHub Actions, không TinyFish, dịch vụ phí, CMD người dùng.**
