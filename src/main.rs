mod cli;
mod config;
mod ssh;
mod monitor;

use clap::Parser;
use cli::Cli;
use config::Config;
use ssh::SshConnection;
use monitor::{Monitor, ForwardingInfo};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    let cli = Cli::parse();
    
    println!("Loading config from: {}", cli.config_file);
    let config = match Config::load_from_file(&cli.config_file) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return;
        }
    };
    
    let monitor = Arc::new(Mutex::new(Monitor::new()));
    
    for server in &config.servers {
        println!("Connecting to server: {}", server.name);
        
        let mut connection = match SshConnection::new(
            server,
            config.get_connect_timeout_secs(),
            config.get_read_timeout_secs(),
            config.get_write_timeout_secs(),
        ) {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Error connecting to server {}: {}", server.name, e);
                continue;
            }
        };
        
        for forwarding in &server.forwardings {
            println!("Forwarding local port {} to {}:{}", 
                     forwarding.local_port, forwarding.remote_host, forwarding.remote_port);
            
            match connection.forward_port(
                forwarding.local_port,
                &forwarding.remote_host,
                forwarding.remote_port
            ) {
                Ok(_) => {
                    println!("Port forwarding established");
                    monitor.lock().unwrap().add_forwarding(ForwardingInfo {
                        local_port: forwarding.local_port,
                        remote_host: forwarding.remote_host.clone(),
                        remote_port: forwarding.remote_port,
                        server_name: server.name.clone(),
                    });
                }
                Err(e) => {
                    eprintln!("Error setting up port forwarding: {}", e);
                }
            }
        }
    }
    
    // 检查是否有活跃转发
    if !monitor.lock().unwrap().has_active_forwardings() {
        println!("No active port forwardings. Exiting.");
        return;
    }
    
    // 启动监控线程
    let monitor_clone = Arc::clone(&monitor);
    thread::spawn(move || {
        loop {
            monitor_clone.lock().unwrap().update();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    // 主循环
    loop {
        let guard = monitor.lock().unwrap();
        
        println!("\nPort forwarding status:");
        for forwarding in guard.iter_active_forwardings() {
            let unknown = "Unknown".to_string();
            let zero_traffic = (0, 0);
            let process = guard.get_process_for_port(forwarding.local_port)
                .unwrap_or(&unknown);
            let traffic = guard.get_traffic_for_port(forwarding.local_port)
                .unwrap_or(&zero_traffic);
            
            println!("localhost:{} -> {}:{} (Server: {}, Process: {}, Traffic: {} sent, {} received)", 
                     forwarding.local_port, 
                     forwarding.remote_host, 
                     forwarding.remote_port, 
                     forwarding.server_name,
                     process, traffic.0, traffic.1);
        }
        
        drop(guard);
        thread::sleep(Duration::from_secs(5));
    }
}
