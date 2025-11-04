# Verbose Mode (-v)

The verbose mode provides detailed debug information about every step of the checking process.

## Usage

```bash
./fwchecker -c config.conf -v
./fwchecker --config=config.conf --verbose
```

## What Verbose Mode Shows

### 1. Config Parsing
```
[DEBUG] Reading config file: bcs-firewall.conf
[DEBUG] Added host: external 8.8.8.8:0 method=ping
[DEBUG] Added host: external google.com:443 method=telnet
[DEBUG] Added host: local 192.168.1.10:8080 method=both
```

### 2. Ping Details
```
[DEBUG] Pinging 8.8.8.8 (3 packets, 2s timeout)...
[DEBUG] Ping output:
  PING 8.8.8.8 (8.8.8.8) 56(84) bytes of data.
  64 bytes from 8.8.8.8: icmp_seq=1 ttl=115 time=20.8 ms
  64 bytes from 8.8.8.8: icmp_seq=2 ttl=115 time=19.7 ms
  64 bytes from 8.8.8.8: icmp_seq=3 ttl=115 time=21.4 ms
[DEBUG] Ping took 2.032s, exit code: 0
```

### 3. Telnet/Connection Details
```
[DEBUG] Checking telnet connection to google.com:443 (timeout: 5s)...
[DEBUG] Using netcat (nc) for connection test
[DEBUG] Netcat output:
  Connection to google.com (142.250.71.238) 443 port [tcp/https] succeeded!
[DEBUG] Connection test took 0.076s, exit code: 0
```

### 4. SSH Port Opening (for local hosts)
```
[DEBUG] Attempting to open port 8080 on 192.168.1.10 via SSH...
[DEBUG] Port 8080 is closed, need to open it via SSH
[DEBUG] sshpass command not found
OR
[DEBUG] Connecting via SSH as admin@192.168.1.10...
[DEBUG] Command: nohup nc -l -p 8080 >/dev/null 2>&1 &
[DEBUG] SSH output:
  [any SSH messages]
[DEBUG] SSH command exit code: 0
[DEBUG] Waiting 2 seconds for nc to start...
[DEBUG] nc listener started successfully
```

## Output Channels

- **Debug messages**: Go to `stderr` (can be redirected with `2>`)
- **Results**: Go to `stdout` (can be redirected with `>`)

This means you can:

### Get clean CSV output while seeing debug info
```bash
./fwchecker -c config.conf -f csv -v > report.csv
# Debug info shows on screen
# Clean CSV goes to report.csv
```

### Suppress debug info and get only results
```bash
./fwchecker -c config.conf -v 2>/dev/null
```

### Save debug info to a log file
```bash
./fwchecker -c config.conf -v 2> debug.log
```

### Save both results and debug info
```bash
./fwchecker -c config.conf -f csv -v > report.csv 2> debug.log
```

## Use Cases

### 1. Troubleshooting Connection Issues
When a connection fails, verbose mode shows:
- Exact command being executed
- Raw output from ping/nc
- Exit codes
- Timing information

```bash
./fwchecker -c problem-hosts.conf -v
```

### 2. Performance Analysis
See how long each check takes:
```
[DEBUG] Ping took 2.032s, exit code: 0
[DEBUG] Connection test took 0.076s, exit code: 0
```

### 3. SSH Debugging
When SSH port opening fails, verbose shows:
- SSH connection attempts
- Commands being executed
- SSH output/errors

### 4. Development & Testing
Perfect for:
- Understanding tool behavior
- Debugging new features
- Validating configuration changes

## Example Comparison

### Normal Mode
```bash
$ ./fwchecker -c test.conf

✓ Loaded 2 host(s) from config

============================================================
  FIREWALL CHECKER - STARTING CHECKS
============================================================

Checking google.com:443 (external)
  ✓ TELNET: OK

Checking github.com:443 (external)
  ✓ PING: OK
  ✓ TELNET: OK
```

