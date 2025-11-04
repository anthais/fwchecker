# 🔥 Firewall Checker (fwchecker)

Công cụ kiểm tra firewall đơn giản và mạnh mẽ để kiểm tra kết nối mạng qua ping và telnet.

## ✨ Tính năng

- ✅ Kiểm tra kết nối qua **ping**
- ✅ Kiểm tra port mở qua **telnet** (TCP connection)
- ✅ Hỗ trợ **external hosts** (chỉ check từ xa)
- ✅ Hỗ trợ **local hosts** với khả năng tự động SSH và mở port bằng `netcat`
- ✅ Output màu sắc đẹp mắt và dễ đọc
- ✅ Báo cáo tổng kết chi tiết

## 📋 Yêu cầu hệ thống

### Python
- Python 3.6 trở lên

### System Tools
```bash
# Ubuntu/Debian
sudo apt-get install -y sshpass netcat-openbsd

# RHEL/CentOS
sudo yum install -y sshpass nc
```

## 🚀 Cài đặt

### Cách 1: Cài đặt như command-line tool
```bash
cd /workspace/anthais/fwchecker
chmod +x setup.sh
./setup.sh
```

### Cách 2: Chạy trực tiếp
```bash
chmod +x fwchecker.py
./fwchecker.py --config=bcs-firewall.conf
```

### Cách 3: Sử dụng Python
```bash
python3 fwchecker.py --config=bcs-firewall.conf
```

## 📝 Cách sử dụng

### Cú pháp cơ bản
```bash
fwchecker --config=<config-file>
# hoặc
fwchecker -c <config-file>
```

### Ví dụ
```bash
fwchecker --config=bcs-firewall.conf
```

## ⚙️ File cấu hình

### Format
```
type,ip,port,method,ssh_user,ssh_password
```

### Các tham số

| Tham số | Giá trị | Mô tả |
|---------|---------|-------|
| type | `external` \| `local` | Loại host |
| ip | IP address hoặc hostname | Địa chỉ cần kiểm tra |
| port | số (0 cho ping-only) | Port cần kiểm tra |
| method | `ping` \| `telnet` \| `both` | Phương thức kiểm tra |
| ssh_user | string | Username SSH (bắt buộc cho local) |
| ssh_password | string | Password SSH (bắt buộc cho local) |

### Ví dụ config

```conf
# External hosts - chỉ kiểm tra từ xa
external,8.8.8.8,0,ping
external,google.com,443,telnet
external,github.com,443,both

# Local hosts - có thể SSH vào để mở port
local,192.168.1.10,8080,telnet,admin,secretpass
local,192.168.1.20,3306,both,root,mysql123
```

## 🎯 Cách hoạt động

### External Hosts
1. Với method `ping`: Thực hiện ping đến host
2. Với method `telnet`: Kiểm tra TCP connection đến host:port
3. Với method `both`: Thực hiện cả ping và telnet

### Local Hosts
1. Giống như external hosts
2. **Thêm tính năng đặc biệt**: Nếu port chưa mở, tool sẽ:
   - SSH vào host với credentials được cung cấp
   - Chạy lệnh `nc -l -p <port>` để mở port
   - Thực hiện kiểm tra telnet

Điều này hữu ích khi bạn cần test firewall nhưng chưa có service nào chạy trên port đó.

## 📊 Output mẫu

```
✓ Đã load 4 host(s) từ config

============================================================
  FIREWALL CHECKER - BẮT ĐẦU KIỂM TRA
============================================================

Checking 8.8.8.8:0 (external)
  ✓ PING: OK

Checking google.com:443 (external)
  ✓ TELNET: OK

Checking 192.168.1.10:8080 (local)
  ✓ SSH/NC: Đã mở port thành công
  ✓ PING: OK
  ✓ TELNET: OK

============================================================
  TỔNG KẾT KẾT QUẢ
============================================================

✓ PASS - 8.8.8.8:0
✓ PASS - google.com:443
✓ PASS - 192.168.1.10:8080

Tổng số: 3 | Passed: 3 | Failed: 0
============================================================
```

## 🔒 Bảo mật

**⚠️ CHÚ Ý**: File config chứa password dạng plaintext. Hãy:
- Đặt quyền file: `chmod 600 bcs-firewall.conf`
- Không commit file config có password thật vào git
- Cân nhắc sử dụng SSH key thay vì password trong production

## 🐛 Troubleshooting

### Lỗi: "sshpass: command not found"
```bash
sudo apt-get install sshpass
```

### Lỗi: "nc: command not found"
```bash
sudo apt-get install netcat-openbsd
```

### SSH không hoạt động với local hosts
- Kiểm tra SSH credentials
- Đảm bảo SSH server đang chạy trên remote host
- Kiểm tra firewall cho phép SSH (port 22)
- Thử SSH thủ công: `ssh user@host`

### Port vẫn không mở sau khi SSH
- Kiểm tra user có quyền bind port (ports < 1024 cần root)
- Kiểm tra port đã được sử dụng chưa: `netstat -tuln | grep <port>`
- Firewall trên remote host có chặn không

## 📄 License

MIT License

## 👤 Author

Created for checking firewall connectivity in network infrastructure.

## 🤝 Contributing

Contributions, issues và feature requests đều được welcome!

