use crate::config::Config;
use crate::executor::CommandExecutor;
use colored::*;
use futures::stream::{self, StreamExt};

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub source: String,
    pub target: String,
    pub method: String,
    pub detail: String,
    pub passed: bool,
    pub order: usize, // Order in config file for sorting
}

pub struct CheckRunner {
    config: Config,
    verbose: bool,
    executor: CommandExecutor,
    max_concurrent: usize,
}

impl CheckRunner {
    pub fn new(config: Config, verbose: bool, max_concurrent: usize) -> Self {
        let executor = CommandExecutor::new(verbose);
        Self {
            config,
            verbose,
            executor,
            max_concurrent,
        }
    }

    pub async fn run_all_checks(&mut self) -> Vec<CheckResult> {
        if self.verbose {
            println!("{}", "Starting checks...".blue().bold());
        }

        // Clone checks to avoid borrow conflicts
        let checks = self.config.checks.clone();
        
        // Collect all commands from all sources with their order
        let mut all_tasks = Vec::new();
        let mut global_order = 0;
        let config_ref = &self.config;
        let executor_ref = &self.executor;
        let verbose = self.verbose;
        let max_concurrent = self.max_concurrent;
        let start_time = std::time::Instant::now();

        for (source_host_name, commands) in &checks {
            if let Some(_source_host) = self.config.hosts.get(source_host_name) {
                // Filter out newlines and collect commands
                let valid_commands: Vec<&str> = commands.iter()
                    .filter(|cmd| !cmd.is_newline)
                    .map(|cmd| cmd.line.as_str())
                    .collect();

                if valid_commands.is_empty() {
                    continue;
                }

                let source = source_host_name.clone();
                let base_order = global_order;
                
                // Create futures for all commands from this source
                for (idx, cmd_line) in valid_commands.iter().enumerate() {
                    let source = source.clone();
                    let cmd_line = (*cmd_line).to_string();
                    let order = base_order + idx;
                    
                    let task = async move {
                        let task_start = std::time::Instant::now();
                        if verbose {
                            eprintln!("  {} [Task {}] Starting at {:.2}s: {} -> {}", 
                                     "[DEBUG]".blue(), 
                                     order + 1,
                                     start_time.elapsed().as_secs_f64(),
                                     source,
                                     cmd_line);
                        }
                        let mut result = Self::process_command_parallel(
                            config_ref,
                            executor_ref,
                            &source,
                            &cmd_line,
                            verbose
                        ).await;
                        // Set order in result
                        if let Some(ref mut r) = result {
                            r.order = order;
                        }
                        if verbose {
                            eprintln!("  {} [Task {}] Completed at {:.2}s (took {:.2}s): {} -> {}", 
                                     "[DEBUG]".blue(), 
                                     order + 1,
                                     start_time.elapsed().as_secs_f64(),
                                     task_start.elapsed().as_secs_f64(),
                                     source,
                                     cmd_line);
                        }
                        result
                    };
                    
                    all_tasks.push(task);
                }
                
                global_order += valid_commands.len();
            }
        }
        
        if self.verbose {
            println!("  {} Running {} total commands with max_concurrent = {}", 
                     "[DEBUG]".blue(), all_tasks.len(), max_concurrent);
        }

        // Execute all tasks from all sources in parallel with concurrency limit
        let command_results: Vec<_> = stream::iter(all_tasks)
            .buffer_unordered(max_concurrent)
            .collect()
            .await;

        // Collect and sort results by original order
        let mut results: Vec<_> = command_results
            .into_iter()
            .filter_map(|r| r)
            .collect();
        
        // Sort by order to maintain config file order
        results.sort_by_key(|r| r.order);

        results
    }

