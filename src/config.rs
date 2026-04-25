use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub servers: Vec<Server>,
    pub timeout: Option<TimeoutConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TimeoutConfig {
    pub connect_timeout_secs: Option<u64>,
    pub read_timeout_secs: Option<u64>,
    pub write_timeout_secs: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub private_key: Option<String>,
    pub forwardings: Vec<Forwarding>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Forwarding {
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn get_connect_timeout_secs(&self) -> u64 {
        self.timeout.as_ref()
            .and_then(|t| t.connect_timeout_secs)
            .unwrap_or(5)
    }
    
    pub fn get_read_timeout_secs(&self) -> u64 {
        self.timeout.as_ref()
            .and_then(|t| t.read_timeout_secs)
            .unwrap_or(10)
    }
    
    pub fn get_write_timeout_secs(&self) -> u64 {
        self.timeout.as_ref()
            .and_then(|t| t.write_timeout_secs)
            .unwrap_or(10)
    }
}