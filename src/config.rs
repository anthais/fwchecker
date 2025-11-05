use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HostType {
    Local,
    Remote,
    External,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Host {
    pub name: String,
    #[serde(rename = "type")]
    pub host_type: HostType,
    pub ip: Option<String>,
    #[serde(default = "default_port")]
    pub port: u16,
    pub user: Option<String>,
    pub password: Option<String>,
    #[serde(default)]
    pub open_service_ports: Vec<u16>,
}

fn default_port() -> u16 {
    22
}

#[derive(Debug, Clone, Deserialize)]
pub struct Check {
    pub source: String,
    pub commands: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    host: Vec<Host>,
    check: Vec<Check>,
}

#[derive(Debug, Clone)]
pub struct CheckCommand {
    pub line: String,
    pub is_newline: bool,
}

#[derive(Debug)]
pub struct Config {
    pub hosts: HashMap<String, Host>,
    pub checks: Vec<(String, Vec<CheckCommand>)>, // (source_host, commands)
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;

        let toml_config: TomlConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML config: {}", path))?;

        let mut hosts = HashMap::new();
        for host in toml_config.host {
            hosts.insert(host.name.clone(), host);
        }

        let mut checks = Vec::new();
        for check in toml_config.check {
            let commands: Vec<CheckCommand> = check.commands
                .into_iter()
                .map(|cmd| {
                    if cmd == "newline" {
                        CheckCommand {
                            line: String::new(),
                            is_newline: true,
                        }
                    } else {
                        CheckCommand {
                            line: cmd,
                            is_newline: false,
                        }
                    }
                })
                .collect();
            checks.push((check.source, commands));
        }

        if hosts.is_empty() {
            return Err(anyhow::anyhow!("No hosts defined in config"));
        }

        if checks.is_empty() {
            return Err(anyhow::anyhow!("No checks defined in config"));
        }

        Ok(Config { hosts, checks })
    }

    pub fn resolve_host(&self, name: &str) -> Option<String> {
        // Check if it's an IP address
        if name.contains('.') && name.chars().all(|c| c.is_digit(10) || c == '.') {
            return Some(name.to_string());
        }

        // Try to resolve from hosts
        if let Some(host) = self.hosts.get(name) {
            if let Some(ip) = &host.ip {
                return Some(ip.clone());
            }
        }

        // Return as-is for DNS resolution
        Some(name.to_string())
    }
}

