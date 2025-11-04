#!/bin/bash
# Setup script for Firewall Checker

set -e

echo "========================================="
echo "  Firewall Checker - Setup Script"
echo "========================================="
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if running as root
if [ "$EUID" -eq 0 ]; then 
    SUDO=""
    INSTALL_DIR="/usr/local/bin"
else
    SUDO="sudo"
    INSTALL_DIR="/usr/local/bin"
fi

# Check Python version
echo "→ Kiểm tra Python..."
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}✗ Python 3 không được tìm thấy. Vui lòng cài đặt Python 3.6+${NC}"
    exit 1
fi

PYTHON_VERSION=$(python3 -c 'import sys; print(".".join(map(str, sys.version_info[:2])))')
echo -e "${GREEN}✓ Python $PYTHON_VERSION${NC}"

# Check required system tools
echo ""
echo "→ Kiểm tra các công cụ hệ thống cần thiết..."

MISSING_TOOLS=()

if ! command -v ping &> /dev/null; then
    MISSING_TOOLS+=("ping")
fi

if ! command -v sshpass &> /dev/null; then
    MISSING_TOOLS+=("sshpass")
fi

if ! command -v nc &> /dev/null; then
    MISSING_TOOLS+=("netcat-openbsd")
fi

if [ ${#MISSING_TOOLS[@]} -ne 0 ]; then
    echo -e "${YELLOW}⚠ Thiếu các công cụ: ${MISSING_TOOLS[*]}${NC}"
    echo ""
    echo "Cài đặt các công cụ còn thiếu? (y/n)"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo "→ Đang cài đặt..."
        if command -v apt-get &> /dev/null; then
            $SUDO apt-get update
            $SUDO apt-get install -y sshpass netcat-openbsd iputils-ping
        elif command -v yum &> /dev/null; then
            $SUDO yum install -y sshpass nc iputils
        else
            echo -e "${RED}✗ Không thể tự động cài đặt. Vui lòng cài đặt thủ công:${NC}"
            echo "  - sshpass"
            echo "  - netcat (nc)"
            echo "  - ping"
            exit 1
        fi
        echo -e "${GREEN}✓ Đã cài đặt các công cụ${NC}"
    else
        echo -e "${YELLOW}⚠ Một số tính năng có thể không hoạt động nếu thiếu công cụ${NC}"
    fi
else
    echo -e "${GREEN}✓ Tất cả công cụ đã có sẵn${NC}"
fi

# Make script executable
echo ""
echo "→ Cấp quyền thực thi cho script..."
chmod +x fwchecker.py
echo -e "${GREEN}✓ Done${NC}"

# Install to system
echo ""
echo "Cài đặt fwchecker vào $INSTALL_DIR? (y/n)"
read -r response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    echo "→ Đang cài đặt..."
    $SUDO cp fwchecker.py $INSTALL_DIR/fwchecker
    $SUDO chmod +x $INSTALL_DIR/fwchecker
    echo -e "${GREEN}✓ Đã cài đặt fwchecker vào $INSTALL_DIR/fwchecker${NC}"
    echo ""
    echo "Bạn có thể chạy bằng lệnh:"
    echo "  fwchecker --config=bcs-firewall.conf"
else
    echo -e "${YELLOW}⚠ Bỏ qua cài đặt hệ thống${NC}"
    echo ""
    echo "Bạn có thể chạy trực tiếp bằng:"
    echo "  ./fwchecker.py --config=bcs-firewall.conf"
fi

# Create sample config if not exists
echo ""
if [ ! -f "bcs-firewall.conf" ]; then
    echo "→ File cấu hình mẫu bcs-firewall.conf đã có sẵn"
else
    echo "→ File cấu hình bcs-firewall.conf đã tồn tại"
fi

echo ""
echo "========================================="
echo -e "${GREEN}  ✓ Cài đặt hoàn tất!${NC}"
echo "========================================="
echo ""
echo "Bước tiếp theo:"
echo "  1. Chỉnh sửa file config: vim bcs-firewall.conf"
echo "  2. Chạy kiểm tra: fwchecker --config=bcs-firewall.conf"
echo ""
echo "Xem thêm thông tin: cat README.md"
echo ""

