# ORVIA — Cloudflare Pages (ứng dụng DAW thật)

**Trạng thái:** MỚI CHUẨN BỊ SOURCE BUILD, CHƯA CÓ DEPLOY CLOUDFARE ĐƯỢC XÁC MINH.
**Repository:** https://github.com/CapCuuStudio/soundcraft
**Nhánh thử nghiệm:** `feature/orvia-web-bootstrap`
**Pull request:** https://github.com/CapCuuStudio/soundcraft/pull/1
**Không sửa production:** `lunor-web.pages.dev/orvia.html` chỉ là trang giới thiệu; không thay hoặc ghi đè lên site này.

## Thiết lập một lần trên Cloudflare (qua giao diện web, không CMD)

1. Vào https://dash.cloudflare.com/ — **Workers & Pages** → **Create application** → **Pages** → **Connect to Git** (tên mục có thể thay đổi).
2. Kết nối GitHub và cấp quyền truy cập repo **CapCuuStudio/soundcraft** (nếu repo thuộc tổ chức, cho phép Cloudflare GitHub App truy cập tổ chức).
3. **Create a new Cloudflare Pages project riêng** (gợi ý `orvia-daw-preview` nếu tên còn trống), KHÔNG dùng lại `lunor-web` hay site đang chạy.
4. Chọn **Production branch = `feature/orvia-web-bootstrap`** *chỉ trong dự án Pages thử nghiệm riêng*. Việc chọn branch này không merge `main` trên GitHub hay triển khai lên website LUNOR đang hoạt động.
5. Framework preset: **None**. Root directory: **để trống (repo root)**. Build command: **`bash scripts/cloudflare-pages-build.sh`**. Build output directory: **`dist/web`**.
6. Chọn build system v3 nếu được hỏi; không cần secrets, không cần DNS riêng và không cần GitHub Actions. Ấn **Save and Deploy**, theo dõi Cloudflare Build Logs đến khi xuất hiện **Success**.
7. Cloudflare sẽ tự cấp URL dạng `https://<tên-project-thực-tế>.pages.dev`; **không suy đoán trước URL**. Mở nó trên Chrome: app phải tải được WebAssembly, xuất hiện giao diện Edit/Mix, bấm Play sau tương tác để kích hoạt WebAudio. Các chức năng nhập Audio/MIDI và Save/Export phải kiểm thử thực tế, không lấy trạng thái build làm bằng chứng âm thanh.

## Đầu vào và cấu trúc

- `apps/soundcraft-web/index.html`: Trunk HTML entry mang nhãn ORVIA.
- `apps/soundcraft-web/Trunk.toml`: sinh artifact tại `dist/web` từ **repo root**.
- `scripts/cloudflare-pages-build.sh`: tự cài Rust 1.95.0, wasm32-unknown-unknown, Trunk v0.21.14 x86_64 Linux (SHA256 xác thực); biên dịch Rust WASM; kiểm tra có `index.html` và file `.wasm`. Từ chối deploy nếu build lỗi / thiếu WASM / asset quá 25 MiB.
- Không có HTML giả lập, iframe ứng dụng khác hay redirect từ trang giới thiệu.

## Rủi ro đã xác định — KHÔNG GIẤU

- Cloudflare Pages Free có giới hạn 20 phút một build và 25 MiB mỗi file. Workspace Rust/egui/wgpu có thể build lâu hoặc .wasm lớn hơn 25 MiB; **chưa kiểm nghiệm giới hạn thật**. Build không đạt tức là CHƯA CÓ web DAW trên Cloudflare. Nếu vậy cần lấy Cloudflare build log và thiết kế phương án build WASM khác **được chủ dự án duyệt**; không thay bằng màn hình giả.
- Chính Cloudflare sẽ báo lỗi nếu thiếu quyền GitHub App, rustup cargo dependencies, wasm-bindgen/trunk hoặc thiếu RAM. Không tự khẳng định thành công khi chưa có log.
- Chưa đo runtime WebAudio trên Chrome/Windows, import/export hay plugin. Người dùng cần bấm một lần tương tác để AudioContext `resume`.
- SoundCraft pre-alpha, UI vẫn còn một số nhãn gốc và giấy phép ArtCraft trademark không cho phép sử dụng dấu hiệu thương hiệu làm ORVIA. Không công bố bản phát hành/chính thức đến khi audit nhận diện.
- **Không có quyền thao tác trực tiếp tài khoản Cloudflare trong phiên ChatGPT hiện tại.** Kết nối GitHub→Cloudflare phải được chủ tài khoản cấp quyền một lần qua dashboard; GitHub connector không thể tạo Cloudflare Pages project.
- Không dùng TinyFish, không CMD trên máy người dùng, không GitHub Actions, không mua dịch vụ, không merge PR.

## Trạng thái kiểm thử (20/10/2026)

- Đã kiểm tra source và tạo build script trên nhánh thử, chưa thực hiện `trunk build` trên runner Cloudflare, chưa có link live.
- Bước tiếp theo duy nhất: chủ dự án kết nối repo với một Cloudflare Pages project mới bằng các thiết lập phía trên. Sau đó xác minh log build/đường link thực tế và sửa lỗi theo log nếu có.
