# 🚀 Quick Start Guide - fwchecker v2.0

## Cài đặt nhanh (30 giây)

```bash
# 1. Clone hoặc cd vào thư mục
cd /workspace/anthais/fwchecker

# 2. Make executable
chmod +x fwchecker

# 3. Test với config mẫu
./fwchecker --config=test-v2.conf

# 4. Xem help
./fwchecker --help
```

## Format config v2 đơn giản

### Cấu trúc cơ bản

```ini
# Define HOSTs
[HOST:name]
type=local|remote|external
ip=<address>
user=<user>
password=<pass>

# Define CHECKs
[CHECK:hostname]
cmd=ping <target>
cmd=telnet <target> <port>
cmd=curl <url>
newline
```

### Example 1: Check từ local

```ini
[HOST:local]
type=local

[HOST:google]
type=external
ip=google.com

[CHECK:local]
cmd=ping google
cmd=telnet google 443
cmd=curl -s https://google.com
```

### Example 2: Check từ remote server

```ini
[HOST:local]
type=local

[HOST:app1]
type=remote
ip=192.168.1.10
user=admin
password=admin123

[HOST:google]
type=external
ip=google.com

# Check FROM local
[CHECK:local]
cmd=ping app1
cmd=telnet app1 22

# Check FROM app1 (via SSH)
[CHECK:app1]
cmd=ping google
cmd=telnet google 443
cmd=curl -s https://google.com
```

### Example 3: With auto port opening

```ini
[HOST:app1]
type=remote
ip=192.168.1.10
user=admin
password=admin123
openServicePorts=80,443    # Auto-open these ports!

[CHECK:local]
cmd=telnet app1 80    # Will auto-open port 80 first
cmd=telnet app1 443   # Will auto-open port 443 first
```

### Example 4: Passwordless SSH

```ini
[HOST:app1]
type=remote
ip=192.168.1.10
# No user/password = use SSH key or ~/.ssh/config

[CHECK:app1]
cmd=ping 8.8.8.8
```

## Chạy checks

```bash
# Pretty output (default)
./fwchecker -c example.conf

# CSV output for Excel
./fwchecker -c example.conf -f csv > report.csv

# Table output
./fwchecker -c example.conf -f table

# Verbose mode (debug)
./fwchecker -c example.conf -v
```

## Output mẫu

```
✓ Loaded 4 host(s) and 2 check section(s) from config

============================================================
  FIREWALL CHECKER - STARTING CHECKS
============================================================

Checking from: local (local)
  ✓ PING local -> google
  ✓ TELNET local -> google:443

  ✓ PING local -> app1
  ✓ TELNET local -> app1:22

Checking from: app1 (remote)
  ✓ PING app1 -> google
  ✓ CURL app1 -> https://google.com

============================================================
  SUMMARY
============================================================

✓ PASS - local -> google (ping)
✓ PASS - local -> google:443 (telnet)
✓ PASS - local -> app1 (ping)
✓ PASS - local -> app1:22 (telnet)
✓ PASS - app1 -> google (ping)
✓ PASS - app1 -> https://google.com (curl)

Total: 6 | Passed: 6 | Failed: 0
```

## Tips

1. **Test local first**: Bắt đầu với [CHECK:local] để test cơ bản
2. **Use SSH keys**: An toàn hơn passwords
3. **Group checks**: Dùng `newline` để phân tách các groups
4. **Verbose mode**: Dùng `-v` khi debug
5. **Start simple**: Tạo config nhỏ, test, rồi mở rộng

## Config Templates

### Template 1: Simple Local Checks
```ini
[HOST:local]
type=local

[HOST:google]
type=external
ip=google.com

[HOST:dns]
type=external
ip=8.8.8.8

[CHECK:local]
cmd=ping dns
cmd=ping google
cmd=telnet google 443
```

### Template 2: Multi-Host Infrastructure
```ini
# Define all hosts
[HOST:local]
type=local

[HOST:app1]
type=remote
ip=192.168.1.10
user=admin

[HOST:app2]
type=remote
ip=192.168.1.20
user=admin

[HOST:db1]
type=remote
ip=192.168.1.30
user=dbadmin

[HOST:dns]
type=external
ip=8.8.8.8

# Check FROM local
[CHECK:local]
cmd=ping app1
cmd=ping app2
cmd=ping db1
newline

# Check FROM app1
[CHECK:app1]
cmd=ping dns
cmd=ping db1
cmd=telnet db1 3306
newline

# Check FROM app2
[CHECK:app2]
cmd=ping dns
cmd=ping db1
cmd=telnet db1 3306
newline

# Check FROM db1
[CHECK:db1]
cmd=ping dns
cmd=ping app1
cmd=ping app2
```

## Migration từ v1

Nếu bạn có config v1 cũ, v1 script vẫn available:

```bash
# Use old version
./fwchecker.v1.bak --config=old-config.conf

# Or migrate to v2 format manually
```

---

Xem chi tiết: [README.md](README.md) | [CHANGELOG.md](CHANGELOG.md)
