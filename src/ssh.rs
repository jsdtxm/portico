use ssh2::Session;
use std::net::TcpStream;
use std::path::Path;
use crate::config::Server;

pub struct SshConnection {
    session: Session,
}

impl SshConnection {
    pub fn new(server: &Server) -> Result<Self, Box<dyn std::error::Error>> {
        let tcp = TcpStream::connect((server.host.as_str(), server.port))?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        
        if let Some(password) = &server.password {
            session.userauth_password(&server.username, password)?;
        } else if let Some(private_key) = &server.private_key {
            session.userauth_pubkey_file(
                &server.username,
                None,
                Path::new(private_key),
                None,
            )?;
        } else {
            return Err("No authentication method provided".into());
        }
        
        Ok(Self {
            session,
        })
    }
    
    pub fn forward_port(
        &mut self,
        _local_port: u16,
        remote_host: &str,
        remote_port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.session.channel_direct_tcpip(remote_host, remote_port, None)?;
        Ok(())
    }
    
    pub fn is_connected(&self) -> bool {
        // 简单的连接状态检查
        true
    }
}