    async fn process_command_parallel(
        config: &Config,
        executor: &CommandExecutor,
        source_host: &str,
        cmd_line: &str,
        verbose: bool,
    ) -> Option<CheckResult> {
        let parts: Vec<&str> = cmd_line.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "ping" => {
                if parts.len() < 2 {
                    return None;
                }
                Self::check_ping_parallel(config, executor, source_host, parts[1], verbose).await
            }
            "telnet" => {
                if parts.len() < 3 {
                    return None;
                }
                Self::check_telnet_parallel(config, executor, source_host, parts[1], parts[2], verbose).await
            }
            "curl" => {
                let curl_args = cmd_line.strip_prefix("curl ").unwrap_or("");
                Self::check_curl_parallel(config, executor, source_host, curl_args, verbose).await
            }
            _ => None,
        }
    }

    async fn check_ping_parallel(
        config: &Config,
        executor: &CommandExecutor,
        source: &str,
        target: &str,
        verbose: bool,
    ) -> Option<CheckResult> {
        let resolved = config.resolve_host(target)?;
        
        if verbose {
            println!("  {} ping {} -> {}", "[DEBUG]".blue(), source, target);
        }

        let source_host = config.hosts.get(source)?;
        let cmd = format!("ping -c 3 -W 2 {}", resolved);
        
        let success = executor.execute_command(source_host, &cmd).await;

        let result = CheckResult {
            source: source.to_string(),
            target: target.to_string(),
            method: "ping".to_string(),
            detail: if success { "OK".to_string() } else { "Failed".to_string() },
            passed: success,
            order: 0, // Will be set by caller
        };

        if success {
            println!("  {} {} -> {}", "✓ PING".green(), source, target);
        } else {
            println!("  {} {} -> {}", "✗ PING".red(), source, target);
        }

        Some(result)
    }


    async fn check_telnet_parallel(
        config: &Config,
        executor: &CommandExecutor,
        source: &str,
        target: &str,
        port: &str,
        verbose: bool,
    ) -> Option<CheckResult> {
        let resolved = config.resolve_host(target)?;
        
        if verbose {
            println!("  {} telnet {} -> {}:{}", "[DEBUG]".blue(), source, target, port);
        }

        let source_host = config.hosts.get(source)?;
        
        // Check if port needs to be opened
        let target_host = Self::find_target_host_parallel(config, target);
        let mut nc_pid: Option<String> = None;
        
        if let Some(ref target_host) = target_host {
            if let Ok(port_num) = port.parse::<u16>() {
                if target_host.open_service_ports.contains(&port_num) {
                    if verbose {
                        println!("  {} Opening port {} on target host", "[DEBUG]".blue(), port);
                    }
                    // Open port using nc on target host
                    if let Some(pid) = Self::open_port_on_host_parallel(executor, target_host, port_num, verbose).await {
                        nc_pid = Some(pid);
                        // Wait a bit for port to be ready (longer for privileged ports)
                        let wait_time = if port_num < 1024 { 1000 } else { 500 };
                        tokio::time::sleep(tokio::time::Duration::from_millis(wait_time)).await;
                    }
                }
            }
        }

        let cmd = format!("timeout 5 telnet {} {} < /dev/null 2>&1", resolved, port);
        let (exit_success, output) = executor.execute_command_with_output(source_host, &cmd).await;
        
        // Cleanup: kill nc process if we opened a port
        if let Some(ref pid) = nc_pid {
            if let Some(ref target_host) = target_host {
                Self::close_port_on_host_parallel(executor, target_host, pid, verbose).await;
            }
        }
        
        // Telnet can exit with status 1 even when connection succeeds.
        // Check for "Connected to" in output as a success indicator.
        let success = exit_success || output.contains("Connected to");
        
        // Extract error message from output if connection failed
        let detail = if success {
            "Connected".to_string()
        } else {
            Self::extract_telnet_error(&output)
        };

        let result = CheckResult {
            source: source.to_string(),
            target: format!("{}:{}", target, port),
            method: "telnet".to_string(),
            detail,
            passed: success,
            order: 0, // Will be set by caller
        };

        if success {
            println!("  {} {} -> {}:{}", "✓ TELNET".green(), source, target, port);
        } else {
            println!("  {} {} -> {}:{}", "✗ TELNET".red(), source, target, port);
        }

        Some(result)
    }


    async fn check_curl_parallel(
        config: &Config,
        executor: &CommandExecutor,
        source: &str,
        curl_args: &str,
        verbose: bool,
    ) -> Option<CheckResult> {
        if verbose {
            println!("  {} curl {} -> {}", "[DEBUG]".blue(), source, curl_args);
        }

        let source_host = config.hosts.get(source)?;
        let cmd = format!("curl {}", curl_args);
        
        let success = executor.execute_command(source_host, &cmd).await;

        // Extract URL for display
        let url = curl_args.split_whitespace()
            .find(|s| s.starts_with("http"))
            .unwrap_or(curl_args);

        let result = CheckResult {
            source: source.to_string(),
            target: url.to_string(),
            method: "curl".to_string(),
            detail: if success { "Success".to_string() } else { "Failed".to_string() },
            passed: success,
            order: 0, // Will be set by caller
        };

        if success {
            println!("  {} {} -> {}", "✓ CURL".green(), source, url);
        } else {
            println!("  {} {} -> {}", "✗ CURL".red(), source, url);
        }

        Some(result)
    }


    fn extract_telnet_error(output: &str) -> String {
        // Common telnet error patterns to extract
        let error_patterns = vec![
            "Connection refused",
            "Connection timed out",
            "No route to host",
            "Network is unreachable",
            "Name or service not known",
            "Connection closed",
            "Connection reset",
            "Host unreachable",
        ];

        // Look for error patterns in the output
        for pattern in &error_patterns {
            if output.contains(pattern) {
                return pattern.to_string();
            }
        }

        // If no specific error found, try to extract from common error formats
        // Look for "telnet: " prefix which often contains error messages
        if let Some(start) = output.find("telnet: ") {
            if let Some(end) = output[start..].find('\n') {
                let error_msg = output[start + 8..start + end].trim();
                if !error_msg.is_empty() {
                    return error_msg.to_string();
                }
            }
        }

        // Default fallback
        "Failed".to_string()
    }

    fn find_target_host_parallel<'a>(config: &'a Config, target: &str) -> Option<&'a crate::config::Host> {
        // First try direct host name lookup
        if let Some(host) = config.hosts.get(target) {
            return Some(host);
        }
        
        // If target is an IP, try to find matching host by IP
        if target.contains('.') {
            for (_, host) in &config.hosts {
                if let Some(host_ip) = &host.ip {
                    if host_ip == target {
                        return Some(host);
                    }
                }
            }
        }
        
        None
    }


    async fn open_port_on_host_parallel(
        executor: &CommandExecutor,
        host: &crate::config::Host,
        port: u16,
        verbose: bool,
    ) -> Option<String> {
        // Only open ports on remote hosts
        if host.host_type != crate::config::HostType::Remote {
            return None;
        }

        // Build sudo command with password if available
        // Use echo "password" | sudo -S command to pass password to sudo
        let sudo_prefix = if port < 1024 {
            if let Some(ref password) = host.password {
                // Escape special characters in password for shell
                let escaped_pwd = password.replace('\\', "\\\\")
                    .replace('$', "\\$")
                    .replace('`', "\\`")
                    .replace('"', "\\\"")
                    .replace('\'', "\\'");
                format!("echo '{}' | sudo -S", escaped_pwd)
            } else {
                "sudo".to_string()
            }
        } else {
            String::new()
        };

        // Use nc to listen on the port in background
        // Use -k to keep listening after connection closes (if supported)
        // Bind to 0.0.0.0 to accept connections from any interface
        let nc_cmd = if port < 1024 {
            format!("{} nc -l -k 0.0.0.0 {}", sudo_prefix, port)
        } else {
            format!("nc -l -k 0.0.0.0 {}", port)
        };
        
        let cmd = format!("{} > /dev/null 2>&1 & sleep 0.3; netstat -tlnp 2>/dev/null | grep ':{} ' | awk '{{print $7}}' | cut -d'/' -f1 | head -1", nc_cmd, port);
        
        let (success, output) = executor.execute_command_with_output(host, &cmd).await;
        
        if success {
            let pid = output.trim().to_string();
            if !pid.is_empty() && pid.chars().all(|c| c.is_ascii_digit()) {
                if verbose {
                    println!("  {} Started nc listener on port {} (PID: {})", "[DEBUG]".blue(), port, pid);
                }
                return Some(pid);
            }
        }
        
        // If -k not supported, try without -k
        let nc_cmd = if port < 1024 {
            format!("{} nc -l 0.0.0.0 {}", sudo_prefix, port)
        } else {
            format!("nc -l 0.0.0.0 {}", port)
        };
        let cmd = format!("{} > /dev/null 2>&1 & sleep 0.3; netstat -tlnp 2>/dev/null | grep ':{} ' | awk '{{print $7}}' | cut -d'/' -f1 | head -1", nc_cmd, port);
        let (success, output) = executor.execute_command_with_output(host, &cmd).await;
        if success {
            let pid = output.trim().to_string();
            if !pid.is_empty() && pid.chars().all(|c| c.is_ascii_digit()) {
                if verbose {
                    println!("  {} Started nc listener on port {} (PID: {})", "[DEBUG]".blue(), port, pid);
                }
                return Some(pid);
            }
        }
        
        // Last resort: try without sudo for privileged ports (might work with capabilities)
        if port < 1024 {
            let cmd = format!("nc -l 0.0.0.0 {} > /dev/null 2>&1 & sleep 0.3; netstat -tlnp 2>/dev/null | grep ':{} ' | awk '{{print $7}}' | cut -d'/' -f1 | head -1", port, port);
            let (success, output) = executor.execute_command_with_output(host, &cmd).await;
            if success {
                let pid = output.trim().to_string();
                if !pid.is_empty() && pid.chars().all(|c| c.is_ascii_digit()) {
                    if verbose {
                        println!("  {} Started nc listener on port {} (PID: {})", "[DEBUG]".blue(), port, pid);
                    }
                    return Some(pid);
                }
            }
        }
        
        None
    }


    async fn close_port_on_host_parallel(
        executor: &CommandExecutor,
        host: &crate::config::Host,
        pid: &str,
        verbose: bool,
    ) {
        if host.host_type != crate::config::HostType::Remote {
            return;
        }

        // Use sudo with password if available for killing processes
        let kill_cmd = if let Some(ref password) = host.password {
            let escaped_pwd = password.replace('\\', "\\\\")
                .replace('$', "\\$")
                .replace('`', "\\`")
                .replace('"', "\\\"")
                .replace('\'', "\\'");
            format!("echo '{}' | sudo -S kill {} 2>/dev/null || kill {} 2>/dev/null || true", escaped_pwd, pid, pid)
        } else {
            format!("sudo kill {} 2>/dev/null || kill {} 2>/dev/null || true", pid, pid)
        };
        
        let (success, _) = executor.execute_command_with_output(host, &kill_cmd).await;
        
        if verbose {
            if success {
                println!("  {} Closed port (killed PID: {})", "[DEBUG]".blue(), pid);
            } else {
                println!("  {} Failed to close port (PID: {})", "[DEBUG]".blue(), pid);
            }
        }
    }

}

