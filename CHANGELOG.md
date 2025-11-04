# Changelog

## [2.0.0] - 2025-11-04

### 🎉 Major Changes: Chuyển sang Bash

- **BREAKING**: Viết lại hoàn toàn bằng Bash thay vì Python
- Lý do: Nhẹ hơn, nhanh hơn, không cần dependencies Python
- Tương thích ngược: Config file format giữ nguyên

### ✨ Cải tiến

- ⚡ Khởi động nhanh hơn (không cần Python interpreter)
- 📦 Không cần cài đặt Python packages
- 🔧 Code dễ đọc và customize hơn
- 🐧 100% native Linux tools

### 🗂️ Files thay đổi

- `fwchecker`: Script Bash mới (thay thế fwchecker.py)
- `fwchecker.py.bak`: Backup phiên bản Python cũ
- `setup.sh`: Cập nhật cho Bash version
- `README.md`: Cập nhật hướng dẫn
- `REQUIREMENTS.txt`: Liệt kê system tools thay vì Python packages

### 📝 Migration

Nếu đang dùng Python version:
```bash
# Backup đã tự động tạo
ls -la fwchecker.py.bak

# Chạy Bash version (cú pháp giống hệt)
./fwchecker --config=bcs-firewall.conf
```

Config file không cần thay đổi gì!

---

## [1.0.0] - 2025-11-04

### ✨ Initial Release (Python)

- Kiểm tra firewall qua ping và telnet
- Hỗ trợ external và local hosts
- Tự động SSH và mở port với netcat
- Output màu sắc đẹp mắt
- File config đơn giản (CSV format)

