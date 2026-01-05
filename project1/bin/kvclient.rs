use std::ascii::AsciiExt;
use std::io::{Read, Write};
use std::net::TcpStream;

fn handle_get(key: &str, stream: &mut TcpStream) -> String {
    stream.write_all(format!("get {}", key).as_bytes());
    let mut stream2 = stream.try_clone().unwrap();
    wait_for_response(&mut stream2)
}
fn handle_set(key: &str, value: &str, stream: &mut TcpStream) -> String {
    stream.write_all(format!("set {} {}", key, value).as_bytes());
    let mut stream2 = stream.try_clone().unwrap();
    wait_for_response(&mut stream2)
}
fn handle_delete(key: &str, stream: &mut TcpStream) -> String {
    stream.write_all(format!("delete {}", key).as_bytes());
    let mut stream2 = stream.try_clone().unwrap();
    wait_for_response(&mut stream2)
}

fn handle_increment(key: &str, stream: &mut TcpStream) -> String {
    "Ok".to_string()
}

fn wait_for_response(stream: &mut TcpStream) -> String {
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).unwrap();
    if n > 0 && (String::from_utf8_lossy(&buf[..n]) == "Ok") {
        return "Ok".to_string();
    }
    "NotOk".to_string()
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.set_nodelay(true)?;

    loop {
        let mut input = String::new();
        print!("KV>");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim();
        let v: Vec<&str> = input.split(' ').collect();
        let response = match v.as_slice() {
            ["get", key] if key.is_ascii() => handle_get(key, &mut stream),
            ["set", key, val] if key.is_ascii() && val.is_ascii() => {
                handle_set(key, val, &mut stream)
            }
            ["delete", key] if key.is_ascii() => handle_delete(key, &mut stream),
            ["incr", key] if key.is_ascii() => handle_increment(key, &mut stream),
            _ => {
                println!("Invalid input. Try get <k>, set <k> <v>, delete <k>, incr <k> [0-9]+");
                "NotOk".to_string()
            }
        };
        println!("{}", response);
    }
}
