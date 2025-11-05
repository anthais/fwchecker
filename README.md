# 🔥 Firewall Checker (fwchecker) v2.0

Công cụ kiểm tra firewall mạnh mẽ với khả năng check từ nhiều hosts khác nhau qua SSH.

## ✨ Tính năng v2.0

- ✅ **Host-based checking**: Định nghĩa hosts và thực hiện checks từ các hosts đó
- ✅ **Local execution**: Chạy checks trực tiếp từ máy local
- ✅ **Remote execution**: SSH vào remote hosts để chạy checks
- ✅ **Passwordless SSH**: Hỗ trợ SSH key authentication
- ✅ **Auto port opening**: Tự động mở ports bằng netcat khi cần
- ✅ **Flexible checks**: ping, telnet, curl với full command support
- ✅ **Pretty output**: Với newline directives để phân tách groups
- ✅ **Multiple formats**: pretty, csv, tsv, table

## 📋 Yêu cầu hệ thống

### Bash
- Bash 4.0+ (khuyến nghị, Bash 3.x cũng có thể hoạt động)
- Có sẵn trên hầu hết các hệ thống Linux

### System Tools
```bash
# Ubuntu/Debian
sudo apt-get install -y netcat-openbsd iputils-ping curl

# Optional: for password-based SSH
sudo apt-get install -y sshpass

# RHEL/CentOS
sudo yum install -y nc iputils curl
sudo yum install -y sshpass  # optional

# Fedora
sudo dnf install -y nc iputils curl
sudo dnf install -y sshpass  # optional
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
chmod +x fwchecker
./fwchecker --config=example.conf
```

## 📝 Cách sử dụng

### Cú pháp cơ bản
```bash
fwchecker --config=<config-file>
# hoặc
fwchecker -c <config-file>

# Với output format khác
fwchecker -c example.conf --format=csv

# Verbose mode
fwchecker -c example.conf -v
```

## ⚙️ File cấu hình v2.0

### Format

Config file được chia thành 2 phần chính:

#### 1. HOST Definitions - Định nghĩa hosts

```ini
[HOST:name]
type=local|remote|external
ip=<ip_or_hostname>
port=<ssh_port>              # default: 22
user=<ssh_username>          # optional
password=<ssh_password>      # optional
openServicePorts=<port1,port2,...>  # optional
```

**Các loại type:**
- `local`: Chạy lệnh check trực tiếp từ máy local
- `remote`: SSH vào host này để chạy lệnh check
- `external`: Target để check (không SSH vào)

**openServicePorts**: Danh sách các ports chưa có service chạy. Tool sẽ tự động SSH vào và mở port bằng `nc -l -p <port>` trước khi check.

#### 2. CHECK Definitions - Định nghĩa checks

```ini
[CHECK:hostname]
cmd=ping <target>
cmd=telnet <target> <port>
cmd=curl <full_curl_command>
newline
```

**Commands:**
- `ping <target>`: Ping đến target
- `telnet <target> <port>`: Check TCP connection
- `curl ...`: Full curl command, nhập gì thì chạy đó
- `newline`: In dòng trống để phân tách groups

### Ví dụ đầy đủ

```ini
# ==============================================================================
# HOST DEFINITIONS
# ==============================================================================

# Local machine
[HOST:local]
type=local

# Remote app servers
[HOST:app1]
type=remote
ip=192.168.1.10
port=22
user=admin
password=admin123
openServicePorts=80,443    # Auto-open these ports

[HOST:app2]
type=remote
ip=192.168.1.20
# No user/password = use SSH key

# Database servers
[HOST:db1]
type=remote
ip=192.168.1.30
user=dbadmin
# No password = use SSH key

# External targets
[HOST:dns]
type=external
ip=8.8.8.8

[HOST:google]
type=external
ip=google.com

[HOST:api]
type=external
ip=api.example.com


# ==============================================================================
# CHECK RULES
# ==============================================================================

# Checks FROM local machine
[CHECK:local]
# Check app servers
cmd=ping app1
cmd=telnet app1 22
cmd=telnet app1 80     # Will auto-open with nc
cmd=telnet app1 443    # Will auto-open with nc
newline

cmd=ping app2
cmd=telnet app2 22
newline

# Check databases
cmd=ping db1
cmd=telnet db1 3306
newline


# Checks FROM app1 server
[CHECK:app1]
# Check external connectivity
cmd=ping dns
cmd=telnet dns 53
cmd=curl -s https://google.com
cmd=curl -X POST https://api.example.com/v1/data -d '{"test":"data"}' -H "Content-Type: application/json"
newline

# Check internal connectivity
cmd=ping db1
cmd=telnet db1 3306
newline

cmd=ping app2
cmd=telnet app2 22
newline


# Checks FROM app2 server
[CHECK:app2]
cmd=ping dns
cmd=curl -s -o /dev/null -w "%{http_code}" https://google.com
newline

cmd=ping db1
cmd=telnet db1 3306
newline


# Checks FROM db1 server
[CHECK:db1]
cmd=ping dns
cmd=telnet dns 53
newline

cmd=ping app1
cmd=telnet app1 22
newline
```

