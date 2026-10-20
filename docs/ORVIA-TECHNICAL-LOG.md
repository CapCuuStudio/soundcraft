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
