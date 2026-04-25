use sysinfo::{System, SystemExt};
use std::collections::HashMap;
use std::time::Instant;

pub struct ForwardingInfo {
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
    pub server_name: String,
}

pub struct Monitor {
    system: System,
    port_process_map: HashMap<u16, String>,
    port_traffic_map: HashMap<u16, (u64, u64)>, // (sent, received)
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
    
    pub fn get_traffic_for_port(&self, port: u16) -> Option<&(u64, u64)> {
        self.port_traffic_map.get(&port)
    }
    
    pub fn add_forwarding(&mut self, info: ForwardingInfo) {
        self.port_process_map.entry(info.local_port).or_insert("Unknown".to_string());
        self.port_traffic_map.entry(info.local_port).or_insert((0, 0));
        self.active_forwardings.push(info);
    }
    
    pub fn has_active_forwardings(&self) -> bool {
        !self.active_forwardings.is_empty()
    }
    
    pub fn iter_active_forwardings(&self) -> std::slice::Iter<'_, ForwardingInfo> {
        self.active_forwardings.iter()
    }
}