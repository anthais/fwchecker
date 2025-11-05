#!/bin/bash
# Build script for FWChecker packages

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                                                              ║${NC}"
echo -e "${BLUE}║          🔨 BUILDING FWCHECKER PACKAGES                     ║${NC}"
echo -e "${BLUE}║                                                              ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if cargo-deb and cargo-generate-rpm are installed
if ! command -v cargo-deb &> /dev/null; then
    echo -e "${YELLOW}⚠️  cargo-deb not found. Installing...${NC}"
    cargo install cargo-deb
fi

if ! command -v cargo-generate-rpm &> /dev/null; then
    echo -e "${YELLOW}⚠️  cargo-generate-rpm not found. Installing...${NC}"
    cargo install cargo-generate-rpm
fi

# Build release binary
echo -e "${GREEN}1. Building release binary...${NC}"
cargo build --release
echo ""

# Build Debian package
echo -e "${GREEN}2. Building Debian package (.deb)...${NC}"
cargo deb
echo ""

# Build RPM package
echo -e "${GREEN}3. Building RPM package (.rpm)...${NC}"
cargo generate-rpm
echo ""

# Show results
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Build complete!${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo "📦 Packages:"
echo ""
ls -lh target/debian/*.deb 2>/dev/null | awk '{printf "  🔹 Debian:  %s (%s)\n", $9, $5}'
ls -lh target/generate-rpm/*.rpm 2>/dev/null | awk '{printf "  🔹 RPM:     %s (%s)\n", $9, $5}'
echo ""

# Create checksums
echo -e "${GREEN}4. Creating checksums...${NC}"
cd target/debian && sha256sum *.deb > fwchecker.deb.sha256 2>/dev/null || true
cd ../../target/generate-rpm && sha256sum *.rpm > fwchecker.rpm.sha256 2>/dev/null || true
cd ../..
echo ""

echo "🔐 Checksums:"
echo ""
if [ -f target/debian/fwchecker.deb.sha256 ]; then
    echo "  Debian (.deb):"
    cat target/debian/fwchecker.deb.sha256 | sed 's/^/    /'
    echo ""
fi
if [ -f target/generate-rpm/fwchecker.rpm.sha256 ]; then
    echo "  RPM (.rpm):"
    cat target/generate-rpm/fwchecker.rpm.sha256 | sed 's/^/    /'
    echo ""
fi

# Package info
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}📋 Package Information:${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Get Debian package info
DEB_FILE=$(find target/debian -name "*.deb" | head -1)
if [ -f "$DEB_FILE" ]; then
    echo "Debian Package:"
    dpkg -I "$DEB_FILE" | grep -E "Package:|Version:|Architecture:|Installed-Size:" | sed 's/^/  /'
    echo ""
fi

# Installation instructions
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}📥 Installation Instructions:${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Debian/Ubuntu:"
echo "  sudo apt install ./target/debian/fwchecker_*_amd64.deb"
echo ""
echo "CentOS/RHEL/Fedora:"
echo "  sudo dnf install ./target/generate-rpm/fwchecker-*.x86_64.rpm"
echo ""

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}🎉 Done! Packages ready for distribution.${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "See PACKAGING.md for distribution options."
echo ""

