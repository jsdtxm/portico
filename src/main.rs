mod cli;
mod config;
mod ssh;
mod monitor;
mod tui;

use clap::Parser;
use cli::Cli;
use config::Config;
use ssh::SshConnection;
use monitor::Monitor;
use tui::App;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    
    println!("Loading config from: {}", cli.config_file);
    let config = match Config::load_from_file(&cli.config_file) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return Ok(());
        }
    };
    
    let monitor = Arc::new(Mutex::new(Monitor::new()));
    
    for server in &config.servers {
        println!("Connecting to server: {}", server.name);
        
        // Add server to monitor first
        monitor.lock().unwrap().add_server(
            server.name.clone(),
            server.host.clone(),
            server.port,
            server.username.clone(),
        );
        
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
                    monitor.lock().unwrap().add_forwarding_to_server(
                        &server.name,
                        forwarding.local_port,
                        forwarding.remote_host.clone(),
                        forwarding.remote_port,
                    );
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
        return Ok(());
    }
    
    // 启动监控线程
    let monitor_clone = Arc::clone(&monitor);
    thread::spawn(move || {
        loop {
            monitor_clone.lock().unwrap().update();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    // 启动TUI
    let mut app = App::new(monitor);
    app.run()?;
    
    Ok(())
}
