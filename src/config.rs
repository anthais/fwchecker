use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer};
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

fn deserialize_commands<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct CommandsVisitor;

    impl<'de> Visitor<'de> for CommandsVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or an array of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value
                .lines()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect())
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(elem) = seq.next_element::<String>()? {
                vec.push(elem);
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_any(CommandsVisitor)
}

#[derive(Debug, Clone, Deserialize)]
pub struct Check {
    pub source: String,
    #[serde(deserialize_with = "deserialize_commands")]
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

