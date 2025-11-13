use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Host {
    pub name: String,
    pub user: String,
    pub host: String,
    pub port: Option<u16>,
    pub password: Option<String>,
    pub command: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub hosts: Vec<Host>,
}

impl Config {
    pub fn path() -> PathBuf {
        let mut p = home_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push(".lazyssh");
        fs::create_dir_all(&p).ok();
        p.push("config.toml");
        p
    }

    pub fn load() -> Self {
        let path = Self::path();
        if !path.exists() {
            let cfg = Config { hosts: vec![] };
            if let Err(e) = fs::write(&path, toml::to_string_pretty(&cfg).unwrap()) {
                eprintln!("Warning: Failed to create config file: {}", e);
            }
            return cfg;
        }
        
        match fs::read_to_string(&path) {
            Ok(s) => {
                toml::from_str(&s).unwrap_or_else(|e| {
                    eprintln!("Warning: Failed to parse config file: {}. Using empty config.", e);
                    Config { hosts: vec![] }
                })
            }
            Err(e) => {
                eprintln!("Warning: Failed to read config file: {}. Using empty config.", e);
                Config { hosts: vec![] }
            }
        }
    }

    pub fn save(&self) {
        let path = Self::path();
        match toml::to_string_pretty(self) {
            Ok(content) => {
                if let Err(e) = fs::write(&path, content) {
                    eprintln!("Error: Failed to save config file: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error: Failed to serialize config: {}", e);
            }
        }
    }

    pub fn add_host(&mut self, host: Host) {
        self.hosts.push(host);
    }

    pub fn remove_host(&mut self, index: usize) {
        if index < self.hosts.len() {
            self.hosts.remove(index);
        }
    }

    pub fn update_host(&mut self, index: usize, host: Host) {
        if index < self.hosts.len() {
            self.hosts[index] = host;
        }
    }
}
