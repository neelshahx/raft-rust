use crate::shared::{Source, APP_SERVERS};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;

pub struct ClientHandler {
    server_id: usize,
}

fn handle_client(mut stream: TcpStream, addr: String, tx: Sender<(Source, String)>) {
    loop {
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        let client_cmd = String::from_utf8_lossy(&buf[..n]);
        tx.send((
            Source::CLIENT,
            format!("{} {}", addr, client_cmd.to_string()),
        ))
        .unwrap();
    }
}

fn send_response(addr: String, msg: String) {
    let mut stream = TcpStream::connect(addr).unwrap();
    let _ = stream.set_nodelay(true);
    stream.write_all(msg.as_bytes());
}

impl ClientHandler {
    pub fn new(server_id: usize) -> Self {
        ClientHandler { server_id }
    }

    // tx: allows message passing back to RaftServer, one channel per client
    pub fn listen(&self, tx: Sender<(Source, String)>) {
        let ip_port = APP_SERVERS[self.server_id].1;
        println!("App server listening on {}", ip_port);
        let listener = TcpListener::bind(ip_port).unwrap();

        loop {
            let (stream, addr) = listener.accept().expect("connection failed");
            stream.set_nodelay(true).unwrap();
            let tx = tx.clone();
            std::thread::spawn(move || handle_client(stream, addr.to_string(), tx));
        }
    }
}
