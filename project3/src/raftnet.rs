use crate::config::APP_SERVERS;
use socket2::{Domain, SockRef, Socket, TcpKeepalive, Type};
use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::time::Duration;

struct RaftNet {
    server_num: u8,
    listener: TcpListener,
    connections: HashMap<u8, TcpStream>,
}

impl RaftNet {
    pub fn new(server_num: u8) -> Result<Self, Box<dyn Error>> {
        let listener = Self::connect(server_num)?;
        Ok(RaftNet {
            server_num,
            listener,
            connections: HashMap::new(),
        })
    }
    fn connect(server_num: u8) -> Result<TcpListener, Box<dyn Error>> {
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
        socket.set_reuse_address(true)?;
        socket.set_tcp_nodelay(true)?;
        let ip_port = APP_SERVERS[usize::from(server_num)].1;
        let addr: SocketAddr = ip_port.parse().unwrap();
        socket.bind(&addr.into())?;
        socket.listen(128)?;
        let listener: TcpListener = socket.into();
        Ok(listener)
    }

    pub fn send(&mut self, dest: u8, msg: &str) -> std::io::Result<()> {
        let stream = self.connections.entry(dest).or_insert_with(|| {
            let ip_port = APP_SERVERS[usize::from(dest)].1;
            let mut stream = TcpStream::connect(ip_port).expect("Failed to connect");
            stream.set_nodelay(true).ok();
            stream.set_write_timeout(Some(Duration::from_secs(5))).ok();
            let keepalive = TcpKeepalive::new().with_time(Duration::from_secs(60));
            SockRef::from(&stream).set_tcp_keepalive(&keepalive).ok();
            stream
        });
        stream.write_all(msg.as_bytes())?;
        Ok(())
    }
    pub fn receive(&self) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            let mut stream = stream?;
            stream.set_write_timeout(Some(Duration::from_secs(5)))?;
            let keepalive = TcpKeepalive::new().with_time(Duration::from_secs(60));
            SockRef::from(&stream).set_tcp_keepalive(&keepalive)?;
            loop {
                let mut buf = [0u8; 1024];
                let n = stream.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                println!("{}", String::from_utf8_lossy(&buf[..n]));
            }
        }
        Ok(())
    }
}
