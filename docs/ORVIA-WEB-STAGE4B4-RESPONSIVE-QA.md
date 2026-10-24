# ORVIA 4B.4 — Tổng kiểm tra responsive và hồi quy UI

**Ngày:** 24/10/2026. **Căn cứ:** bản 4B.3 `feature/orvia-4b3-brand-control-states` đã được chủ dự án xác nhận “xong”; mã `d7dd37d` Cloudflare PASS; docs-only `6cc460e`. **Môi trường cần nghiệm thu:** Web Cloudflare Preview nhánh 4B.4, Chrome/Edge.

## 1. Phát hiện và bản sửa thực hiện

| Mức | Bộ phận | Vấn đề trong mã 4B.3 | Sửa 4B.4 | Phạm vi không được đổi |
|---|---|---|---|---|
| P1 | Lower Zone → MixConsole | Panel min-height 180 logical px nhưng nội dung strip cao hơn; `ScrollArea::horizontal` chỉ xử lý chiều ngang, Fader/Pan/Routing phía dưới có thể không cuộn tới được | `ScrollArea::both`, thanh cuộn hiện khi cần, giữ header và toàn bộ channel engine commands | volume dB, pan, meters, sends, bus, undo-merge |
| P1 | Studio Rack | Rack chỉ có `ScrollArea::vertical` trong danh sách plugin; ở Browser min-height 160, Track chọn/Insert/chi tiết phía trên nằm ngoài cuộn | Đưa cuộn dọc ra `orvia_browser` bao toàn bộ Rack; bỏ ScrollArea dọc lồng trong catalog, giữ render plugin và click handlers | chọn Instrument/FX, track routing, insert, sampler, giới hạn Web VST3 |
| Prevent | Responsive QA | Cloudflare build không chạy kiểm tra source contract để bảo vệ 4A.2–4B.3 trước Rust WASM compile | `tests/orvia-4b4-responsive-contract.mjs` chạy bằng Node built-in đầu `scripts/cloudflare-pages-build.sh` | trunk build, project session, web runtime, production |

## 2. Kiểm tra tự động

- Node kiểm tra 7 nhóm: Mixer cả hai trục/cuộn khi nhỏ; Studio Rack một vùng cuộn dọc bao toàn bộ thao tác; Inspector chiều rộng và cuộn; Key Editor min 420×260, header 56 và lệnh MIDI; Toolbar 78/140px breakpoint 1550 và transport; Timeline/Lower Zone docking commands; ORVIA mark/control states.
- Node chỉ dùng thư viện chuẩn, **không thay thế test Chrome**. Nếu một contract nguồn bị xóa, script exit nonzero làm Cloudflare build dừng sớm; chỉ khi PASS mới biên dịch Rust/WASM.
- Đã kiểm tra mã hiện tại qua GitHub source (8 nhóm predicate, tất cả khớp trước PR). Kết quả test Node trên Cloudflare và deploy đúng HEAD phải được đối chiếu sau.
- Các Rust unit tests có từ 4A.1–4B.3 **không được tính PASS** trừ khi có báo cáo chạy Cargo tests độc lập. Cloudflare Rust/WASM build PASS chỉ xác nhận biên dịch/bundle.

## 3. Ma trận manual UAT / Chrome — chưa tự động xác minh

| Kích thước logical px | Thanh Toolbar | Lower Zone / Splitter | Mục tiêu |
|---|---|---|---|
| 1024×768 | 2 hàng, scroll ngang riêng | Mixer hai trục + Rack dọc | Stress min width |
| 1366×768 | 2 hàng, 140px | MixConsole cao/thấp, Key Editor | Laptop 14” |
| 1440×900 | 2 hàng, 140px | Inspector 190–410px, Rack 200–520px | Trung bình |
| 1738×863 | 1 hàng, 78px | Timeline không mất khi mở dưới | Wide |
| 1920×1080 | 1 hàng, 78px | Studio Rack rất dài/10 Inserts | Wide/large |

**Các bước cần kiểm ở bản Preview 4B.4:**
1. Mở file/session demo, Play/Stop/Record/Loop, Undo/Redo, Save/Open; xác nhận meters phát tín hiệu và không thay đổi Audio/MIDI.
2. Resize Browser xuống ~160px: tìm plugin, chọn Track, mở Insert Rack, mở chi tiết, gán và Bypass/Gỡ plugin; cuộn để truy cập tất cả phần trên/dưới.
3. Resize Lower Zone xuống ~180px: chuyển MixConsole, cuộn xuống Fader/Pan/Sends/Output, cuộn ngang >4 track, chỉnh M/S/volume/Pan rồi Undo; kéo lại 300–450px.
4. Mở Key Editor MIDI với tên Clip dài, ở 420px/260px: title và Quantize/Vel±10/Legato không đè nhau; cuộn nốt/velocity/controller lane, vẽ/chọn/xóa nốt, giữ clip colors.
5. Inspector bên trái co 190px và mở 270px/410px; Timeline track headers, Clip select/drag/snap/playhead/grid không bị phủ.
6. Toolbar ở 1366px/1738px, hover/focus/selected cho Play, Stop, Record, Loop, tool/edit modes; tab Lower Zone và nút Reset Layout dùng được. Reset chỉ khi người dùng chủ động bấm.
7. Dark/Light/System; zoom trình duyệt 90/100/125%, kiểm vị trí con trỏ/Focus; thử tên Track/Clip dài và tiếng Việt dấu.
8. Studio Rack bản Web **không** tuyên bố Kontakt/VST3 native: những xử lý này vẫn thuộc host Desktop; không tạo dummy audio.
9. Chụp screenshot từng viewport + so màu/độ cao nếu có nguồn chạy Chrome; nghe trực tiếp/ghi âm/thử xuất MIDI nếu có thiết bị kiểm thử.

## 4. Release gate

- [x] Đọc nhật ký kỹ thuật, source trước sửa, kế thừa 4B.3 đã chốt.
- [x] Sửa đúng hai nguồn P1 theo code review, không chạm logic âm thanh/MIDI.
- [x] Thêm Node smoke và tích hợp Cloudflare build script.
- [x] Kiểm tra cấu trúc source sau sửa: engine call count không đổi trong hai file P1.
- [ ] Cloudflare source-contract **và** Trunk Rust WASM `completed/success` trên code HEAD 4B.4.
- [ ] Chrome UAT/ảnh thực ở các viewport trên, A/V playback/Save/Open/MIDI.
- [ ] Chủ dự án nghiệm thu bản Preview 4B.4.

**Tình trạng chưa xác minh phải giữ HOLD, không tự điền PASS.** PR Draft tách 4B.3; không merge production; không GitHub Actions thủ công, phí, TinyFish hay CMD Windows.
