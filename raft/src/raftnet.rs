use crate::shared::RAFT_SERVERS;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct RaftNet {
    listener: TcpListener,
}

// TODO
// connection reuse
// should receive run in a loop from caller?
// should receive use incoming instead?
impl RaftNet {
    pub(crate) fn new(server_id: usize) -> Self {
        let ip_port = RAFT_SERVERS[server_id].1;
        println!("Raft server listening on {}", ip_port);
        RaftNet {
            listener: TcpListener::bind(ip_port).unwrap(),
        }
    }

    pub fn send(&self, dest: usize, msg: &str) -> std::io::Result<()> {
        let ip_port = RAFT_SERVERS[usize::from(dest)].1;
        let mut stream = TcpStream::connect(ip_port)?;
        stream.set_nodelay(true)?;
        stream.write_all(msg.as_bytes())?;
        Ok(())
    }

    pub fn receive(&self) -> std::io::Result<String> {
        let (mut stream, addr) = self.listener.accept()?;
        stream.set_nodelay(true)?;
        println!("Received message from {}", addr);
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf[..n]).to_string())
    }
}
