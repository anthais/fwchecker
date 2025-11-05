# Testing Guide

## Prerequisites

### 1. Install Rust

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2. Install System Dependencies

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y libssh2-1-dev pkg-config build-essential

# RHEL/CentOS/Fedora
sudo yum install -y libssh2-devel gcc

# macOS
brew install libssh2
```

## Build the Project

```bash
# Debug build (faster compilation, slower execution)
cargo build

# Release build (slower compilation, faster execution)
cargo build --release
```

## Quick Tests

### Test 1: Minimal Test (Fastest)

Just ping Google DNS - verifies basic functionality:

```bash
# Using debug build
cargo run -- --config test-quick.toml

# Using release build
./target/release/fwchecker --config test-quick.toml
```

**Expected output:**
```
✅ local -> google-dns (8.8.8.8) | Ping: Success
```

### Test 2: Basic Test

Ping multiple targets:

```bash
cargo run -- --config test.toml
```

**Expected output:**
```
✅ local -> google-dns | Ping: Success
✅ local -> google | Ping: Success
✅ local -> google | Telnet 443: Success
```

### Test 3: Extended Test

With curl and visual separators:

```bash
cargo run -- --config test-extended.toml
```

**Expected output:**
```
✅ local -> dns | Ping: Success
✅ local -> dns | Telnet 53: Success
────────────────────────────────
✅ local -> google | Ping: Success
✅ local -> google | Telnet 443: Success
✅ local -> google | Curl: HTTP 200
...
```

### Test 4: Full Test

All check types (ping, telnet, curl):

```bash
cargo run -- --config test-full.toml
```

## Test Different Output Formats

### Pretty Output (Default - Colored)

```bash
./target/release/fwchecker -c test.toml
```

Output:
```
✅ Success checks in green
❌ Failed checks in red
📊 Statistics at the end
```

### CSV Output

```bash
./target/release/fwchecker -c test.toml --format csv
```

Output:
```csv
Source,Target,Method,Detail,Status
local,8.8.8.8,Ping,,Success
local,google.com,Ping,,Success
local,google.com,Telnet,443,Success
```

### TSV Output

```bash
./target/release/fwchecker -c test.toml --format tsv
```

Output (tab-separated):
```
Source	Target	Method	Detail	Status
local	8.8.8.8	Ping		Success
```

### Table Output

```bash
./target/release/fwchecker -c test.toml --format table
```

Output:
```
+--------+-------------+--------+--------+---------+
| Source | Target      | Method | Detail | Status  |
+--------+-------------+--------+--------+---------+
| local  | 8.8.8.8     | Ping   |        | Success |
+--------+-------------+--------+--------+---------+
```

### Verbose Mode

```bash
./target/release/fwchecker -c test.toml -v
```

Shows detailed command execution information.

## Test Configs Overview

| Config File | Description | Time | Requires |
|-------------|-------------|------|----------|
| `test-quick.toml` | Single ping check | ~1s | Internet |
| `test.toml` | Basic ping & telnet | ~3s | Internet |
| `test-extended.toml` | With curl & newlines | ~5s | Internet |
| `test-full.toml` | All check types | ~10s | Internet |
| `example.toml` | Full featured with SSH | ~30s | SSH access |

## Testing SSH Features

### Local to Remote SSH Test

Create a test config:

```toml
[[host]]
name = "local"
type = "local"

[[host]]
name = "myserver"
type = "remote"
ip = "192.168.1.100"
port = 22
user = "username"
password = "password"

[[host]]
name = "google"
type = "external"
ip = "google.com"

# Check from local to server
[[check]]
source = "local"
commands = [
    "ping myserver",
    "telnet myserver 22",
]

