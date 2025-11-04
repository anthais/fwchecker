#!/usr/bin/env python3
"""
Firewall Checker Tool
Kiểm tra kết nối firewall qua ping và telnet
Hỗ trợ cả external và local hosts với khả năng tự động mở port
"""

import argparse
import socket
import subprocess
import sys
import time
import threading
from typing import Dict, List, Tuple
from dataclasses import dataclass
from enum import Enum


class CheckType(Enum):
    PING = "ping"
    TELNET = "telnet"
    BOTH = "both"


class HostType(Enum):
    EXTERNAL = "external"
    LOCAL = "local"


@dataclass
class HostConfig:
    """Cấu hình cho một host cần kiểm tra"""
    host_type: HostType
    ip: str
    port: int
    method: CheckType
    ssh_user: str = ""
    ssh_password: str = ""


class Colors:
    """ANSI color codes cho output đẹp"""
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    BOLD = '\033[1m'
    END = '\033[0m'


class FirewallChecker:
    """Main class để thực hiện các kiểm tra firewall"""
    
    def __init__(self, config_file: str):
        self.config_file = config_file
        self.hosts: List[HostConfig] = []
        self.results: List[Dict] = []
    
    def parse_config(self) -> bool:
        """Parse file config và load danh sách hosts"""
        try:
            with open(self.config_file, 'r') as f:
                lines = f.readlines()
            
            for line_num, line in enumerate(lines, 1):
                line = line.strip()
                # Skip comments và empty lines
                if not line or line.startswith('#'):
                    continue
                
                parts = [p.strip() for p in line.split(',')]
                if len(parts) < 4:
                    print(f"{Colors.YELLOW}Warning: Bỏ qua dòng {line_num} - không đúng format{Colors.END}")
                    continue
                
                try:
                    host_type = HostType(parts[0])
                    ip = parts[1]
                    port = int(parts[2])
                    method = CheckType(parts[3])
                    
                    ssh_user = parts[4] if len(parts) > 4 else ""
                    ssh_password = parts[5] if len(parts) > 5 else ""
                    
                    # Validate local type phải có SSH credentials
                    if host_type == HostType.LOCAL and (not ssh_user or not ssh_password):
                        print(f"{Colors.YELLOW}Warning: Dòng {line_num} - local type cần ssh_user và ssh_password{Colors.END}")
                        continue
                    
                    host = HostConfig(host_type, ip, port, method, ssh_user, ssh_password)
                    self.hosts.append(host)
                    
                except ValueError as e:
                    print(f"{Colors.YELLOW}Warning: Dòng {line_num} - giá trị không hợp lệ: {e}{Colors.END}")
                    continue
            
            if not self.hosts:
                print(f"{Colors.RED}Error: Không có host nào được load từ config{Colors.END}")
                return False
            
            print(f"{Colors.GREEN}✓ Đã load {len(self.hosts)} host(s) từ config{Colors.END}\n")
            return True
            
        except FileNotFoundError:
            print(f"{Colors.RED}Error: Không tìm thấy file config: {self.config_file}{Colors.END}")
            return False
        except Exception as e:
            print(f"{Colors.RED}Error: Lỗi khi đọc config: {e}{Colors.END}")
            return False
    
    def check_ping(self, ip: str) -> Tuple[bool, str]:
        """Kiểm tra ping đến host"""
        try:
            # Sử dụng -c 3 (3 packets) và -W 2 (timeout 2s)
            result = subprocess.run(
                ['ping', '-c', '3', '-W', '2', ip],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                timeout=10
            )
            
            if result.returncode == 0:
                return True, "OK"
            else:
                return False, "Không thể ping"
                
        except subprocess.TimeoutExpired:
            return False, "Timeout"
        except Exception as e:
            return False, f"Error: {str(e)}"
    
    def check_telnet(self, ip: str, port: int, timeout: int = 5) -> Tuple[bool, str]:
        """Kiểm tra kết nối telnet (TCP) đến host:port"""
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(timeout)
            result = sock.connect_ex((ip, port))
            sock.close()
            
            if result == 0:
                return True, "OK"
            else:
                return False, f"Không thể kết nối (code: {result})"
                
        except socket.timeout:
            return False, "Timeout"
        except socket.gaierror:
            return False, "Không resolve được hostname"
        except Exception as e:
            return False, f"Error: {str(e)}"
    
    def ssh_open_port(self, host: HostConfig) -> Tuple[bool, str]:
        """SSH vào local host và mở port với nc"""
        try:
            # Kiểm tra xem port đã mở chưa
            is_open, msg = self.check_telnet(host.ip, host.port, timeout=2)
            if is_open:
                return True, "Port đã mở sẵn"
            
            print(f"  → Port {host.port} chưa mở, đang SSH vào {host.ip} để mở port...")
            
            # Sử dụng sshpass để tự động nhập password
            # Command: sshpass -p 'password' ssh user@host 'nc -l -p port'
            # Chạy trong background với nohup
            ssh_cmd = (
                f"sshpass -p '{host.ssh_password}' ssh "
                f"-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null "
                f"-o ConnectTimeout=5 "
                f"{host.ssh_user}@{host.ip} "
                f"'nohup nc -l -p {host.port} > /dev/null 2>&1 &'"
            )
            
            result = subprocess.run(
                ssh_cmd,
                shell=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                timeout=10
            )
            
            if result.returncode != 0:
                stderr = result.stderr.decode('utf-8', errors='ignore')
                return False, f"SSH failed: {stderr[:100]}"
            
            # Đợi một chút để nc khởi động
            time.sleep(2)
            
            # Kiểm tra lại xem port đã mở chưa
            is_open, msg = self.check_telnet(host.ip, host.port, timeout=2)
            if is_open:
                return True, "Đã mở port thành công"
            else:
                return False, f"Không thể mở port: {msg}"
                
        except subprocess.TimeoutExpired:
            return False, "SSH timeout"
        except Exception as e:
            return False, f"Error: {str(e)}"
    
    def check_host(self, host: HostConfig) -> Dict:
        """Thực hiện kiểm tra cho một host"""
        result = {
            'host_type': host.host_type.value,
            'ip': host.ip,
            'port': host.port,
            'method': host.method.value,
            'ping': None,
            'telnet': None,
            'ssh_open': None
        }
        
        print(f"{Colors.BOLD}Checking {host.ip}:{host.port} ({host.host_type.value}){Colors.END}")
        
        # Nếu là local type và method là telnet, thử mở port trước
        if host.host_type == HostType.LOCAL and host.method in [CheckType.TELNET, CheckType.BOTH]:
            if host.port > 0:  # Port 0 means ping only
                success, msg = self.ssh_open_port(host)
                result['ssh_open'] = {'success': success, 'message': msg}
                if success:
                    print(f"  {Colors.GREEN}✓ SSH/NC: {msg}{Colors.END}")
                else:
                    print(f"  {Colors.YELLOW}⚠ SSH/NC: {msg}{Colors.END}")
        
        # Thực hiện ping check
        if host.method in [CheckType.PING, CheckType.BOTH]:
            success, msg = self.check_ping(host.ip)
            result['ping'] = {'success': success, 'message': msg}
            
            if success:
                print(f"  {Colors.GREEN}✓ PING: {msg}{Colors.END}")
            else:
                print(f"  {Colors.RED}✗ PING: {msg}{Colors.END}")
        
        # Thực hiện telnet check
        if host.method in [CheckType.TELNET, CheckType.BOTH] and host.port > 0:
            success, msg = self.check_telnet(host.ip, host.port)
            result['telnet'] = {'success': success, 'message': msg}
            
            if success:
                print(f"  {Colors.GREEN}✓ TELNET: {msg}{Colors.END}")
            else:
                print(f"  {Colors.RED}✗ TELNET: {msg}{Colors.END}")
        
        print()
        return result
    
    def run_checks(self):
        """Chạy tất cả các checks"""
        print(f"\n{Colors.BOLD}{Colors.BLUE}{'='*60}{Colors.END}")
        print(f"{Colors.BOLD}{Colors.BLUE}  FIREWALL CHECKER - BẮT ĐẦU KIỂM TRA{Colors.END}")
        print(f"{Colors.BOLD}{Colors.BLUE}{'='*60}{Colors.END}\n")
        
        for host in self.hosts:
            result = self.check_host(host)
            self.results.append(result)
        
        self.print_summary()
    
    def print_summary(self):
        """In tổng kết kết quả"""
        print(f"{Colors.BOLD}{Colors.BLUE}{'='*60}{Colors.END}")
        print(f"{Colors.BOLD}{Colors.BLUE}  TỔNG KẾT KẾT QUẢ{Colors.END}")
        print(f"{Colors.BOLD}{Colors.BLUE}{'='*60}{Colors.END}\n")
        
        total = len(self.results)
        passed = 0
        failed = 0
        
        for result in self.results:
            all_success = True
            
            if result['ping'] is not None:
                all_success = all_success and result['ping']['success']
            
            if result['telnet'] is not None:
                all_success = all_success and result['telnet']['success']
            
            if all_success:
                passed += 1
                status = f"{Colors.GREEN}✓ PASS{Colors.END}"
            else:
                failed += 1
                status = f"{Colors.RED}✗ FAIL{Colors.END}"
            
            print(f"{status} - {result['ip']}:{result['port']}")
        
        print(f"\n{Colors.BOLD}Tổng số: {total} | Passed: {Colors.GREEN}{passed}{Colors.END}{Colors.BOLD} | Failed: {Colors.RED}{failed}{Colors.END}")
        print(f"{Colors.BOLD}{Colors.BLUE}{'='*60}{Colors.END}\n")
        
        # Return exit code
        return 0 if failed == 0 else 1


