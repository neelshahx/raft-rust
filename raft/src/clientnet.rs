use crate::shared::{InternalMessage, APP_SERVERS};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub struct ClientNet {
    server_id: usize,
    streams: Arc<Mutex<HashMap<String, TcpStream>>>,
}

impl ClientNet {
    pub fn new(server_id: usize) -> Self {
        ClientNet {
            server_id,
            streams: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn listen(&self, tx: Sender<InternalMessage>) {
        let ip_port = APP_SERVERS[self.server_id].1;
        println!("App server listening on {}", ip_port);
        let listener = TcpListener::bind(ip_port).expect("bind failed");

        for stream in listener.incoming() {
            let stream = stream.expect("connection failed");
            let _ = stream.set_nodelay(true);
            let tx = tx.clone();
            let streams = self.streams.clone();
            let addr = stream
                .peer_addr()
                .ok()
                .map(|a| a.to_string())
                .unwrap_or_default();
            std::thread::spawn(move || Self::handle_client(stream, addr, tx, streams));
        }
    }

    pub fn send(&self, addr: &str, msg: &str) {
        if let Some(stream) = self.streams.lock().unwrap().get_mut(addr) {
            let _ = stream.write_all(msg.as_bytes());
        }
    }

    fn handle_client(
        mut stream: TcpStream,
        addr: String,
        tx: Sender<InternalMessage>,
        streams: Arc<Mutex<HashMap<String, TcpStream>>>,
    ) {
        streams
            .lock()
            .unwrap()
            .insert(addr.clone(), stream.try_clone().unwrap());
        loop {
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            if n == 0 {
                break;
            }
            let command = String::from_utf8_lossy(&buf[..n]);
            let _ = tx.send(InternalMessage::ClientCommand {
                addr: addr.clone(),
                payload: command.to_string(),
            });
        }
        streams.lock().unwrap().remove(&addr);
    }
}
