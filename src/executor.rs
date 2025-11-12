use crate::config::{Host, HostType};
use colored::*;
use std::process::Command;
use ssh2::Session;
use std::net::TcpStream;
use std::io::Read;

pub struct CommandExecutor {
    verbose: bool,
}

impl CommandExecutor {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    pub async fn execute_command(&self, host: &Host, cmd: &str) -> bool {
        match host.host_type {
            HostType::Local => self.execute_local(cmd),
            HostType::Remote => self.execute_remote(host, cmd).await,
            HostType::External => {
                eprintln!("{} Cannot execute commands on external hosts", 
                         "Warning:".yellow());
                false
            }
        }
    }

    pub async fn execute_command_with_output(&self, host: &Host, cmd: &str) -> (bool, String) {
        match host.host_type {
            HostType::Local => {
                let output = self.execute_local_with_output(cmd);
                (output.0, output.1)
            }
            HostType::Remote => {
                match self.execute_remote_with_output(host, cmd).await {
                    Ok(result) => result,
                    Err(_) => (false, String::new()),
                }
            }
            HostType::External => {
                eprintln!("{} Cannot execute commands on external hosts", 
                         "Warning:".yellow());
                (false, String::new())
            }
        }
    }

    fn execute_local(&self, cmd: &str) -> bool {
        self.execute_local_with_output(cmd).0
    }

    fn execute_local_with_output(&self, cmd: &str) -> (bool, String) {
        if self.verbose {
            println!("  {} Executing locally: {}", "[DEBUG]".blue(), cmd);
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output();

        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout).to_string();
                let stderr = String::from_utf8_lossy(&result.stderr).to_string();
                let combined = if stderr.is_empty() {
                    stdout
                } else {
                    format!("{}\n{}", stdout, stderr)
                };
                (result.status.success(), combined)
            }
            Err(e) => {
                if self.verbose {
                    eprintln!("  {} Local execution failed: {}", "[ERROR]".red(), e);
                }
                (false, String::new())
            }
        }
    }

    async fn execute_remote(&self, host: &Host, cmd: &str) -> bool {
        self.execute_remote_with_output(host, cmd).await
            .map(|(success, _)| success)
            .unwrap_or(false)
    }

    async fn execute_remote_with_output(&self, host: &Host, cmd: &str) -> Result<(bool, String), Box<dyn std::error::Error + Send + Sync>> {
        let ip = match &host.ip {
            Some(i) => i,
            None => {
                if self.verbose {
                    eprintln!("  {} No IP specified for remote host", "[ERROR]".red());
                }
                return Err("No IP specified".into());
            }
        };

        if self.verbose {
            println!("  {} Executing via SSH on {}: {}", "[DEBUG]".blue(), ip, cmd);
        }

        // Connect via SSH in a blocking task to avoid blocking the async runtime
        let ip = ip.clone();
        let port = host.port;
        let user = host.user.clone();
        let password = host.password.clone();
        let cmd = cmd.to_string();
        let verbose = self.verbose;
        
        tokio::task::spawn_blocking(move || {
            Self::ssh_execute_blocking(&ip, port, &user, &password, &cmd, verbose)
        }).await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
            format!("Task join error: {}", e).into()
        })?
    }
    
    fn ssh_execute_blocking(
        ip: &str,
        port: u16,
        user: &Option<String>,
        password: &Option<String>,
        cmd: &str,
        verbose: bool,
    ) -> Result<(bool, String), Box<dyn std::error::Error + Send + Sync>> {
        if verbose {
            println!(
                "  {} Connecting to SSH {}:{} as user: {:?}, password: {:?}",
                "[DEBUG]".blue(),
                ip,
                port,
                user,
                password.as_ref().map(|_| "****").unwrap_or("None")
            );
            println!("  {} Will execute command: {}", "[DEBUG]".blue(), cmd);
        }
        // Connect to SSH server
        let tcp = TcpStream::connect(format!("{}:{}", ip, port))?;
        if verbose {
            println!("  {} TCP connected to {}:{}", "[DEBUG]".blue(), ip, port);
        }
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        if verbose {
            println!("  {} SSH handshake complete", "[DEBUG]".blue());
        }

        // Authenticate
        match (user, password) {
            (Some(u), Some(p)) => {
                if verbose {
                    println!("  {} Authenticating with password for user {}", "[DEBUG]".blue(), u);
                }
                sess.userauth_password(u, p)?;
            }
            (Some(u), None) => {
                if verbose {
                    println!("  {} Authenticating with SSH agent for user {}", "[DEBUG]".blue(), u);
                }
                // Try key-based auth
                sess.userauth_agent(u)?;
            }
            (None, _) => {
                // Try current user
                let current_user = std::env::var("USER").unwrap_or_else(|_| "root".to_string());
                if verbose {
                    println!("  {} Authenticating with SSH agent for current user: {}", "[DEBUG]".blue(), current_user);
                }
                sess.userauth_agent(&current_user)?;
            }
        }

        if !sess.authenticated() {
            if verbose {
                println!("  {} SSH authentication failed", "[ERROR]".red());
            }
            return Err("SSH authentication failed".into());
        } else if verbose {
            println!("  {} SSH authentication successful", "[DEBUG]".blue());
        }

        // Execute command
        let mut channel = sess.channel_session()?;
        if verbose {
            println!("  {} Opening SSH channel session", "[DEBUG]".blue());
        }
        channel.exec(cmd)?;
        if verbose {
            println!("  {} Command sent, reading output...", "[DEBUG]".blue());
        }
        
        let mut output = String::new();
        channel.read_to_string(&mut output)?;
        channel.wait_close()?;
        
        let exit_status = channel.exit_status()?;
        if verbose {
            println!(
                "  {} Command output: {}",
                "[DEBUG]".blue(),
                output.trim()
            );
            println!(
                "  {} SSH exit status: {}",
                "[DEBUG]".blue(),
                exit_status
            );
        }

        Ok((exit_status == 0, output))
    }
}

