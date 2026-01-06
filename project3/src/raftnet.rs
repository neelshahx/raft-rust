use crate::config::RAFT_SERVERS;
use crate::network::{configure_stream, make_streaming_socket};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct RaftNet {
    server_id: usize,
    listener: TcpListener,
}

impl RaftNet {
    pub fn new(server_id: usize) -> Self {
        let ip_port = RAFT_SERVERS[server_id].1;
        println!("Raft server listening on {}", ip_port);
        let socket = make_streaming_socket(ip_port).expect("Failed to make socket");
        RaftNet {
            server_id,
            listener: socket.into(),
        }
    }

    pub fn send(&self, dest: usize, msg: &str) -> std::io::Result<()> {
        let ip_port = RAFT_SERVERS[usize::from(dest)].1;
        let mut stream = TcpStream::connect(ip_port).expect("Failed to connect");
        configure_stream(&stream).expect("Failed to configure");
        stream.write_all(msg.as_bytes())?;
        Ok(())
    }
    pub fn receive(&self) -> std::io::Result<String> {
        let (mut stream, _addr) = self.listener.accept()?;
        configure_stream(&stream).expect("Failed to configure");
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf[..n]).to_string())
    }
}