# Check from server to internet
[[check]]
source = "myserver"
commands = [
    "ping google",
    "curl https://google.com",
]
```

Run:
```bash
./target/release/fwchecker -c my-ssh-test.toml
```

### SSH Key Authentication Test

```toml
[[host]]
name = "myserver"
type = "remote"
ip = "192.168.1.100"
user = "username"
# No password = uses SSH key from ~/.ssh/
```

### Using ~/.ssh/config

```toml
[[host]]
name = "myserver"
type = "remote"
ip = "myserver.example.com"
# No user/password = uses ~/.ssh/config
```

## Common Issues & Solutions

### Issue 1: Compilation Error - libssh2 not found

```
error: failed to run custom build command for `ssh2-sys`
```

**Solution:**
```bash
# Ubuntu/Debian
sudo apt-get install libssh2-1-dev

# RHEL/CentOS
sudo yum install libssh2-devel

# macOS
brew install libssh2
```

### Issue 2: Permission Denied (ping)

```
Error: Permission denied (ping requires root)
```

**Solution:**
```bash
# Option 1: Run with sudo (not recommended)
sudo ./target/release/fwchecker -c test.toml

# Option 2: Set capability (recommended)
sudo setcap cap_net_raw+ep ./target/release/fwchecker

# Option 3: Use system ping (automatic fallback)
# The tool should automatically use /bin/ping if raw sockets fail
```

### Issue 3: SSH Connection Failed

```
Error: Failed to connect via SSH
```

**Solution:**
- Check IP address is correct
- Verify SSH port (default 22)
- Test SSH manually: `ssh user@host`
- Check firewall rules
- Verify credentials

### Issue 4: Telnet Check Failed

```
❌ local -> google | Telnet 443: Failed
```

**Possible causes:**
- Port is closed/filtered
- Firewall blocking
- Target host down
- Network connectivity issue

**Debug:**
```bash
# Test manually
telnet google.com 443

# Or using nc
nc -zv google.com 443
```

### Issue 5: Curl Check Failed

```
❌ local -> api | Curl: Failed
```

**Debug:**
```bash
# Test manually
curl -v https://api.example.com

# Check DNS
dig api.example.com

# Check connectivity
ping api.example.com
```

## Dry Run (Test Config Only)

To validate config without running checks:

```bash
# Coming soon - config validation
cargo run -- --config test.toml --dry-run
```

## Performance Testing

### Sequential vs Concurrent

```bash
# Time the execution
time ./target/release/fwchecker -c test-full.toml

# With verbose output
time ./target/release/fwchecker -c test-full.toml -v
```

### Memory Usage

```bash
# Monitor memory
/usr/bin/time -v ./target/release/fwchecker -c test-full.toml
```

## Automated Testing

### Shell Script

```bash
#!/bin/bash
# test-all.sh

echo "🧪 Testing FWChecker..."

echo "1. Quick test..."
cargo run -- --config test-quick.toml || exit 1

echo "2. Basic test..."
cargo run -- --config test.toml || exit 1

echo "3. Extended test..."
cargo run -- --config test-extended.toml || exit 1

echo "4. Full test..."
cargo run -- --config test-full.toml || exit 1

echo "✅ All tests passed!"
```

Run:
```bash
chmod +x test-all.sh
./test-all.sh
```

### CI/CD Integration

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install dependencies
        run: sudo apt-get install libssh2-1-dev
      
      - name: Build
        run: cargo build --release
      
      - name: Test
        run: |
          cargo run -- --config test-quick.toml
          cargo run -- --config test.toml
```

## Benchmarking

```bash
# Build with optimizations
cargo build --release

# Run multiple times and average
for i in {1..10}; do
    time ./target/release/fwchecker -c test-full.toml
done
```

## Summary

**Recommended Test Sequence:**

1. ✅ `test-quick.toml` - Verify it compiles and runs (1 second)
2. ✅ `test.toml` - Basic functionality (3 seconds)
3. ✅ `test-extended.toml` - Full features without SSH (5 seconds)
4. ✅ `test-full.toml` - All check types (10 seconds)
5. ✅ `example.toml` - With SSH (requires setup)

**Quick Start:**
```bash
# One-liner test
cargo build --release && ./target/release/fwchecker -c test-quick.toml
```

Happy testing! 🚀

