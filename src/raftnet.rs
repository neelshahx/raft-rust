use crate::log;
use crate::shared::{InternalMessage, RaftNetMessage, RAFT_SERVERS};
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

    pub fn listen(&self, tx: Sender<InternalMessage>) {
        let ip_port = RAFT_SERVERS[self.server_id].1;
        log!("Raft server listening on {}", ip_port);
        let listener = TcpListener::bind(ip_port).expect("bind failed");

        for stream in listener.incoming() {
            let stream = stream.expect("connection failed");
            let _ = stream.set_nodelay(true);
            let tx = tx.clone();
            std::thread::spawn(move || Self::forward(stream, tx));
        }
    }

    pub fn send(&self, server_id: usize, message: &RaftNetMessage) {
        let mut streams = self.streams.lock().unwrap();
        let payload = serde_json::to_string(&message).unwrap();

        let result = if let Some(stream) = streams.get_mut(&server_id) {
            stream.write_all(payload.as_bytes())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "no connection",
            ))
        };

        if result.is_err() {
            streams.remove(&server_id);
            let ip_port = RAFT_SERVERS[server_id].1;
            let retry_result = TcpStream::connect(ip_port).and_then(|mut stream| {
                stream.set_nodelay(true)?;
                stream.write_all(payload.as_bytes())?;
                streams.insert(server_id, stream);
                Ok(())
            });

            if let Err(e) = retry_result {
                eprintln!("Failed to send to server {}: {}", server_id, e);
            }
        }
    }

    fn forward(mut stream: TcpStream, tx: Sender<InternalMessage>) {
        loop {
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            if n > 0 {
                let message_str = std::str::from_utf8(&buf[..n]).unwrap();
                let message: RaftNetMessage = serde_json::from_str(message_str).unwrap();
                let _ = tx.send(InternalMessage::IncomingRaftMessage(message));
            }
        }
    }
}
