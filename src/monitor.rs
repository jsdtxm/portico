use sysinfo::{System, SystemExt};
use std::collections::HashMap;
use std::time::{Instant, Duration};

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub forwardings: Vec<ForwardingInfo>,
}

#[derive(Debug, Clone)]
pub struct ForwardingInfo {
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
    pub status: ForwardingStatus,
    pub created_at: Instant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForwardingStatus {
    Active,
    Error,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct TrafficStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub last_update: Instant,
}

pub struct Monitor {
    system: System,
    port_process_map: HashMap<u16, String>,
    port_traffic_map: HashMap<u16, TrafficStats>,
    servers: Vec<ServerInfo>,
    last_update: Instant,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            port_process_map: HashMap::new(),
            port_traffic_map: HashMap::new(),
            servers: Vec::new(),
            last_update: Instant::now(),
        }
    }
    
    pub fn update(&mut self) {
        self.system.refresh_all();
        self.update_port_process_map();
        self.last_update = Instant::now();
    }
    
    fn update_port_process_map(&mut self) {
        for _process in self.system.processes().values() {
        }
    }
    
    pub fn get_process_for_port(&self, port: u16) -> Option<&String> {
        self.port_process_map.get(&port)
    }
    
    pub fn get_traffic_for_port(&self, port: u16) -> Option<&TrafficStats> {
        self.port_traffic_map.get(&port)
    }
    
    pub fn add_server(&mut self, name: String, host: String, port: u16, username: String) {
        self.servers.push(ServerInfo {
            name,
            host,
            port,
            username,
            forwardings: Vec::new(),
        });
    }
    
    pub fn add_forwarding_to_server(&mut self, server_name: &str, local_port: u16, remote_host: String, remote_port: u16) {
        if let Some(server) = self.servers.iter_mut().find(|s| s.name == server_name) {
            self.port_process_map.entry(local_port).or_insert("Unknown".to_string());
            self.port_traffic_map.entry(local_port).or_insert(TrafficStats {
                bytes_sent: 0,
                bytes_received: 0,
                last_update: Instant::now(),
            });
            server.forwardings.push(ForwardingInfo {
                local_port,
                remote_host,
                remote_port,
                status: ForwardingStatus::Active,
                created_at: Instant::now(),
            });
        }
    }
    
    pub fn has_active_forwardings(&self) -> bool {
        self.servers.iter().any(|s| !s.forwardings.is_empty())
    }
    
    pub fn iter_servers(&self) -> std::slice::Iter<'_, ServerInfo> {
        self.servers.iter()
    }
    
    pub fn get_uptime(&self, created_at: Instant) -> Duration {
        created_at.elapsed()
    }
}

impl Default for Monitor {
    fn default() -> Self {
        Self::new()
    }
}