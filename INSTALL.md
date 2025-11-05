# Installation Guide

## Quick Install (Recommended)

### Debian/Ubuntu

```bash
# Download latest release
wget https://github.com/anthais/fwchecker/releases/latest/download/fwchecker_1.0.0-1_amd64.deb

# Install
sudo apt install ./fwchecker_1.0.0-1_amd64.deb

# Verify
fwchecker --version
```

### CentOS/RHEL/Fedora

```bash
# Download latest release
wget https://github.com/anthais/fwchecker/releases/latest/download/fwchecker-1.0.0-1.x86_64.rpm

# Install
sudo dnf install ./fwchecker-1.0.0-1.x86_64.rpm

# Verify
fwchecker --version
```

## What Gets Installed

- **Binary**: `/usr/bin/fwchecker`
- **Documentation**: `/usr/share/doc/fwchecker/`
  - `README.md` - Full documentation
  - `LICENSE` - MIT License
  - `examples/` - Example configurations

## Dependencies

The following packages will be automatically installed:

**Debian/Ubuntu:**
- `openssh-client` - For SSH connections
- `iputils-ping` - For ICMP ping
- `telnet` - For TCP port checks
- `curl` - For HTTP/HTTPS checks

**CentOS/RHEL/Fedora:**
- `openssh-clients` - For SSH connections
- `iputils` - For ICMP ping
- `telnet` - For TCP port checks
- `curl` - For HTTP/HTTPS checks

## First Run

```bash
# Create a config file
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

## Verify Installation

```bash
# Check version
fwchecker --version

# Check binary location
which fwchecker

# View help
fwchecker --help

# List installed files
dpkg -L fwchecker          # Debian/Ubuntu
rpm -ql fwchecker          # CentOS/RHEL
```

## Update

To update to a new version:

```bash
# Download new version
wget https://github.com/anthais/fwchecker/releases/latest/download/fwchecker_1.1.0-1_amd64.deb

# Upgrade (Debian/Ubuntu)
sudo apt install ./fwchecker_1.1.0-1_amd64.deb

# Or upgrade (CentOS/RHEL)
sudo dnf upgrade ./fwchecker-1.1.0-1.x86_64.rpm
```

## Uninstall

```bash
# Debian/Ubuntu
sudo apt remove fwchecker

# CentOS/RHEL/Fedora
sudo dnf remove fwchecker
```

## Alternative: Build from Source

If you prefer to build from source:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/anthais/fwchecker.git
cd fwchecker/fwchecker-rs

# Build and install
cargo build --release
sudo cp target/release/fwchecker /usr/local/bin/
```

## Troubleshooting

### Issue: Package not found

Make sure you're downloading from the correct release page:
```
https://github.com/anthais/fwchecker/releases
```

### Issue: Dependencies not satisfied

Install dependencies manually:

**Debian/Ubuntu:**
```bash
sudo apt update
sudo apt install openssh-client iputils-ping telnet curl
```

**CentOS/RHEL:**
```bash
sudo dnf install openssh-clients iputils telnet curl
```

### Issue: Permission denied running fwchecker

Some checks require elevated permissions. Set capabilities:
```bash
sudo setcap cap_net_raw+ep /usr/bin/fwchecker
```

### Issue: Command not found after install

Reload shell or source profile:
```bash
source ~/.bashrc
# or
hash -r
```

## Verify Package Integrity

Download and verify checksums:

```bash
# Download package and checksum
wget https://github.com/anthais/fwchecker/releases/download/v1.0.0/fwchecker_1.0.0-1_amd64.deb
wget https://github.com/anthais/fwchecker/releases/download/v1.0.0/fwchecker.deb.sha256

# Verify
sha256sum -c fwchecker.deb.sha256
```

## Support

- **Documentation**: [README.md](README.md)
- **Examples**: `/usr/share/doc/fwchecker/examples/`
- **Issues**: https://github.com/anthais/fwchecker/issues

## Next Steps

After installation:

1. Read the [README.md](README.md) for full documentation
2. Check example configs in `/usr/share/doc/fwchecker/examples/`
3. Create your own config file
4. Run your first check!

```bash
fwchecker --config /usr/share/doc/fwchecker/examples/test.toml
```

