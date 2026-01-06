use project1::KVStore;
use socket2::{Domain, SockRef, Socket, TcpKeepalive, Type};
use std::io::{Read, Result};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn update_store(input: String, data: &Arc<Mutex<KVStore>>) -> Result<String> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let response = match parts.as_slice() {
        ["get", key] if key.is_ascii() => {
            let store = data.lock().unwrap();
            store.get(*key).unwrap_or("Not found".to_string())
        }
        ["set", key, val] if key.is_ascii() && val.is_ascii() => {
            let mut store = data.lock().unwrap();
            match store.set(*key, *val) {
                Some(old) => format!("Old value: {}", old),
                None => "Set".to_string(),
            }
        }
        ["delete", key] if key.is_ascii() => {
            let mut store = data.lock().unwrap();
            match store.delete(*key) {
                Some(old) => format!("Deleted value: {}", old),
                None => "No such key".to_string(),
            }
        }
        // TODO increment
        _ => "Invalid input".to_string(),
    };
    Ok(response)
}

fn handle_client(stream: &mut TcpStream, data: Arc<Mutex<KVStore>>) -> Result<String> {
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf)?;
    if n > 0 {
        let input = String::from_utf8_lossy(&buf[..n]);
        return update_store(input.to_string(), &data);
    }
    Ok("Invalid input".to_string())
}

fn main() -> Result<()> {
    let data = Arc::new(Mutex::new(KVStore::new()));

    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    socket.set_tcp_nodelay(true)?;

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    socket.bind(&addr.into())?;
    socket.listen(8)?;

    let listener: TcpListener = socket.into();
    for stream in listener.incoming() {
        let mut stream = stream?;
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;

        let data = Arc::clone(&data);
        std::thread::spawn(move || {
            if let Err(e) = handle_client(&mut stream, data) {
                eprintln!("Error: {}", e);
            }
        });
    }

    Ok(())
}