def main():
    """Main function"""
    parser = argparse.ArgumentParser(
        description='Firewall Checker Tool - Kiểm tra kết nối firewall',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Ví dụ:
  %(prog)s --config=bcs-firewall.conf
  %(prog)s -c my-config.conf

File config format:
  type,ip,port,method,ssh_user,ssh_password
  
  type: external | local
  method: ping | telnet | both
  
  Ví dụ:
    external,8.8.8.8,0,ping
    external,google.com,443,telnet
    local,192.168.1.10,8080,telnet,root,password123
        """
    )
    
    parser.add_argument(
        '--config', '-c',
        required=True,
        help='Đường dẫn đến file config'
    )
    
    args = parser.parse_args()
    
    # Tạo checker instance
    checker = FirewallChecker(args.config)
    
    # Parse config
    if not checker.parse_config():
        sys.exit(1)
    
    # Run checks
    try:
        exit_code = checker.run_checks()
        sys.exit(exit_code)
    except KeyboardInterrupt:
        print(f"\n{Colors.YELLOW}Đã hủy bỏ bởi người dùng{Colors.END}")
        sys.exit(130)
    except Exception as e:
        print(f"{Colors.RED}Error: {e}{Colors.END}")
        sys.exit(1)


if __name__ == '__main__':
    main()