## 🎯 Cách hoạt động

### Flow

1. **Parse config**: Đọc HOST definitions và CHECK rules
2. **Group by source host**: Nhóm các checks theo source host
3. **Execute checks**:
   - Nếu `type=local`: Chạy lệnh trực tiếp
   - Nếu `type=remote`: SSH vào host và chạy lệnh từ đó
4. **Auto-open ports**: Nếu target có `openServicePorts`, tự động mở ports trước khi check
5. **Collect results**: Tổng hợp kết quả theo format

### Hostname Resolution

- Nếu command dùng hostname (vd: `ping app1`), tool sẽ resolve từ HOST definitions
- Nếu không tìm thấy, sẽ dùng DNS resolution mặc định
- Có thể dùng IP trực tiếp (vd: `telnet 192.168.1.10 22`)

### SSH Authentication

Tool hỗ trợ 3 cách:
1. **Password**: Cung cấp `user=` và `password=` (cần `sshpass`)
2. **SSH Key**: Chỉ cung cấp `user=`, không có `password=`
3. **SSH Config**: Không cung cấp `user=` và `password=`, SSH sẽ dùng `~/.ssh/config`

## 📊 Output Formats

### Pretty (default)
```
✓ Loaded 8 host(s) and 4 check section(s) from config

============================================================
  FIREWALL CHECKER - STARTING CHECKS
============================================================

Checking from: local (local)
  ✓ PING local -> app1
  ✓ TELNET local -> app1:22
  ✓ TELNET local -> app1:80

  ✓ PING local -> app2
  ✓ TELNET local -> app2:22

Checking from: app1 (remote)
  ✓ PING app1 -> dns
  ✓ CURL app1 -> https://google.com

  ✓ TELNET app1 -> db1:3306

============================================================
  SUMMARY
============================================================

✓ PASS - local -> app1 (ping)
✓ PASS - local -> app1:22 (telnet)
...

Total: 10 | Passed: 10 | Failed: 0
```

### CSV Format
```bash
fwchecker -c example.conf -f csv > report.csv
```

Output:
```csv
Source,Target,Method,Detail,Result
local,app1,ping,"-",PASS
local,app1:22,telnet,"Connection successful",PASS
app1,dns,ping,"-",PASS
app1,https://google.com,curl,"Success",PASS
```

### Table Format
```bash
fwchecker -c example.conf -f table
```

### TSV Format
```bash
fwchecker -c example.conf -f tsv
```

## 🔒 Bảo mật

**⚠️ CHÚ Ý**: 

1. File config có thể chứa passwords dạng plaintext
2. Đặt quyền file: `chmod 600 example.conf`
3. Không commit file config có password thật vào git
4. **Khuyến nghị**: Sử dụng SSH key thay vì password trong production

## 🆚 So sánh v1 vs v2

| Feature | v1 | v2 |
|---------|----|----|
| Config format | CSV flat file | Structured INI-style |
| Check source | Local only | Multiple hosts |
| Remote execution | SSH to open ports only | Full remote execution |
| Grouping | N/A | By source host |
| Curl support | Yes | Yes, full command |
| SSH methods | Password only | Password, Key, Config |
| Output formatting | Basic | With newline groups |

## 🐛 Troubleshooting

### SSH không hoạt động
```bash
# Check SSH manually
ssh user@host

# Test passwordless
ssh -o PasswordAuthentication=no user@host

# Check SSH config
cat ~/.ssh/config
```

### sshpass not found
```bash
sudo apt-get install sshpass
# Hoặc dùng SSH key thay thế
```

### Port vẫn không mở sau SSH
- Check user có quyền bind port (ports < 1024 cần root)
- Check firewall trên remote host
- Check nc có được cài đặt: `which nc`

## 💡 Tips

1. **Test SSH trước**: Đảm bảo SSH hoạt động trước khi dùng tool
2. **Dùng SSH keys**: An toàn hơn passwords
3. **Setup ~/.ssh/config**: Tiện lợi cho nhiều hosts
4. **Verbose mode**: Dùng `-v` để debug
5. **Test từng phần**: Tạo config nhỏ để test trước

## 📄 License

MIT License

## 👤 Author

Created for checking firewall connectivity in distributed network infrastructure.

## 🤝 Contributing

Contributions, issues và feature requests đều được welcome!
