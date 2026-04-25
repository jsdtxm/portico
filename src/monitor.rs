use sysinfo::{System, SystemExt};
use std::collections::HashMap;
use std::time::Instant;

pub struct Monitor {
    system: System,
    port_process_map: HashMap<u16, String>,
    port_traffic_map: HashMap<u16, (u64, u64)>, // (sent, received)
    last_update: Instant,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            port_process_map: HashMap::new(),
            port_traffic_map: HashMap::new(),
            last_update: Instant::now(),
        }
    }
    
    pub fn update(&mut self) {
        self.system.refresh_all();
        self.update_port_process_map();
        // 这里可以添加流量监控逻辑
        self.last_update = Instant::now();
    }
    
    fn update_port_process_map(&mut self) {
        // 这里需要实现获取端口对应的进程名称的逻辑
        // 简化版本：仅作示例
        for _process in self.system.processes().values() {
            // 实际实现需要获取进程打开的端口
            // 这里简化处理
        }
    }
    
    pub fn get_process_for_port(&self, port: u16) -> Option<&String> {
        self.port_process_map.get(&port)
    }
    
    pub fn get_traffic_for_port(&self, port: u16) -> Option<&(u64, u64)> {
        self.port_traffic_map.get(&port)
    }
    
    pub fn add_port(&mut self, port: u16) {
        self.port_process_map.entry(port).or_insert("Unknown".to_string());
        self.port_traffic_map.entry(port).or_insert((0, 0));
    }
}