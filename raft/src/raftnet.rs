use crate::shared::{Source, RAFT_SERVERS};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub struct RaftNet {
    server_id: usize,
    streams: Arc<Mutex<HashMap<usize, TcpStream>>>,
}

impl RaftNet {
    pub fn new(server_id: usize) -> Self {
        RaftNet {
            server_id,
            streams: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn listen(&self, tx: Sender<(Source, String)>) {
        let ip_port = RAFT_SERVERS[self.server_id].1;
        println!("Raft server listening on {}", ip_port);
        let listener = TcpListener::bind(ip_port).expect("bind failed");

        for stream in listener.incoming() {
            let stream = stream.expect("connection failed");
            let _ = stream.set_nodelay(true);
            let tx = tx.clone();
            std::thread::spawn(move || Self::handle_raftserver(stream, tx));
        }
    }

    pub fn send(&self, dest: usize, msg: &str) -> std::io::Result<()> {
        let mut streams = self.streams.lock().unwrap();
        if let Some(stream) = streams.get_mut(&dest) {
            stream.write_all(msg.as_bytes())?;
        } else {
            let ip_port = RAFT_SERVERS[dest].1;
            let mut stream = TcpStream::connect(ip_port)?;
            stream.set_nodelay(true)?;
            stream.write_all(msg.as_bytes())?;
            streams.insert(dest, stream);
        }
        Ok(())
    }

    fn handle_raftserver(mut stream: TcpStream, tx: Sender<(Source, String)>) {
        loop {
            let mut buf = [0u8; 1024];
            let n = match stream.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => n,
            };
            let message = String::from_utf8_lossy(&buf[..n]).to_string();
            let _ = tx.send((Source::INTERNAL, message));
        }
    }
}