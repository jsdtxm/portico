use ssh2::Session;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::Path;
use std::time::Duration;
use crate::config::Server;

pub struct SshConnection {
    session: Session,
}

impl SshConnection {
    pub fn new(
        server: &Server,
        connect_timeout_secs: u64,
        read_timeout_secs: u64,
        write_timeout_secs: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let addr_str = format!("{}:{}", server.host, server.port);
        let addr = addr_str.to_socket_addrs()?
            .next()
            .ok_or("Could not resolve address")?;
        
        let tcp = TcpStream::connect_timeout(&addr, Duration::from_secs(connect_timeout_secs))?;
        tcp.set_read_timeout(Some(Duration::from_secs(read_timeout_secs)))?;
        tcp.set_write_timeout(Some(Duration::from_secs(write_timeout_secs)))?;
        
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
}