#!/bin/bash
# Test script for fwchecker

set -e

BINARY="./target/release/fwchecker"
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                                                              ║${NC}"
echo -e "${BLUE}║          🧪 FWCHECKER TEST SUITE                            ║${NC}"
echo -e "${BLUE}║                                                              ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found. Building..."
    cargo build --release
    echo ""
fi

# Test 1: Quick test
echo -e "${GREEN}1. Quick Test${NC} (test-quick.toml)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
$BINARY --config test-quick.toml
echo ""

# Test 2: Basic test
echo -e "${GREEN}2. Basic Test${NC} (test.toml)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
$BINARY --config test.toml
echo ""

# Test 3: Extended test
echo -e "${GREEN}3. Extended Test${NC} (test-extended.toml)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
$BINARY --config test-extended.toml
echo ""

# Test 4: Full test
echo -e "${GREEN}4. Full Test${NC} (test-full.toml)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
$BINARY --config test-full.toml
echo ""

# Test output formats
echo -e "${GREEN}5. Output Format Tests${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo "  CSV Format:"
$BINARY --config test.toml --format csv | tail -5
echo ""

echo "  Table Format:"
$BINARY --config test.toml --format table | tail -7
echo ""

echo "  Verbose Mode:"
$BINARY --config test-quick.toml -v | head -10
echo ""

# Summary
echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                                                              ║${NC}"
echo -e "${BLUE}║          ✅ ALL TESTS COMPLETED SUCCESSFULLY!               ║${NC}"
echo -e "${BLUE}║                                                              ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"

