# Changelog

All notable changes to fwchecker will be documented in this file.

## [2.0.0] - 2025-11-05

### 🎉 Major Rewrite - Complete Redesign

#### Added
- **Host-based configuration**: Define multiple hosts and run checks from each host
- **Remote execution**: SSH into hosts and run checks from there
- **Passwordless SSH support**: Use SSH keys or ~/.ssh/config
- **openServicePorts**: Auto-open ports with netcat before checking
- **newline directive**: Better output formatting with groups
- **Full curl support**: Pass complete curl commands
- **Hostname resolution**: Use hostname aliases from config
- **Source host column**: Track which host performed each check
- **Verbose mode improvements**: Better debug information

#### Changed
- **Config format**: From CSV to structured INI-style format
  - Old: `type,ip,port,method,user,pass`
  - New: `[HOST:name]` and `[CHECK:name]` sections
- **Check execution**: Group by source host instead of flat list
- **SSH handling**: Support multiple authentication methods
- **Output format**: Include source host in results

#### Breaking Changes
- Config file format completely changed (v1 configs not compatible)
- Command-line options remain compatible
- Output format structure changed (added Source column)

#### Migration Guide

**Old v1 config:**
```
external,8.8.8.8,0,ping
external,google.com,443,telnet
local,192.168.1.10,8080,telnet,admin,pass123
```

**New v2 config:**
```ini
[HOST:local]
type=local

[HOST:google]
type=external
ip=google.com

[HOST:app1]
type=remote
ip=192.168.1.10
user=admin
password=pass123

[CHECK:local]
cmd=ping google
cmd=telnet google 443
cmd=telnet app1 8080
```

#### Backup
- Old version backed up as `fwchecker.v1.bak`
- Old configs still work with `fwchecker.v1.bak --config=old.conf`

---

## [1.0.0] - 2024-XX-XX

### Initial Release

#### Features
- Check firewall connectivity via ping and telnet
- Check HTTP/HTTPS via curl
- Support external and local hosts
- Auto-open ports on local hosts via SSH
- Multiple output formats (pretty, csv, tsv, table)
- Verbose mode for debugging
- Multiple check methods can be combined (ping+telnet+curl)

#### Commands
- `ping`: ICMP echo test
- `telnet`: TCP connection test  
- `curl`: HTTP/HTTPS request test

#### Configuration
- Simple CSV format
- Support for external and local types
- SSH authentication with password
- Method combinations with + separator
