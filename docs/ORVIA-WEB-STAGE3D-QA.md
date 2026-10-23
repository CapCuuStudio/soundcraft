# ORVIA 3D — Track Automation QA

Giai đoạn 3D kế thừa 3C (Cloudflare build thành công tại commit 4a8f1cf; Chrome UAT chưa xác nhận). Nhánh: feature/orvia-3d-track-automation.

## Đã triển khai
- Inspector: tùy chọn Volume, Pan, Pan R khi track có, và Mute, dựa trên Mixer và Track thật.
- Hiện Lane trên Timeline thông qua track.view. Về Clips khôi phục dạng hiển thị clip.
- Thêm điểm tại con trỏ Edit qua automation.set_point, chỉnh giá trị từng điểm và xóa riêng điểm bằng automation.clear với khoảng 1 sample.
- Ghi giá trị vào vùng Edit Selection chỉ khi vùng chọn có chiều dài; thông qua automation.write_range.
- Track automation được lưu trong Track::automation, engine hỗ trợ Undo/Redo và Save/Open, không tạo định dạng mới. MIDI CC thuộc một quy trình khác.

## Cổng kiểm thử bắt buộc
1. Cloudflare phải báo Deploy successful đúng commit 3D và Preview URL bất biến.
2. Rust unit tests cần runner thực tế; chưa có kết quả PASS.
3. Chrome: Audio Track → Inspector → Automation; thêm hai điểm Volume ở hai thời điểm, chỉnh giá trị, Undo/Redo; xóa đúng một điểm; Save/Open giữ nguyên điểm; âm lượng thay đổi khi Play.
4. Pan và Mute ở Track có đường audio, thử lane khác và track khác không bị xóa nhầm.
5. Ghi vào Edit Selection, kiểm tra các điểm ngoài vùng vẫn tồn tại.
6. Hồi quy 2C Copy/Split/Delete, 3A MixConsole, 3B Piano Roll+MIDI CC, 3C Instrument/FX Rack, audio và Export.
7. Layout Chrome 1366x768, Inspector scroll được và Timeline không bị che.

Không merge main/production, không GitHub Actions tự phát, TinyFish, phí hay CMD Windows. Build thành công không tương đương UAT âm thanh.