### Verbose Mode
```bash
$ ./fwchecker -c test.conf -v

[DEBUG] Reading config file: test.conf
[DEBUG] Added host: external google.com:443 method=telnet
[DEBUG] Added host: external github.com:443 method=both
✓ Loaded 2 host(s) from config

============================================================
  FIREWALL CHECKER - STARTING CHECKS
============================================================

[DEBUG] ───────────────────────────────────────────────────────────
[DEBUG] Checking host #1: google.com:443 (external, method=telnet)
[DEBUG] ───────────────────────────────────────────────────────────
Checking google.com:443 (external)
[DEBUG] --- TELNET CHECK ---
[DEBUG] Checking telnet connection to google.com:443 (timeout: 5s)...
[DEBUG] Using netcat (nc) for connection test
[DEBUG] Netcat output:
  Connection to google.com (142.250.71.238) 443 port [tcp/https] succeeded!
[DEBUG] Connection test took 0.076s, exit code: 0
[DEBUG] Result: ✓ PASS
  ✓ TELNET: OK

[DEBUG] ───────────────────────────────────────────────────────────
[DEBUG] Checking host #2: github.com:443 (external, method=both)
[DEBUG] ───────────────────────────────────────────────────────────
Checking github.com:443 (external)
[DEBUG] --- PING CHECK ---
[DEBUG] Pinging github.com (3 packets, 2s timeout)...
[DEBUG] Ping output:
  PING github.com (20.205.243.166) 56(84) bytes of data.
  64 bytes from 20.205.243.166: icmp_seq=1 ttl=110 time=55.0 ms
  64 bytes from 20.205.243.166: icmp_seq=2 ttl=110 time=55.2 ms
  64 bytes from 20.205.243.166: icmp_seq=3 ttl=110 time=55.0 ms
[DEBUG] Ping took 2.062s, exit code: 0
[DEBUG] Result: ✓ PASS
  ✓ PING: OK
[DEBUG] --- TELNET CHECK ---
[DEBUG] Checking telnet connection to github.com:443 (timeout: 5s)...
[DEBUG] Using netcat (nc) for connection test
[DEBUG] Netcat output:
  Connection to github.com (20.205.243.166) 443 port [tcp/https] succeeded!
[DEBUG] Connection test took 0.078s, exit code: 0
[DEBUG] Result: ✓ PASS
  ✓ TELNET: OK
```

## Tips

1. **Always use verbose mode when troubleshooting**
   ```bash
   ./fwchecker -c failing-host.conf -v
   ```

2. **Combine with CSV for analysis**
   ```bash
   ./fwchecker -c all-hosts.conf -f csv -v > results.csv 2> debug.log
   ```

3. **Use grep to filter debug info**
   ```bash
   ./fwchecker -c config.conf -v 2>&1 | grep -E "\[DEBUG\]|FAIL"
   ```

4. **Check timing for slow connections**
   ```bash
   ./fwchecker -c config.conf -v 2>&1 | grep "took"
   ```

## Debug Message Types

| Pattern | What it shows |
|---------|---------------|
| `─────────────` | Separator between hosts |
| `Checking host #X` | Starting check for host number X |
| `--- PING CHECK ---` | Beginning of ping test section |
| `--- TELNET CHECK ---` | Beginning of telnet test section |
| `--- SSH PORT OPENING ---` | Beginning of SSH section |
| `Result: ✓ PASS` | Check passed successfully |
| `Result: ✗ FAIL` | Check failed |
| `Reading config file` | Config parsing started |
| `Added host` | Each host loaded from config |
| `Pinging X` | Ping check started |
| `Ping output:` | Raw ping command output |
| `Ping took Xs` | Ping duration and result |
| `Checking telnet connection` | Telnet check started |
| `Using netcat (nc)` | Tool being used for check |
| `Netcat output:` | Raw nc command output |
| `Connection test took Xs` | Telnet duration and result |
| `Attempting to open port` | SSH port opening started |
| `Port X is already open` | Port already accessible |
| `Port X is closed` | Need to SSH and open port |
| `Connecting via SSH` | SSH connection attempt |
| `Command: ...` | Exact command being executed |
| `SSH output:` | Raw SSH command output |
| `SSH command exit code` | SSH result |
| `Waiting X seconds` | Delay for service startup |

## Common Exit Codes

- `0` - Success
- `1` - General failure
- `124` - Timeout (from timeout command)
- `130` - Interrupted by user (Ctrl+C)

