use std::process::{Command, Child, Stdio};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::config::Server;

fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~") {
        if let Some(home) = std::env::home_dir() {
            if path.len() == 1 {
                return home;
            }
            if let Some(rest) = path.strip_prefix("~/") {
                return home.join(rest);
            }
        }
    }
    PathBuf::from(path)
}

#[derive(Clone)]
pub struct SshConnection {
    server: Server,
    children: Arc<Mutex<Vec<Child>>>,
}

impl SshConnection {
    pub fn new(
        server: &Server,
        _connect_timeout_secs: u64,
        _read_timeout_secs: u64,
        _write_timeout_secs: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            server: server.clone(),
            children: Arc::new(Mutex::new(Vec::new())),
        })
    }
    
    pub fn forward_port(
        &self,
        local_port: u16,
        remote_host: String,
        remote_port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::new("ssh");
        
        // 端口转发配置: -L local_port:remote_host:remote_port
        cmd.arg("-L");
        cmd.arg(format!("{}:{}:{}", local_port, remote_host, remote_port));
        
        // 连接选项
        cmd.arg("-N"); // 不执行远程命令
        cmd.arg("-q"); // 安静模式
        cmd.arg("-o");
        cmd.arg("StrictHostKeyChecking=no");
        cmd.arg("-o");
        cmd.arg("UserKnownHostsFile=/dev/null");
        cmd.arg("-o");
        cmd.arg("LogLevel=QUIET");
        cmd.arg("-o");
        cmd.arg("BatchMode=yes");
        
        // 服务器信息
        cmd.arg(format!("{}@{}", self.server.username, self.server.host));
        cmd.arg("-p");
        cmd.arg(self.server.port.to_string());
        
        // 私钥选项
        if let Some(private_key) = &self.server.private_key {
            cmd.arg("-i");
            cmd.arg(expand_tilde(private_key));
        }
        
        // 重定向输出
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
        
        // 启动 SSH 进程并保存句柄
        let child = cmd.spawn()?;
        self.children.lock().unwrap().push(child);
        
        Ok(())
    }
    
    pub fn stop_all(&self) {
        let mut children = self.children.lock().unwrap();
        for child in children.iter_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
        children.clear();
    }
}

impl Drop for SshConnection {
    fn drop(&mut self) {
        self.stop_all();
    }
}