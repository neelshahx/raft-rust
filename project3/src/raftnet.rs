use crate::config::APP_SERVERS;
use crate::network::{configure_stream, make_streaming_socket};
use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct RaftNet {
    server_num: u8,
    listener: TcpListener,
    connections: HashMap<u8, TcpStream>,
}

impl RaftNet {
    pub fn new(server_num: u8) -> Result<Self, Box<dyn Error>> {
        let listener = Self::bind(server_num)?;
        Ok(RaftNet {
            server_num,
            listener,
            connections: HashMap::new(),
        })
    }
    fn bind(server_num: u8) -> Result<TcpListener, Box<dyn Error>> {
        let ip_port = APP_SERVERS[usize::from(server_num)].1;
        let socket = make_streaming_socket(ip_port)?;
        let listener: TcpListener = socket.into();
        Ok(listener)
    }

    pub fn send(&mut self, dest: u8, msg: &str) -> std::io::Result<()> {
        let stream = self.connections.entry(dest).or_insert_with(|| {
            let ip_port = APP_SERVERS[usize::from(dest)].1;
            let stream = TcpStream::connect(ip_port).expect("Failed to connect");
            configure_stream(&stream).expect("Failed to configure");
            stream
        });
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
