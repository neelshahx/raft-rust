use project1::KVStore;
use socket2::{Domain, SockRef, Socket, TcpKeepalive, Type};
use std::io::{Read, Result, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn update_store(input: String, store: &mut KVStore) -> Result<String> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let response = match parts.as_slice() {
        ["get", key] if key.is_ascii() => store.get(*key).unwrap_or("Not found".to_string()),
        ["set", key, val] if key.is_ascii() && val.is_ascii() => match store.set(*key, *val) {
            Some(old) => format!("Old value: {}", old),
            None => "Set".to_string(),
        },
        ["delete", key] if key.is_ascii() => match store.delete(*key) {
            Some(old) => format!("Deleted value: {}", old),
            None => "No such key".to_string(),
        },
        ["incr", key] if key.is_ascii() => match store.get(*key) {
            Some(val) => match val.parse::<i32>() {
                Ok(n) => {
                    let new_val = n + 1;
                    store.set(*key, &new_val.to_string());
                    format!("Incr {}", new_val)
                }
                Err(_) => "Invalid: can only incr integral values".to_string(),
            },
            None => "Invalid: no such key".to_string(),
        },
        _ => "Invalid command".to_string(),
    };
    Ok(response)
}

fn handle_client(stream: &mut TcpStream, data: Arc<Mutex<KVStore>>) -> Result<String> {
    loop {
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        let input = String::from_utf8_lossy(&buf[..n]);
        let response = {
            let mut store = data.lock().unwrap();
            update_store(input.to_string(), &mut store)?
        };
        stream.write_all(response.as_bytes())?;
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
    socket.listen(128)?;

    let listener: TcpListener = socket.into();
    for stream in listener.incoming() {
        let mut stream = stream?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;
        let keepalive = TcpKeepalive::new().with_time(Duration::from_secs(60));
        SockRef::from(&stream).set_tcp_keepalive(&keepalive)?;

        let data = Arc::clone(&data);
        std::thread::spawn(move || {
            if let Err(e) = handle_client(&mut stream, data) {
                eprintln!("Error: {}", e);
            }
        });
    }

    Ok(())
}
