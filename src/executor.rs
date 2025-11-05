use crate::config::{Host, HostType};
use colored::*;
use std::process::{Command, Stdio};
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
            HostType::Remote => self.execute_remote(host, cmd),
            HostType::External => {
                eprintln!("{} Cannot execute commands on external hosts", 
                         "Warning:".yellow());
                false
            }
        }
    }

    fn execute_local(&self, cmd: &str) -> bool {
        if self.verbose {
            println!("  {} Executing locally: {}", "[DEBUG]".blue(), cmd);
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        match output {
            Ok(status) => status.success(),
            Err(e) => {
                if self.verbose {
                    eprintln!("  {} Local execution failed: {}", "[ERROR]".red(), e);
                }
                false
            }
        }
    }

    fn execute_remote(&self, host: &Host, cmd: &str) -> bool {
        let ip = match &host.ip {
            Some(i) => i,
            None => {
                if self.verbose {
                    eprintln!("  {} No IP specified for remote host", "[ERROR]".red());
                }
                return false;
            }
        };

        if self.verbose {
            println!("  {} Executing via SSH on {}: {}", "[DEBUG]".blue(), ip, cmd);
        }

        // Connect via SSH
        match self.ssh_execute(ip, host.port, &host.user, &host.password, cmd) {
            Ok(success) => success,
            Err(e) => {
                if self.verbose {
                    eprintln!("  {} SSH execution failed: {}", "[ERROR]".red(), e);
                }
                false
            }
        }
    }

    fn ssh_execute(
        &self,
        ip: &str,
        port: u16,
        user: &Option<String>,
        password: &Option<String>,
        cmd: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Connect to SSH server
        let tcp = TcpStream::connect(format!("{}:{}", ip, port))?;
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;

        // Authenticate
        match (user, password) {
            (Some(u), Some(p)) => {
                sess.userauth_password(u, p)?;
            }
            (Some(u), None) => {
                // Try key-based auth
                sess.userauth_agent(u)?;
            }
            (None, _) => {
                // Try current user
                let current_user = std::env::var("USER").unwrap_or_else(|_| "root".to_string());
                sess.userauth_agent(&current_user)?;
            }
        }

        if !sess.authenticated() {
            return Err("SSH authentication failed".into());
        }

        // Execute command
        let mut channel = sess.channel_session()?;
        channel.exec(cmd)?;
        
        let mut output = String::new();
        channel.read_to_string(&mut output)?;
        channel.wait_close()?;
        
        let exit_status = channel.exit_status()?;

        Ok(exit_status == 0)
    }
}

