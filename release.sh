#!/bin/bash
# Simple GitHub Release script for FWChecker

set -e

VERSION=${1:-"1.0.0"}

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                                                              ║${NC}"
echo -e "${BLUE}║          📦 GITHUB RELEASE FOR FWCHECKER                    ║${NC}"
echo -e "${BLUE}║                                                              ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo -e "${YELLOW}⚠️  GitHub CLI (gh) not found. Installing...${NC}"
    echo ""
    echo "Install with:"
    echo "  # Debian/Ubuntu"
    echo "  sudo apt install gh"
    echo ""
    echo "  # CentOS/RHEL"
    echo "  sudo dnf install gh"
    echo ""
    echo "  # Or download from https://cli.github.com/"
    exit 1
fi

# Check if logged in
if ! gh auth status &> /dev/null; then
    echo -e "${YELLOW}⚠️  Not logged in to GitHub. Please authenticate:${NC}"
    echo ""
    gh auth login
    echo ""
fi

# Build packages
echo -e "${GREEN}1. Building packages...${NC}"
./build-packages.sh
echo ""

# Get package files
DEB_FILE=$(find target/debian -name "*.deb" | head -1)
RPM_FILE=$(find target/generate-rpm -name "*.rpm" | head -1)

if [ ! -f "$DEB_FILE" ] || [ ! -f "$RPM_FILE" ]; then
    echo -e "${YELLOW}❌ Packages not found. Run ./build-packages.sh first${NC}"
    exit 1
fi

echo -e "${GREEN}2. Creating GitHub release v${VERSION}...${NC}"
echo ""

# Create release
gh release create "v${VERSION}" \
    "$DEB_FILE" \
    "$RPM_FILE" \
    --title "FWChecker v${VERSION}" \
    --notes "## FWChecker v${VERSION}

Fast, reliable network connectivity checker written in Rust.

### 📦 Installation

**Debian/Ubuntu:**
\`\`\`bash
wget https://github.com/$(gh repo view --json nameWithOwner -q .nameWithOwner)/releases/download/v${VERSION}/$(basename $DEB_FILE)
sudo apt install ./$(basename $DEB_FILE)
\`\`\`

**CentOS/RHEL/Fedora:**
\`\`\`bash
wget https://github.com/$(gh repo view --json nameWithOwner -q .nameWithOwner)/releases/download/v${VERSION}/$(basename $RPM_FILE)
sudo dnf install ./$(basename $RPM_FILE)
\`\`\`

### ✨ Features
- Multi-source checks (local or remote via SSH)
- Multiple check types (ping, telnet, curl)
- TOML configuration with type-safe parsing
- Multiple output formats (pretty, CSV, TSV, table)
- Auto port opening for testing

### 📚 Documentation
See [README.md](https://github.com/$(gh repo view --json nameWithOwner -q .nameWithOwner)#readme) for full documentation.

---

**Checksums:**
\`\`\`
$(cat target/debian/fwchecker.deb.sha256)
$(cat target/generate-rpm/fwchecker.rpm.sha256)
\`\`\`
"

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Release created successfully!${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Get release URL
RELEASE_URL=$(gh release view "v${VERSION}" --json url -q .url)
echo "🔗 Release URL: $RELEASE_URL"
echo ""

echo -e "${GREEN}📦 Packages uploaded:${NC}"
echo "  ✅ $(basename $DEB_FILE)"
echo "  ✅ $(basename $RPM_FILE)"
echo ""

echo -e "${GREEN}🎉 Done! Users can now install with:${NC}"
echo ""
echo "Debian/Ubuntu:"
echo "  wget $RELEASE_URL/$(basename $DEB_FILE)"
echo "  sudo apt install ./$(basename $DEB_FILE)"
echo ""
echo "CentOS/RHEL:"
echo "  wget $RELEASE_URL/$(basename $RPM_FILE)"
echo "  sudo dnf install ./$(basename $RPM_FILE)"
echo ""

