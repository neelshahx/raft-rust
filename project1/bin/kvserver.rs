use socket2::{Domain, Socket, Type};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use project1::KVStore;

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            return Ok(());
        }
        todo!("update key value store");
        todo!("write response");
    }
}

fn main() -> std::io::Result<()> {
    let data = KVStore::new(); //guard with mutex p367-374

    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    socket.set_tcp_nodelay(true)?;

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    socket.bind(&addr.into())?;
    socket.listen(8);

    let listener: TcpListener = socket.into();
    for stream in listener.incoming() {
        let mut stream = stream?;
        std::thread::spawn(move || {
            if let Err(e) = handle_client(&mut stream) {
                eprintln!("Error: {}", e);
            }
        });
    }

    Ok(())
}
