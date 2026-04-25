use sysinfo::{System, SystemExt};
use std::collections::HashMap;
use std::time::{Instant, Duration};

#[derive(Debug, Clone)]
pub struct ForwardingInfo {
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
    pub server_name: String,
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
    active_forwardings: Vec<ForwardingInfo>,
    last_update: Instant,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            port_process_map: HashMap::new(),
            port_traffic_map: HashMap::new(),
            active_forwardings: Vec::new(),
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
    
    pub fn add_forwarding(&mut self, info: ForwardingInfo) {
        self.port_process_map.entry(info.local_port).or_insert("Unknown".to_string());
        self.port_traffic_map.entry(info.local_port).or_insert(TrafficStats {
            bytes_sent: 0,
            bytes_received: 0,
            last_update: Instant::now(),
        });
        self.active_forwardings.push(info);
    }
    
    pub fn add_forwarding_simple(&mut self, local_port: u16, remote_host: String, remote_port: u16, server_name: String) {
        self.add_forwarding(ForwardingInfo {
            local_port,
            remote_host,
            remote_port,
            server_name,
            status: ForwardingStatus::Active,
            created_at: Instant::now(),
        });
    }
    
    pub fn has_active_forwardings(&self) -> bool {
        !self.active_forwardings.is_empty()
    }
    
    pub fn iter_active_forwardings(&self) -> std::slice::Iter<'_, ForwardingInfo> {
        self.active_forwardings.iter()
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