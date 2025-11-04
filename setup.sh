#!/bin/bash
# Setup script for Firewall Checker (Bash version)

set -e

echo "========================================="
echo "  Firewall Checker - Setup Script"
echo "========================================="
echo ""

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Check Bash version
echo "→ Kiểm tra Bash..."
if [[ -z "$BASH_VERSION" ]]; then
    echo -e "${RED}✗ Script này cần Bash để chạy${NC}"
    exit 1
fi

BASH_MAJOR_VERSION="${BASH_VERSION%%.*}"
if [[ "$BASH_MAJOR_VERSION" -lt 4 ]]; then
    echo -e "${YELLOW}⚠ Bash version cũ ($BASH_VERSION). Khuyến nghị Bash 4+${NC}"
else
    echo -e "${GREEN}✓ Bash $BASH_VERSION${NC}"
fi

# Check required system tools
echo ""
echo "→ Kiểm tra các công cụ hệ thống..."

MISSING_TOOLS=()

command -v ping &>/dev/null || MISSING_TOOLS+=("iputils-ping")
command -v nc &>/dev/null || MISSING_TOOLS+=("netcat-openbsd")
command -v sshpass &>/dev/null || MISSING_TOOLS+=("sshpass")
command -v timeout &>/dev/null || MISSING_TOOLS+=("coreutils")

if [[ ${#MISSING_TOOLS[@]} -ne 0 ]]; then
    echo -e "${YELLOW}⚠ Thiếu các công cụ: ${MISSING_TOOLS[*]}${NC}"
    echo ""
    echo "Cài đặt các công cụ còn thiếu? (y/n)"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo "→ Đang cài đặt..."
        if command -v apt-get &>/dev/null; then
            sudo apt-get update
            sudo apt-get install -y sshpass netcat-openbsd iputils-ping coreutils
        elif command -v yum &>/dev/null; then
            sudo yum install -y sshpass nc iputils coreutils
        elif command -v dnf &>/dev/null; then
            sudo dnf install -y sshpass nc iputils coreutils
        else
            echo -e "${RED}✗ Không thể tự động cài đặt. Vui lòng cài đặt thủ công:${NC}"
            echo "  - sshpass"
            echo "  - netcat (nc)"
            echo "  - ping (iputils-ping)"
            echo "  - timeout (coreutils)"
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
chmod +x fwchecker
echo -e "${GREEN}✓ Done${NC}"

# Test script
echo ""
echo "→ Kiểm tra script có chạy được không..."
if ./fwchecker --help &>/dev/null; then
    echo -e "${GREEN}✓ Script hoạt động tốt${NC}"
else
    echo -e "${RED}✗ Script có vấn đề${NC}"
    exit 1
fi

# Install to system
echo ""
echo "Cài đặt fwchecker vào /usr/local/bin? (y/n)"
read -r response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    echo "→ Đang cài đặt..."
    sudo cp fwchecker /usr/local/bin/
    sudo chmod +x /usr/local/bin/fwchecker
    echo -e "${GREEN}✓ Đã cài đặt fwchecker vào /usr/local/bin/fwchecker${NC}"
    echo ""
    echo "Bạn có thể chạy bằng lệnh:"
    echo "  fwchecker --config=bcs-firewall.conf"
else
    echo -e "${YELLOW}⚠ Bỏ qua cài đặt hệ thống${NC}"
    echo ""
    echo "Bạn có thể chạy trực tiếp bằng:"
    echo "  ./fwchecker --config=bcs-firewall.conf"
fi

# Create sample config if not exists
echo ""
if [ -f "bcs-firewall.conf" ]; then
    echo "→ File cấu hình mẫu bcs-firewall.conf đã tồn tại"
else
    echo "→ Tạo file cấu hình mẫu..."
    cat > bcs-firewall.conf << 'EOF'
# Firewall Checker Configuration File
# Format: type,ip,port,method,ssh_user,ssh_password
# 
# type: external | local
# ip: IP address to check
# port: Port number (optional for ping-only checks, use 0 for ping only)
# method: telnet | ping | both
# ssh_user: SSH username (required for local type)
# ssh_password: SSH password (required for local type)

# External hosts - only telnet/ping check
external,8.8.8.8,0,ping
external,1.1.1.1,0,ping
external,google.com,443,telnet
external,github.com,443,both

# Local hosts - can SSH to open ports if needed
# local,192.168.1.10,8080,telnet,admin,secretpass
# local,192.168.1.20,3306,telnet,root,mysql123
EOF
    echo -e "${GREEN}✓ Đã tạo bcs-firewall.conf${NC}"
fi

echo ""
echo "========================================="
echo -e "${GREEN}  ✓ Cài đặt hoàn tất!${NC}"
echo "========================================="
echo ""
echo "Bước tiếp theo:"
echo "  1. Chỉnh sửa file config: vim bcs-firewall.conf"
echo "  2. Chạy kiểm tra: ./fwchecker --config=bcs-firewall.conf"
echo ""
echo "Xem thêm thông tin: cat README.md"
echo ""
