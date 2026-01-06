use project3::config::APP_SERVERS;
use project3::kvstore::KVStore;
use socket2::{Domain, SockRef, Socket, TcpKeepalive, Type};
use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn update_store(input: String, store: &mut KVStore) -> std::io::Result<String> {
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

fn handle_client(stream: &mut TcpStream, data: Arc<Mutex<KVStore>>) -> std::io::Result<String> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let n: u8 = env::args()
        .nth(1)
        .expect("Missing argument server number [1-5]")
        .parse()
        .unwrap();

    if n == 0 || n > 5 {
        return Err("n must be between 0 and 4".to_string().into());
    }

    let data = Arc::new(Mutex::new(KVStore::new()));

    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    socket.set_tcp_nodelay(true)?;

    let ip_port = APP_SERVERS[usize::from(n)].1;
    println!("Listening on {}", ip_port);
    let addr: SocketAddr = ip_port.parse().unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_existing_key() {
        let mut store = KVStore::new();
        store.set("foo", "bar");
        let result = update_store("get foo".to_string(), &mut store).unwrap();
        assert_eq!(result, "bar");
    }

    #[test]
    fn test_get_nonexistent_key() {
        let mut store = KVStore::new();
        let result = update_store("get missing".to_string(), &mut store).unwrap();
        assert_eq!(result, "Not found");
    }

    #[test]
    fn test_set_new_key() {
        let mut store = KVStore::new();
        let result = update_store("set mykey myvalue".to_string(), &mut store).unwrap();
        assert_eq!(result, "Set");
        assert_eq!(store.get("mykey"), Some("myvalue".to_string()));
    }

    #[test]
    fn test_set_existing_key() {
        let mut store = KVStore::new();
        store.set("key", "old");
        let result = update_store("set key new".to_string(), &mut store).unwrap();
        assert_eq!(result, "Old value: old");
        assert_eq!(store.get("key"), Some("new".to_string()));
    }

    #[test]
    fn test_delete_existing_key() {
        let mut store = KVStore::new();
        store.set("foo", "bar");
        let result = update_store("delete foo".to_string(), &mut store).unwrap();
        assert_eq!(result, "Deleted value: bar");
        assert_eq!(store.get("foo"), None);
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let mut store = KVStore::new();
        let result = update_store("delete missing".to_string(), &mut store).unwrap();
        assert_eq!(result, "No such key");
    }

    #[test]
    fn test_incr_existing_integer() {
        let mut store = KVStore::new();
        store.set("counter", "5");
        let result = update_store("incr counter".to_string(), &mut store).unwrap();
        assert_eq!(result, "Incr 6");
        assert_eq!(store.get("counter"), Some("6".to_string()));
    }

    #[test]
    fn test_incr_negative_integer() {
        let mut store = KVStore::new();
        store.set("counter", "-10");
        let result = update_store("incr counter".to_string(), &mut store).unwrap();
        assert_eq!(result, "Incr -9");
        assert_eq!(store.get("counter"), Some("-9".to_string()));
    }

    #[test]
    fn test_incr_zero() {
        let mut store = KVStore::new();
        store.set("counter", "0");
        let result = update_store("incr counter".to_string(), &mut store).unwrap();
        assert_eq!(result, "Incr 1");
        assert_eq!(store.get("counter"), Some("1".to_string()));
    }

    #[test]
    fn test_incr_noninteger_value() {
        let mut store = KVStore::new();
        store.set("key", "notanumber");
        let result = update_store("incr key".to_string(), &mut store).unwrap();
        assert_eq!(result, "Invalid: can only incr integral values");
    }

    #[test]
    fn test_incr_nonexistent_key() {
        let mut store = KVStore::new();
        let result = update_store("incr missing".to_string(), &mut store).unwrap();
        assert_eq!(result, "Invalid: no such key");
    }

    #[test]
    fn test_invalid_command() {
        let mut store = KVStore::new();
        let result = update_store("invalid".to_string(), &mut store).unwrap();
        assert_eq!(result, "Invalid command");
    }

    #[test]
    fn test_invalid_get_no_key() {
        let mut store = KVStore::new();
        let result = update_store("get".to_string(), &mut store).unwrap();
        assert_eq!(result, "Invalid command");
    }

    #[test]
    fn test_invalid_set_missing_value() {
        let mut store = KVStore::new();
        let result = update_store("set key".to_string(), &mut store).unwrap();
        assert_eq!(result, "Invalid command");
    }

    #[test]
    fn test_invalid_delete_no_key() {
        let mut store = KVStore::new();
        let result = update_store("delete".to_string(), &mut store).unwrap();
        assert_eq!(result, "Invalid command");
    }

    #[test]
    fn test_extra_whitespace() {
        let mut store = KVStore::new();
        let result = update_store("  set   key   value  ".to_string(), &mut store).unwrap();
        assert_eq!(result, "Set");
        assert_eq!(store.get("key"), Some("value".to_string()));
    }

    #[test]
    fn test_multiple_incr() {
        let mut store = KVStore::new();
        store.set("count", "0");
        update_store("incr count".to_string(), &mut store).unwrap();
        update_store("incr count".to_string(), &mut store).unwrap();
        let result = update_store("incr count".to_string(), &mut store).unwrap();
        assert_eq!(result, "Incr 3");
        assert_eq!(store.get("count"), Some("3".to_string()));
    }
}
