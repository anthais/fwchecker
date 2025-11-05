# FWChecker

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Release](https://img.shields.io/github/v/release/anthais/fwchecker)](https://github.com/anthais/fwchecker/releases)

> 🚀 Fast network connectivity checker with multi-host SSH support

Check firewall rules and network connectivity from multiple sources to multiple targets using ping, telnet, and HTTP/HTTPS requests.

## ✨ Features

- 🚀 **Fast & Concurrent** - Parallel execution with async/await
- 🔒 **Type-Safe** - Strong typing with TOML config validation
- 🌐 **Multi-Source Checks** - Run checks from local or remote hosts via SSH
- 🎯 **Flexible Targets** - Check ICMP, TCP ports, and HTTP endpoints
- 📊 **Multiple Output Formats** - Pretty, CSV, TSV, or table output
- 🔐 **SSH Support** - Passwordless or password-based SSH authentication
- 🎨 **Colored Output** - Clear visual feedback with status colors
- 🛠️ **Auto Port Opening** - Automatically open ports with netcat for testing

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone <repo-url>
cd fwchecker-rs

# Build release binary
cargo build --release

# Binary is at: target/release/fwchecker
```

### Basic Usage

```bash
# Run with default pretty output
./target/release/fwchecker --config example.toml

# CSV output
./target/release/fwchecker -c example.toml --format csv

# Verbose mode
./target/release/fwchecker -c example.toml -v

# Help
./target/release/fwchecker --help
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
fwchecker-rs/
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

Run the included test configs:

```bash
# Simple test (3 checks)
cargo run -- --config test.toml

# Extended test (with curl and newline)
cargo run -- --config test-extended.toml

# Full example
cargo run -- --config example.toml
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
