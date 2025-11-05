# Release Guide

Simple guide to release FWChecker on GitHub.

## Prerequisites

1. **GitHub CLI** installed:
   ```bash
   # Debian/Ubuntu
   sudo apt install gh
   
   # CentOS/RHEL
   sudo dnf install gh
   
   # Or download from https://cli.github.com/
   ```

2. **Authenticate** (first time only):
   ```bash
   gh auth login
   ```

## Release Process

### Option 1: Automatic (Recommended)

Just push a tag, GitHub Actions will handle everything:

```bash
# Create and push tag
git tag v1.0.0
git push origin v1.0.0

# GitHub Actions will automatically:
# - Build packages
# - Create release
# - Upload .deb and .rpm files
```

### Option 2: Manual

Use the release script:

```bash
# Build and release in one command
./release.sh 1.0.0
```

That's it! The script will:
- Build both packages (.deb and .rpm)
- Create GitHub release
- Upload packages
- Generate release notes

## What Users Get

After release, users can install with:

**Debian/Ubuntu:**
```bash
wget https://github.com/anthais/fwchecker/releases/download/v1.0.0/fwchecker_1.0.0-1_amd64.deb
sudo apt install ./fwchecker_1.0.0-1_amd64.deb
```

**CentOS/RHEL:**
```bash
wget https://github.com/anthais/fwchecker/releases/download/v1.0.0/fwchecker-1.0.0-1.x86_64.rpm
sudo dnf install ./fwchecker-1.0.0-1.x86_64.rpm
```

## Version Updates

To release a new version:

1. **Update version** in `Cargo.toml`:
   ```toml
   version = "1.1.0"
   ```

2. **Commit changes**:
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 1.1.0"
   git push
   ```

3. **Create release**:
   ```bash
   git tag v1.1.0
   git push origin v1.1.0
   ```

GitHub Actions will handle the rest!

## Manual Build (Testing)

To build packages locally without releasing:

```bash
./build-packages.sh
```

Packages will be in:
- `target/debian/fwchecker_*.deb`
- `target/generate-rpm/fwchecker-*.rpm`

## Files Included in Release

Each release includes:
- `fwchecker_X.X.X-1_amd64.deb` - Debian package
- `fwchecker-X.X.X-1.x86_64.rpm` - RPM package
- `fwchecker.deb.sha256` - Checksum for .deb
- `fwchecker.rpm.sha256` - Checksum for .rpm

## GitHub Actions Workflow

The workflow (`.github/workflows/release.yml`) automatically:

1. ✅ Builds release binary
2. ✅ Creates .deb package
3. ✅ Creates .rpm package
4. ✅ Generates checksums
5. ✅ Creates GitHub release
6. ✅ Uploads all files
7. ✅ Generates release notes

## Troubleshooting

### Issue: gh command not found

Install GitHub CLI:
```bash
curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
sudo apt update
sudo apt install gh
```

### Issue: Not authenticated

Run:
```bash
gh auth login
```

### Issue: Packages not building

Make sure dependencies are installed:
```bash
sudo apt install libssh2-1-dev pkg-config
cargo install cargo-deb cargo-generate-rpm
```

## Release Checklist

Before releasing:

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md` (if exists)
- [ ] Test locally: `./build-packages.sh`
- [ ] Test installation locally
- [ ] Commit all changes
- [ ] Create and push tag: `git tag v1.0.0 && git push origin v1.0.0`
- [ ] Wait for GitHub Actions to complete
- [ ] Verify release on GitHub
- [ ] Test download and install from release

## Summary

**For quick releases:**
```bash
git tag v1.0.0 && git push origin v1.0.0
```

**For manual releases:**
```bash
./release.sh 1.0.0
```

That's all you need! 🚀

