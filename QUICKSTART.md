# 🚀 Quick Start Guide

## Cài đặt nhanh (30 giây)

```bash
# 1. Clone hoặc cd vào thư mục
cd /workspace/anthais/fwchecker

# 2. Chạy setup script
chmod +x setup.sh
./setup.sh

# 3. Chỉnh sửa config
vim bcs-firewall.conf

# 4. Chạy kiểm tra
fwchecker --config=bcs-firewall.conf
```

## Hoặc chạy trực tiếp (không cài đặt)

```bash
chmod +x fwchecker
./fwchecker --config=bcs-firewall.conf
```

## Format config đơn giản

```
type,ip,port,method,ssh_user,ssh_password
```

### Ví dụ cơ bản

```conf
# Ping Google DNS
external,8.8.8.8,0,ping

# Kiểm tra HTTPS qua telnet
external,google.com,443,telnet

# Kiểm tra HTTPS qua curl
external,google.com,443,curl

# Kết hợp nhiều loại check
external,github.com,443,ping+telnet
external,api.example.com,443,ping+curl
external,web.example.com,443,ping+telnet+curl
```

### Ví dụ với local host

```conf
# Local server với SSH
local,192.168.1.10,8080,telnet,admin,password123
local,192.168.1.20,80,curl,webadmin,password456
local,192.168.1.30,3306,ping+telnet,dbadmin,password789
```

## Test nhanh

```bash
# Test với config mẫu (external hosts only)
./fwchecker --config=bcs-firewall.conf

# Xem help
./fwchecker --help
```

## Kết quả mẫu

```
✓ Đã load 2 host(s) từ config

Checking 8.8.8.8:0 (external)
  ✓ PING: OK

Checking google.com:443 (external)
  ✓ TELNET: OK

Tổng số: 2 | Passed: 2 | Failed: 0
```

---

Xem chi tiết: [README.md](README.md)

