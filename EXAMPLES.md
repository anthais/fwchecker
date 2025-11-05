# Firewall Checker - Output Examples

## Output Formats

The tool supports 4 different output formats suitable for different purposes.

---

## 1. CSV Format (for Excel)

Perfect for importing into Excel or Google Sheets.

### Command:
```bash
./fwchecker --config=bcs-firewall.conf --format=csv
```

### Output:
```csv
Host,Port,Type,Method,Ping,Ping_Message,Telnet,Telnet_Message,Curl,Curl_Message,Result
8.8.8.8,0,external,ping,OK,"Ping successful",-,"-",-,"-",PASS
google.com,443,external,telnet,-,"-",OK,"Connection successful",-,"-",PASS
google.com,443,external,curl,-,"-",-,"-",OK,"HTTP 200 - Success",PASS
github.com,443,external,ping+telnet,OK,"Ping successful",OK,"Connection successful",-,"-",PASS
api.example.com,443,external,ping+curl,OK,"Ping successful",-,"-",OK,"HTTP 200 - Success",PASS
```

### Save to file:
```bash
./fwchecker -c bcs-firewall.conf -f csv > report.csv
```

Then open `report.csv` in Excel.

---

## 2. TSV Format (Tab-separated)

Tab-separated format, also works well with Excel.

### Command:
```bash
./fwchecker --config=bcs-firewall.conf --format=tsv
```

### Output:
```tsv
Host	Port	Type	Method	Ping	Ping_Message	Telnet	Telnet_Message	Curl	Curl_Message	Result
8.8.8.8	0	external	ping	OK	Ping successful	-	-	-	-	PASS
google.com	443	external	telnet	-	-	OK	Connection successful	-	-	PASS
google.com	443	external	curl	-	-	-	-	OK	HTTP 200 - Success	PASS
```

---

## 3. Table Format (Plain text)

Human-readable table format for reports and documentation.

### Command:
```bash
./fwchecker --config=bcs-firewall.conf --format=table
```

### Output:
```
Host                 Port   Type       Method          Ping   Ping_Message         Telnet  Telnet_Message       Curl   Curl_Message         Result  
-------------------- ------ ---------- --------------- ------ -------------------- ------- -------------------- ------ -------------------- --------
8.8.8.8              0      external   ping            OK     Ping successful      -       -                    -      -                    PASS    
google.com           443    external   telnet          -      -                    OK      Connection success   -      -                    PASS    
google.com           443    external   curl            -      -                    -       -                    OK     HTTP 200 - Success   PASS    
github.com           443    external   ping+telnet     OK     Ping successful      OK      Connection success   -      -                    PASS    

Total: 4 | Passed: 4 | Failed: 0
```

### Save to file:
```bash
./fwchecker -c bcs-firewall.conf -f table > report.txt
```

---

## 4. Pretty Format (Default)

Colorful output with progress information, best for interactive use.

### Command:
```bash
./fwchecker --config=bcs-firewall.conf
# or
./fwchecker --config=bcs-firewall.conf --format=pretty
```

### Output:
```
✓ Loaded 4 host(s) from config

============================================================
  FIREWALL CHECKER - STARTING CHECKS
============================================================

Checking 8.8.8.8:0 (external)
  ✓ PING: OK

Checking google.com:443 (external)
  ✓ TELNET: OK

Checking github.com:443 (external)
  ✓ PING: OK
  ✓ TELNET: OK

Checking api.example.com:443 (external)
  ✓ CURL: HTTP 200 - Success

Checking web.example.com:443 (external)
  ✓ PING: OK
  ✓ TELNET: OK
  ✓ CURL: HTTP 200 - Success

============================================================
  SUMMARY
============================================================

✓ PASS - 8.8.8.8:0
✓ PASS - google.com:443
✓ PASS - github.com:443
✗ FAIL - 192.168.1.20:3306

Total: 4 | Passed: 3 | Failed: 1
============================================================
```

---

## Column Descriptions

| Column | Description |
|--------|-------------|
| **Host** | IP address or hostname being checked |
| **Port** | Port number (0 for ping-only checks) |
| **Type** | `external` or `local` |
| **Method** | `ping`, `telnet`, `curl`, or combination (e.g., `ping+telnet+curl`) |
| **Ping** | Ping result: `OK`, `FAIL`, or `-` (not checked) |
| **Ping_Message** | Detailed ping status message |
| **Telnet** | Telnet result: `OK`, `FAIL`, or `-` (not checked) |
| **Telnet_Message** | Detailed telnet status message |
| **Curl** | Curl result: `OK`, `FAIL`, or `-` (not checked) |
| **Curl_Message** | Detailed curl status message with HTTP code |
| **Result** | Overall result: `PASS` or `FAIL` |

---

## Telnet Message Types

The tool provides detailed telnet connection status:

| Message | Meaning |
|---------|---------|
| **Connection successful** | Port is open and accepting connections |
| **Connection refused** | Port is closed or service not running |
| **Connection timeout** | Firewall blocking or host unreachable |
| **No route to host** | Network routing issue |
| **Cannot resolve hostname** | DNS resolution failed |
| **Connection failed** | Generic connection error |

---

## Ping Message Types

| Message | Meaning |
|---------|---------|
| **Ping successful** | Host is reachable via ICMP |
| **Cannot ping host** | Host is unreachable or blocking ICMP |

---

## Curl Message Types

| Message | Meaning |
|---------|---------|
| **HTTP 200 - Success** | HTTP request successful (2xx status) |
| **HTTP 301 - Success** | Redirect (3xx status, still reachable) |
| **HTTP 404 - Reachable but error** | Server responded with 4xx error |
| **HTTP 500 - Reachable but error** | Server responded with 5xx error |
| **Cannot resolve host** | DNS resolution failed |
| **Connection refused** | Port is closed or service not running |
| **Connection timeout** | Firewall blocking or host unreachable |
| **curl not installed** | curl command not available |

---

## Use Cases

### 1. Daily Report for Management
```bash
./fwchecker -c production.conf -f csv > daily-$(date +%Y%m%d).csv
```

### 2. Quick Visual Check
```bash
./fwchecker -c servers.conf
```

### 3. Documentation
```bash
./fwchecker -c infrastructure.conf -f table > firewall-status.txt
```

### 4. Data Analysis in Excel
```bash
./fwchecker -c all-servers.conf -f csv > analysis.csv
# Open analysis.csv in Excel, create pivot tables, charts, etc.
```

### 5. Automated Monitoring
```bash
#!/bin/bash
# Check and alert if any fails
./fwchecker -c critical.conf -f csv | grep "FAIL" && send-alert.sh
```

---

## Tips for Excel

1. **Import CSV**: 
   - Open Excel → Data → From Text/CSV
   - Select the CSV file
   - Excel will auto-detect the delimiter

2. **Copy-Paste**:
   - Run: `./fwchecker -c config.conf -f csv`
   - Select all output (Ctrl+A)
   - Copy (Ctrl+C)
   - Paste into Excel (Ctrl+V)

3. **Filter & Sort**:
   - Use Excel's filter feature on the header row
   - Sort by Result to see all FAILs first
   - Filter by Type to see only local or external hosts

4. **Conditional Formatting**:
   - Highlight cells with "FAIL" in red
   - Highlight cells with "PASS" in green

