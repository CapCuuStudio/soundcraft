# ORVIA — Nhật ký kỹ thuật (nhánh thử nghiệm)

**Ngày khởi tạo:** 2026-10-20 (Asia/Ho_Chi_Minh)
**Sản phẩm:** ORVIA — Digital Audio Workstation, thuộc LUNOR MUSIC
**Repository:** https://github.com/CapCuuStudio/soundcraft
**Cơ sở:** SoundCraft, upstream https://github.com/storytold/soundcraft
**Base tại thời điểm khởi tạo:** `main` — `3a28a657692c1b98b28c276d8f61df9471074f0f`
**Nhánh:** `feature/orvia-web-bootstrap`
**Website giới thiệu:** https://lunor-web.pages.dev/orvia.html (không phải DAW đang chạy)

## Phải đọc trước mỗi lượt phát triển

1. Nhật ký kỹ thuật chung LUNOR MUSIC trên Google Drive (nguồn quyết định dự án); đồng thời đọc tài liệu này.
2. `AGENTS.md`, `ROADMAP.md`, `docs/gaps.md` của SoundCraft và trạng thái GitHub/preview thực tế.
3. Trước khi sửa, đối chiếu trạng thái branch/PR; không tự merge `main`, không deploy production, không dùng TinyFish, không phí mới, không yêu cầu CMD trên máy người dùng.
4. Mỗi thay đổi phải có nguyên nhân, phạm vi, nguồn đối chiếu, bằng chứng test và việc tiếp theo.

## Khảo sát baseline — 20/10/2026

- Ba nhánh `main`, `craft-upstream`, `lunor-dev` cùng trỏ tới commit `3a28a657...`; chưa có thay đổi ORVIA.
- Workspace Rust với desktop (`apps/soundcraft`), web WASM (`apps/soundcraft-web`) và CLI; UI chia sẻ qua `crates/ui-egui`.
- Dự án gốc tự đánh giá **pre-alpha** (xem `ROADMAP.md` và `docs/gaps.md`), không được tuyên bố là sản phẩm hoàn chỉnh.
- `apps/soundcraft-web/index.html` dùng WebAssembly/Trunk, WebAudio được resume sau tương tác của người dùng. **Chưa kiểm thử runtime của ORVIA**.
- Thương hiệu ArtCraft trong `docs/brand` có giấy phép riêng và không được dùng như dấu nhận diện của ORVIA. Không sao chép logo/trademark vào sản phẩm.
- Phải giữ nguyên cơ chế session `.scraft` / `.scraftz` và tên crate nội bộ trong giai đoạn đầu để tránh phá tương thích trước khi có test.

## Phạm vi mốc 0 — Bootstrap ORVIA Web

- Tạo nhánh tách biệt, đưa nhãn ORVIA vào tiêu đề trang, trạng thái tải và bảng About/menu; ghi nhận nguồn gốc SoundCraft bằng văn bản, không ngụ ý ArtCraft bảo trợ ORVIA.
- Giữ nguyên audio engine, MIDI, định dạng session, playback, export, plugin host và đóng gói hệ thống; không đổi các ID HTML/rust nội bộ khi chưa cần.
- Chưa thêm logo thương hiệu mới; không dùng favicon SoundCraft trong ORVIA Web.
- **Không coi thay nhãn là bản DAW chạy được.** Chưa có build WASM, kiểm thử âm thanh hay preview ORVIA công khai.

## Chặng tiếp theo, theo thứ tự

1. Kiểm tra khả năng build WebAssembly và luồng WebAudio thực, import/export, MIDI, mixer.
2. Thiết kế lại thương hiệu người dùng và thay tất cả tài sản mang dấu ArtCraft trước khi phát hành ORVIA; giữ thông tin giấy phép/ghi công hợp lệ.
3. Xuất Web Preview của **ứng dụng DAW thật** để chủ dự án trực tiếp test, không phải mô phỏng HTML.
4. Chỉ sau nghiệm thu web mới cân nhắc desktop Windows/đóng gói.

## Kiểm thử / chưa xác minh

- Đã kiểm tra tệp nguồn qua GitHub connector; **chưa biên dịch hoặc chạy** ORVIA trên Chrome.
- Chưa có Cloudflare Preview hay file cài đặt cho ORVIA; mọi tuyên bố ngược lại là không chính xác.
- Chưa kiểm tra toàn bộ giấy phép tài sản và logo nhúng trong ứng dụng desktop; không phát hành bản đổi thương hiệu trước khi rà soát.

## Triển khai Cloudflare Pages — 20/10/2026 (chờ kết nối Git)

**Yêu cầu:** Chủ dự án muốn URL Cloudflare Pages chạy ORVIA DAW thật.
**Đã làm:** thêm `scripts/cloudflare-pages-build.sh` (Rust 1.95.0 + wasm32 + Trunk v0.21.14, kiểm SHA256 và output WASM) và `docs/ORVIA-CLOUDFLARE-PAGES.md` (hướng dẫn cấu hình dự án Pages thử nghiệm tách biệt). Cloudflare đặt repo root, build command `bash scripts/cloudflare-pages-build.sh`, output `dist/web`, nhánh `feature/orvia-web-bootstrap`. Không khởi chạy GitHub Actions.
**Kiểm thử:** Kiểm tra nội dung commit qua GitHub connector, script đối chiếu Bash syntax cơ bản trong môi trường độc lập; chưa biên dịch Rust/WASM. Cloudflare Free có giới hạn 20 phút/build và 25 MiB/asset nên build còn có thể thất bại. Chưa có Cloudflare project/URL được xác minh; không được báo đã deploy.
**Vướng mắc:** Không có quyền truy cập tài khoản Cloudflare trong kết nối ChatGPT hiện tại; Cloudflare cần chủ tài khoản thực hiện GitHub OAuth qua dashboard một lần. Không sử dụng TinyFish.
**Việc tiếp theo duy nhất:** Chủ dự án mở Cloudflare Workers & Pages, tạo Pages project mới, Connect to Git → `CapCuuStudio/soundcraft`, nhập cấu hình trong `docs/ORVIA-CLOUDFLARE-PAGES.md`, xem log thành công/thất bại; sau đó kiểm thử ORVIA trực tiếp tại URL thực. Không chạm site LUNOR chính.

