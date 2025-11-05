use crate::config::Config;
use crate::executor::CommandExecutor;
use colored::*;

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub source: String,
    pub target: String,
    pub method: String,
    pub detail: String,
    pub passed: bool,
}

pub struct CheckRunner {
    config: Config,
    verbose: bool,
    executor: CommandExecutor,
}

impl CheckRunner {
    pub fn new(config: Config, verbose: bool) -> Self {
        let executor = CommandExecutor::new(verbose);
        Self {
            config,
            verbose,
            executor,
        }
    }

    pub async fn run_all_checks(&mut self) -> Vec<CheckResult> {
        let mut results = Vec::new();

        if self.verbose {
            println!("{}", "Starting checks...".blue().bold());
        }

        // Clone checks to avoid borrow conflicts
        let checks = self.config.checks.clone();

        for (source_host_name, commands) in &checks {
            if let Some(source_host) = self.config.hosts.get(source_host_name) {
                println!("\n{} {} ({})", 
                         "Checking from:".bold(),
                         source_host_name,
                         format!("{:?}", source_host.host_type).to_lowercase());

                for cmd in commands {
                    if cmd.is_newline {
                        println!();
                        continue;
                    }

                    let result = self.process_command(source_host_name, &cmd.line).await;
                    if let Some(r) = result {
                        results.push(r);
                    }
                }
            }
        }

        results
    }

    async fn process_command(&mut self, source_host: &str, cmd_line: &str) -> Option<CheckResult> {
        let parts: Vec<&str> = cmd_line.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "ping" => {
                if parts.len() < 2 {
                    return None;
                }
                self.check_ping(source_host, parts[1]).await
            }
            "telnet" => {
                if parts.len() < 3 {
                    return None;
                }
                self.check_telnet(source_host, parts[1], parts[2]).await
            }
            "curl" => {
                let curl_args = cmd_line.strip_prefix("curl ").unwrap_or("");
                self.check_curl(source_host, curl_args).await
            }
            _ => None,
        }
    }

    async fn check_ping(&self, source: &str, target: &str) -> Option<CheckResult> {
        let resolved = self.config.resolve_host(target)?;
        
        if self.verbose {
            println!("  {} ping {} -> {}", "[DEBUG]".blue(), source, target);
        }

        let source_host = self.config.hosts.get(source)?;
        let cmd = format!("ping -c 3 -W 2 {}", resolved);
        
        let success = self.executor.execute_command(source_host, &cmd).await;

        let result = CheckResult {
            source: source.to_string(),
            target: target.to_string(),
            method: "ping".to_string(),
            detail: if success { "OK".to_string() } else { "Failed".to_string() },
            passed: success,
        };

        if success {
            println!("  {} {} -> {}", "✓ PING".green(), source, target);
        } else {
            println!("  {} {} -> {}", "✗ PING".red(), source, target);
        }

        Some(result)
    }

    async fn check_telnet(&self, source: &str, target: &str, port: &str) -> Option<CheckResult> {
        let resolved = self.config.resolve_host(target)?;
        
        if self.verbose {
            println!("  {} telnet {} -> {}:{}", "[DEBUG]".blue(), source, target, port);
        }

        let source_host = self.config.hosts.get(source)?;
        
        // Check if port needs to be opened
        if let Some(target_host) = self.config.hosts.get(target) {
            if let Ok(port_num) = port.parse::<u16>() {
                if target_host.open_service_ports.contains(&port_num) {
                    // TODO: Implement port opening via SSH
                    if self.verbose {
                        println!("  {} Port {} needs to be opened", "[DEBUG]".blue(), port);
                    }
                }
            }
        }

        let cmd = format!("timeout 5 telnet {} {} < /dev/null 2>&1", resolved, port);
        let success = self.executor.execute_command(source_host, &cmd).await;

        let result = CheckResult {
            source: source.to_string(),
            target: format!("{}:{}", target, port),
            method: "telnet".to_string(),
            detail: if success { "Connected".to_string() } else { "Failed".to_string() },
            passed: success,
        };

        if success {
            println!("  {} {} -> {}:{}", "✓ TELNET".green(), source, target, port);
        } else {
            println!("  {} {} -> {}:{}", "✗ TELNET".red(), source, target, port);
        }

        Some(result)
    }

    async fn check_curl(&self, source: &str, curl_args: &str) -> Option<CheckResult> {
        if self.verbose {
            println!("  {} curl {} -> {}", "[DEBUG]".blue(), source, curl_args);
        }

        let source_host = self.config.hosts.get(source)?;
        let cmd = format!("curl {}", curl_args);
        
        let success = self.executor.execute_command(source_host, &cmd).await;

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
        };

        if success {
            println!("  {} {} -> {}", "✓ CURL".green(), source, url);
        } else {
            println!("  {} {} -> {}", "✗ CURL".red(), source, url);
        }

        Some(result)
    }
}

