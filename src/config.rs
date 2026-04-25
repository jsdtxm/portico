use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub private_key: Option<String>,
    pub forwardings: Vec<Forwarding>,
}

#[derive(Debug, Deserialize, Serialize)]
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
}