## Giai đoạn UI kiểu Cubase — Stage 1 (20/10/2026)

**Yêu cầu:** Chủ dự án thích bố cục Cubase hơn Pro Tools; đồng ý làm và hỏi chia giai đoạn. Chọn 4 giai đoạn, ghi trong `docs/ORVIA-CUBASE-UI-PLAN.md`. Tuyệt đối không dùng asset/mã nguồn độc quyền Cubase.
**Đã đọc trước khi làm:** Nhật ký LUNOR MUSIC Google Drive, file ORVIA này và AGENTS.md, ROADMAP.md, docs/gaps.md; kiểm tra PR #1 và Cloudflare Preview Stage 0.
**Đã triển khai source trên branch:** `feature/orvia-cubase-layout-stage1` (base `feature/orvia-web-bootstrap`), Draft PR #2 https://github.com/CapCuuStudio/soundcraft/pull/2. Thêm panel Inspector hiển thị dữ liệu track đang chọn, nút Open Mixer/Open MIDI Editor/Add Track gọi lệnh có sẵn; workspace có tab Inspector/Tracks bên trái 224px, Timeline thực giữa, Media & Clips bên phải 256px và MIDI dock mở/tắt ở dưới; thứ tự toolbar ưu tiên Transport/Counter/Tempo; thêm 2 Rust unit tests kiểm tra workspace edit và mix độc lập.
**Không thay đổi:** Engine/DSP/audio callback/MIDI events/import-export/session file, Cloudflare build script, nhánh main/production hay site giới thiệu.
**QA thật:** Đã so GitHub compare về danh sách 4 file; chưa có test Rust/WASM build được xác minh, chưa có Cloudflare bot xác nhận PR #2, chưa nghe thử audio trên Chrome. Không được gọi là phiên bản UI hoàn thiện.
**Việc tiếp theo:** Chờ Cloudflare Pages deploy preview commit PR #2. Nếu build fail, sửa theo log. Khi thành công, đưa link immutable cho chủ dự án thử và chờ nghiệm thu trước Stage 2.

## Giai đoạn 1 — Project Zone đúng Cubase (20/10/2026; nhánh thử PR #4)

**Yêu cầu:** Người dùng duyệt bố cục Cubase gồm Inspector bên trái, track headers đi kèm timeline ở trung tâm, VST/Media bên phải, Lower Zone thu gọn được và nhấp đúp MIDI mở Key Editor.
**Đã đọc trước khi sửa:** Nhật ký kỹ thuật LUNOR MUSIC Google Drive; ORVIA repo log, Cubase UI plan, SoundCraft AGENTS.md, ROADMAP/docs/gaps các lượt trước, mã docking/ui/edit_window và tình trạng Draft PR #3 đã Deploy successful commit `d4dd11e`.
**Base:** `feature/orvia-cubase-stage1-rework` tại `d4dd11ebcc3ac69eb46eec5a6bc6db755245bc24`.
**Branch:** `feature/orvia-cubase-project-zone` (Draft PR #4 https://github.com/CapCuuStudio/soundcraft/pull/4).
**Phạm vi triển khai:** `crates/ui-egui/src/docking.rs` bỏ Panel::Tracks khỏi default Edit layout, giữ các track headers trong timeline thực, loại Lower Zone mặc định; reset trở về project layout chuẩn. `crates/ui-egui/src/lib.rs` đặt defaults không ép Tracks/MIDI, thêm `orvia_layout_version=2` và migration cho UI state từ nhánh cũ (không động dữ liệu Session). `crates/ui-egui/src/edit_window.rs` thay footer toggle MixConsole/Key Editor, hai trạng thái không tranh nhau, Reset Layout và double-click clip MIDI gọi lệnh docking mở piano roll. Thêm Rust unit test Lower Zone mở/đóng và khung layout mặc định.
**Chưa thay đổi:** `main`, website LUNOR production, engine audio, MIDI data, plugin host, export/session, Cloudflare build script. Không dùng ảnh/mã có bản quyền của Steinberg/Cubase.
**QA thực tế:** Source đã ghi qua GitHub connector; **chưa xác nhận Cargo test hoặc Cloudflare WASM build cho HEAD PR #4; chưa có URL immutable bản này**. Không được tuyên bố đã chạy trên web/đã đạt Cubase hoặc chất lượng audio chỉ từ source. Kiểm tra sau build: timeline toàn chiều cao, Inspector + track headers một cột, bật/tắt Lower MixConsole/Key Editor và kéo splitter, nhấp đúp MIDI, phát âm thanh, save/undo.
**Việc tiếp theo:** Theo dõi bình luận Cloudflare Pages bot trên PR #4; nếu failed phải đọc log và sửa; nếu successful đúng HEAD mới gửi đường dẫn preview cho người dùng kiểm thử.
