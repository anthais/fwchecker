# FWChecker

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Release](https://img.shields.io/github/v/release/anthais/fwchecker)](https://github.com/anthais/fwchecker/releases)

> 🚀 Fast network connectivity checker with multi-host SSH support

---

## Tại sao dự án này ra đời?

Khi làm việc trong hệ thống nội bộ của một công ty, việc phát triển một dịch vụ mới cần kết nối tới các dịch vụ khác sẽ đòi hỏi bạn phải khai báo những rule firewall cần mở. Sau khi báo cho admin để mở firewall, bạn lại phải kiểm tra xem đã mở thành công chưa. Nếu dịch vụ cần kết nối đến hàng chục, hàng trăm server, việc SSH vào từng máy rồi gõ cả trăm lệnh thực sự là cơn ác mộng—có thể mất hàng giờ và dễ bỏ sót.

Ở giai đoạn đầu của dự án, có thể chưa có ứng dụng nào chạy nên không thể kiểm tra firewall qua telnet được. Bạn thường phải SSH vào máy đích tạo dịch vụ "ảo" bằng `nc` (netcat). Sự lặp đi lặp lại tẻ nhạt, dễ sai sót này khiến tôi mệt mỏi, và đó là lý do **fwchecker** ra đời.

Với **fwchecker**, bạn chỉ cần khai báo toàn bộ rule firewall trong một file cấu hình. Chỉ với một lệnh, mọi thứ tự động thực hiện song song. Cuộc sống tươi đẹp hơn hẳn! ✨

---

## Why did this project come to life?

When I work in a company's internal system, developing a new service that needs to connect to other services requires declaring which firewall rules need to be opened. After notifying the system administrator to open the firewall rule, I need to verify whether it was successful. When a service needs to connect to tens or hundreds of other services, having to SSH into each server and run hundreds of commands becomes a nightmare—it can take hours, and I might miss checking some rules.

In the early stages of creating a new service, there might not be any application running yet, making firewall checks via telnet unfeasible. This often means I have to SSH into the target machine to create a dummy service using `nc` (netcat). I got tired of this repetitive, error-prone process, and that's how **fwchecker** was born.

With **fwchecker**, you can declare all firewall rules in a single configuration file and, with one command, all the jobs are done, in parallel. Life is so beautiful! ✨

---

## ✨ Key Functionality

- ✅ **Network Connectivity Checks**: Test connectivity using ICMP ping, TCP port checks (Telnet), and HTTP/HTTPS requests.
- 🌐 **Source Flexibility**: Run tests from both local and remote (SSH-accessible) hosts.
- 🎯 **Multi-Target Support**: Check multiple destinations in a single run, including servers, network devices, or services.
- 🔄 **Parallel Execution**: Perform checks concurrently for fast and efficient results.
- 📝 **Configurable via TOML**: Define hosts, checks, and targets in a structured, type-safe TOML config file.
- 📋 **Detailed Output Formats**: Export results as pretty text, CSV, TSV, or ASCII tables for easy reporting and automation.
- 🔐 **SSH Integration**: Support passwordless (key-based) or password authentication for remote checks.
- 🛠️ **Test TCP Ports Easily**: Automatically open test ports with netcat on targets if needed.

## 📦 Installation

### Quick Install (Recommended)

Download pre-built packages from [GitHub Releases](https://github.com/anthais/fwchecker/releases). Replace `${VERSION}` with the release number you want to install (for example, `1.2.0` for tag `v1.2.0`).

**Debian/Ubuntu:**
```bash
VERSION=1.2.0
wget https://github.com/anthais/fwchecker/releases/download/v${VERSION}/fwchecker_${VERSION}-1_amd64.deb
sudo apt install ./fwchecker_${VERSION}-1_amd64.deb
```

**CentOS/RHEL/Fedora:**
```bash
VERSION=1.2.0
wget https://github.com/anthais/fwchecker/releases/download/v${VERSION}/fwchecker-${VERSION}-1.x86_64.rpm
sudo dnf install ./fwchecker-${VERSION}-1.x86_64.rpm
```

**Windows (PowerShell):**
```powershell
$Version = "1.2.0"
$Tag = "v$Version"
Invoke-WebRequest -Uri "https://github.com/anthais/fwchecker/releases/download/$Tag/fwchecker-$Version-windows.zip" -OutFile "fwchecker-$Version-windows.zip"
Expand-Archive -Path "fwchecker-$Version-windows.zip" -DestinationPath "."
.\fwchecker.exe --help
```

**Verify Installation (Linux/macOS):**
```bash
fwchecker --help
which fwchecker
```

### Build from Source

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone repository
git clone https://github.com/anthais/fwchecker.git
cd fwchecker

# Install system dependencies
# Ubuntu/Debian
sudo apt install libssh2-1-dev pkg-config

# Build release binary
cargo build --release

# Binary at: target/release/fwchecker
# Optional: Install to system
sudo cp target/release/fwchecker /usr/local/bin/
```

### Package Contents

When installed via package manager:
- **Binary**: `/usr/bin/fwchecker`
- **Documentation**: `/usr/share/doc/fwchecker/`
- **Examples**: `/usr/share/doc/fwchecker/examples/`

**Dependencies** (auto-installed):
- `openssh-client/clients` - SSH support
- `iputils-ping/iputils` - ICMP ping
- `telnet` - TCP port checks
- `curl` - HTTP/HTTPS checks

## 🚀 Quick Start

### Basic Usage

```bash
# Run with default pretty output
fwchecker --config example.toml

# CSV output
fwchecker -c example.toml --format csv

# Verbose mode
fwchecker -c example.toml -v

# Help
fwchecker --help
```

### First Check

```bash
# Create a simple config
cat > test.toml << 'EOF'
[[host]]
name = "local"
type = "local"

[[host]]
name = "google"
type = "external"
ip = "google.com"

[[check]]
source = "local"
commands = [
    "ping google",
    "telnet google 443",
]
EOF

# Run check
fwchecker --config test.toml
```

## 📝 Configuration

FWChecker uses TOML configuration files with a simple, type-safe structure:

### Basic Example

```toml
# Define hosts
[[host]]
name = "local"
type = "local"

[[host]]
name = "webserver"
type = "remote"
ip = "192.168.1.10"
port = 22
user = "admin"
password = "admin123"
open_service_ports = [80, 443]  # Auto-open these ports

[[host]]
name = "google"
type = "external"
ip = "google.com"

# Define checks
[[check]]
source = "local"
commands = [
    "ping webserver",
    "telnet webserver 22",
    "telnet webserver 80",
    "curl https://google.com",
]

[[check]]
source = "webserver"
commands = [
    "ping google",
    "telnet google 443",
]
```

### Host Configuration

Each host is defined with:

- **`name`** (required) - Unique identifier for the host
- **`type`** (required) - One of: `local`, `remote`, `external`
  - `local` - Run checks on the local machine
  - `remote` - SSH into this host to run checks
  - `external` - A target to check (cannot SSH into)
- **`ip`** (optional) - IP address or hostname
- **`port`** (optional) - SSH port (default: 22)
- **`user`** (optional) - SSH username (uses key auth if not provided)
- **`password`** (optional) - SSH password (uses key auth if not provided)
- **`open_service_ports`** (optional) - Ports to auto-open with netcat

### Check Configuration

Each check is defined with:

- **`source`** (required) - Name of the host to run checks from
- **`commands`** (required) - Array of commands to execute:
  - `ping <target>` - ICMP ping check
  - `telnet <target> <port>` - TCP port connectivity check
  - `curl <url> [options]` - HTTP/HTTPS request
  - `newline` - Visual separator in output

### Complete Example

```toml
# Local machine
[[host]]
name = "local"
type = "local"

# Application servers
[[host]]
name = "app1"
type = "remote"
ip = "192.168.1.10"
port = 22
user = "admin"
password = "secret123"
open_service_ports = [80, 443]

[[host]]
name = "app2"
type = "remote"
ip = "192.168.1.20"
# No user/password = uses SSH key or ~/.ssh/config

# Database server
[[host]]
name = "db1"
type = "remote"
ip = "192.168.1.30"
user = "dbadmin"

# External targets
[[host]]
name = "dns"
type = "external"
ip = "8.8.8.8"

[[host]]
name = "api"
type = "external"
ip = "api.example.com"

# Checks from local machine
[[check]]
source = "local"
commands = [
    "ping app1",
    "telnet app1 22",
    "telnet app1 80",
    "telnet app1 443",
    "newline",
    
    "ping app2",
    "telnet app2 22",
    "newline",
]

# Checks from app1 server
[[check]]
source = "app1"
commands = [
    "ping dns",
    "telnet dns 53",
    "curl -s https://google.com",
    "newline",
    
    "ping db1",
    "telnet db1 3306",
]

# Checks from app2 server
[[check]]
source = "app2"
commands = [
    "ping api",
    "curl -X POST https://api.example.com/health",
]
```

## 🎯 Usage Examples

### Check Local Connectivity

```toml
[[host]]
name = "local"
type = "local"

[[host]]
name = "google"
type = "external"
ip = "google.com"

[[check]]
source = "local"
commands = [
    "ping google",
    "telnet google 443",
    "curl https://google.com",
]
```

### Check Remote Server Connectivity

```toml
[[host]]
name = "local"
type = "local"

[[host]]
name = "server"
type = "remote"
ip = "192.168.1.100"
user = "admin"
password = "pass123"

[[check]]
source = "local"
commands = [
    "ping server",
    "telnet server 22",
]

[[check]]
source = "server"
commands = [
    "ping 8.8.8.8",
    "curl https://google.com",
]
```

### Auto-Open Ports for Testing

```toml
[[host]]
name = "webserver"
type = "remote"
ip = "192.168.1.10"
user = "admin"
password = "pass123"
open_service_ports = [80, 443]  # Opens these ports automatically

[[check]]
source = "local"
commands = [
    "telnet webserver 80",   # Will auto-open port 80 if not listening
    "telnet webserver 443",  # Will auto-open port 443 if not listening
]
```

## 📊 Output Formats

### Pretty (Default)

```bash
./target/release/fwchecker -c config.toml

# Colored output with visual separators:
# ✅ Success in green
# ❌ Failure in red
# 📊 Statistics at the end
```

### CSV

```bash
./target/release/fwchecker -c config.toml --format csv

# Output:
# Source,Target,Method,Detail,Status
# local,google.com,Ping,,Success
# local,google.com,Telnet,443,Success
```

### TSV

```bash
./target/release/fwchecker -c config.toml --format tsv

# Tab-separated values for easy parsing
```

### Table

```bash
./target/release/fwchecker -c config.toml --format table

# ASCII table format:
# +--------+-------------+--------+--------+---------+
# | Source | Target      | Method | Detail | Status  |
# +--------+-------------+--------+--------+---------+
```

## 🏗️ Project Structure

```
fwchecker/
├── Cargo.toml          # Dependencies and project metadata
├── README.md           # This file
├── .gitignore          # Git ignore rules
│
├── Config Files:
│   ├── example.toml    # Full featured example
│   ├── test.toml       # Simple test config
│   └── test-extended.toml  # Extended test config
│
└── src/
    ├── main.rs         # CLI entry point and orchestration
    ├── config.rs       # TOML config parser with serde
    ├── checker.rs      # Check execution (ping/telnet/curl)
    ├── executor.rs     # Command execution (local/SSH)
    └── output.rs       # Output formatters
```

## 📚 Dependencies

- **clap** 4.4 - CLI argument parsing with derive macros
- **toml** 0.8 - TOML configuration parsing
- **serde** 1.0 - Serialization/deserialization framework
- **tokio** 1.35 - Async runtime for concurrency
- **ssh2** 0.9 - SSH client library (libssh2 bindings)
- **colored** 2.1 - Terminal color output
- **anyhow** 1.0 - Flexible error handling
- **reqwest** 0.11 - HTTP client for curl checks

## 🔨 Development

```bash
# Check code without building
cargo check

# Run with arguments
cargo run -- --config example.toml

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Build optimized release
cargo build --release

# Install to ~/.cargo/bin
cargo install --path .
```

## 🧪 Testing

### Quick Tests

**Test 1: Minimal (Fastest ~1s)**
```bash
fwchecker --config test-quick.toml
# Expected: ✅ local -> google-dns | Ping: Success
```

**Test 2: Basic (~3s)**
```bash
fwchecker --config test.toml
# Expected: 3 successful checks (ping + telnet)
```

**Test 3: Extended (~5s)**
```bash
fwchecker --config test-extended.toml
# Expected: 8 checks with curl and newlines
```

**Test 4: Full (~10s)**
```bash
fwchecker --config test-full.toml
# Expected: All check types
```

### Test Output Formats

```bash
# Pretty (colored, default)
fwchecker -c test.toml

# CSV (for Excel/reporting)
fwchecker -c test.toml --format csv

# TSV (tab-separated)
fwchecker -c test.toml --format tsv

# ASCII Table
fwchecker -c test.toml --format table

# Verbose mode (show commands)
fwchecker -c test.toml -v
```

### Test SSH Features

Create SSH test config:
```toml
[[host]]
name = "local"
type = "local"

[[host]]
name = "myserver"
type = "remote"
ip = "192.168.1.100"
user = "admin"
password = "pass123"

[[host]]
name = "google"
type = "external"
ip = "google.com"

[[check]]
source = "local"
commands = ["ping myserver", "telnet myserver 22"]

[[check]]
source = "myserver"
commands = ["ping google", "curl https://google.com"]
```

Run: `fwchecker -c my-ssh-test.toml`

### Automated Testing

Use the included test script:
```bash
./test-all.sh
```

Or run tests individually:
```bash
# Using debug build
cargo run -- --config test-quick.toml

# Using release build  
./target/release/fwchecker --config test.toml
```

## 🚀 Performance

- **Concurrent Execution**: Multiple checks run in parallel using Tokio
- **Fast Compilation**: ~1-2 minutes clean build
- **Small Binary**: ~5MB release binary (can be optimized further)
- **Low Memory**: Minimal memory footprint
- **Single Binary**: No runtime dependencies (statically linked)

## 🔧 System Requirements

### Build Requirements
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- libssh2 development files (for SSH support)
  ```bash
  # Debian/Ubuntu
  sudo apt install libssh2-1-dev
  
  # RHEL/CentOS
  sudo yum install libssh2-devel
  
  # macOS
  brew install libssh2
  ```

### Runtime Requirements
- Linux, macOS, or Windows
- Network access to target hosts
- SSH access to remote hosts (if using remote checks)
- Standard network tools (ping, telnet, curl) on remote hosts

## 📖 CLI Options

```
Usage: fwchecker [OPTIONS]

Options:
  -c, --config <FILE>     Configuration file path (required)
  -f, --format <FORMAT>   Output format: pretty, csv, tsv, table [default: pretty]
  -v, --verbose           Verbose output (show command details)
  -h, --help              Print help
  -V, --version           Print version
```

## 🎨 Features Deep Dive

### SSH Authentication

FWChecker supports multiple SSH authentication methods:

1. **Password Authentication**
   ```toml
   [[host]]
   name = "server"
   type = "remote"
   ip = "192.168.1.10"
   user = "admin"
   password = "secret123"
   ```

2. **SSH Key Authentication**
   ```toml
   [[host]]
   name = "server"
   type = "remote"
   ip = "192.168.1.10"
   user = "admin"
   # No password = uses SSH key from ~/.ssh/
   ```

3. **SSH Config**
   ```toml
   [[host]]
   name = "server"
   type = "remote"
   ip = "server.example.com"
   # No user/password = uses ~/.ssh/config
   ```

### Auto Port Opening

For testing firewall rules without running services:

```toml
[[host]]
name = "server"
type = "remote"
ip = "192.168.1.10"
user = "admin"
password = "pass123"
open_service_ports = [8080, 8443]  # Auto-open with netcat
```

When checking these ports, FWChecker will:
1. SSH into the server
2. Run `nc -l -p <port>` in the background
3. Perform the connectivity check
4. Clean up the netcat process

### Command Types

**Ping**: ICMP connectivity check
```toml
commands = ["ping google.com"]
```

**Telnet**: TCP port connectivity check
```toml
commands = [
    "telnet google.com 443",
    "telnet 192.168.1.10 22",
]
```

**Curl**: HTTP/HTTPS request
```toml
commands = [
    "curl https://google.com",
    "curl -X POST https://api.example.com/health",
    "curl -s -o /dev/null -w '%{http_code}' https://example.com",
]
```

**Newline**: Visual separator in output
```toml
commands = [
    "ping host1",
    "telnet host1 80",
    "newline",
    "ping host2",
]
```

## 🔧 Troubleshooting

### Build Issues

**Error: libssh2 not found**
```bash
# Ubuntu/Debian
sudo apt install libssh2-1-dev pkg-config

# RHEL/CentOS
sudo yum install libssh2-devel

# macOS
brew install libssh2
```

### Runtime Issues

**Permission denied (ping)**
```bash
# Set capabilities (recommended)
sudo setcap cap_net_raw+ep /usr/bin/fwchecker

# Or use system ping (automatic fallback)
```

**SSH connection failed**
- Verify IP address and port
- Test manually: `ssh user@host`
- Check firewall rules
- Verify credentials

**Telnet check failed**
- Check if port is open: `nc -zv host port`
- Verify firewall rules
- Check target host status

**Command not found after install**
```bash
source ~/.bashrc
# or
hash -r
```

### Verify Package Integrity

```bash
# Download package and checksum
wget https://github.com/anthais/fwchecker/releases/download/v1.0.0/fwchecker_1.0.0-1_amd64.deb
wget https://github.com/anthais/fwchecker/releases/download/v1.0.0/fwchecker.deb.sha256

# Verify
sha256sum -c fwchecker.deb.sha256
```

## 🔄 Update & Uninstall

### Update to New Version

```bash
# Download new version
wget https://github.com/anthais/fwchecker/releases/latest/download/fwchecker_1.1.0-1_amd64.deb

# Upgrade
sudo apt install ./fwchecker_1.1.0-1_amd64.deb  # Debian/Ubuntu
sudo dnf upgrade ./fwchecker-1.1.0-1.x86_64.rpm  # CentOS/RHEL
```

### Uninstall

```bash
sudo apt remove fwchecker          # Debian/Ubuntu
sudo dnf remove fwchecker          # CentOS/RHEL
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## 📄 License

MIT License - see LICENSE file for details

## 🔗 Links

- Documentation: See this README
- Issues: GitHub Issues
- Examples: See `example.toml`, `test.toml`

---

**Built with ❤️ in Rust**
