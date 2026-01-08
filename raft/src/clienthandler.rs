use crate::shared::{configure_stream, make_streaming_socket};
use crate::shared::{Source, APP_SERVERS};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;

pub(crate) struct ClientHandler {
    server_id: usize,
}

fn handle_client(mut stream: TcpStream, tx: Sender<(Source, String)>) {
    loop {
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        let client_cmd = String::from_utf8_lossy(&buf[..n]);
        tx.send((Source::CLIENT, client_cmd.to_string())).unwrap();
    }
}

fn send_response() {
    todo!();
}

impl ClientHandler {
    pub(crate) fn new(server_id: usize) -> Self {
        ClientHandler { server_id }
    }

    pub(crate) fn listen(&self, tx: Sender<(Source, String)>) {
        let ip_port = APP_SERVERS[self.server_id].1;
        println!("App server listening on {}", ip_port);
        let socket = make_streaming_socket(ip_port).unwrap();

        let listener: TcpListener = socket.into();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let tx = tx.clone();
            configure_stream(&stream).unwrap();
            std::thread::spawn(move || handle_client(stream, tx));
        }
    }
}
