use std::io::Write;
use std::net::TcpStream;
use std::thread;

fn handle_get(key: &str, stream: &mut TcpStream) {
    stream.write_all(format!("get {}", key).as_bytes());
}
fn handle_set(key: &str, value: &str, stream: &mut TcpStream) {
    stream.write_all(format!("set {} {}", key, value).as_bytes());
}
fn handle_delete(key: &str, stream: &mut TcpStream) {
    stream.write_all(format!("delete {}", key).as_bytes());
}

fn main() -> std::io::Result<()> {
    // TODO: handle server not up, and hard-coded server name
    let stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.set_nodelay(true)?;

    loop {
        let mut input = String::new();
        print!("KV>");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;

        let mut stream_copy = stream.try_clone()?;

        thread::spawn(move || {
            let input = input.trim();
            let v: Vec<&str> = input.split(' ').collect();
            match v.as_slice() {
                ["get", key] if key.is_ascii() => handle_get(key, &mut stream_copy),
                ["set", key, val] if key.is_ascii() && val.is_ascii() => {
                    handle_set(key, val, &mut stream_copy)
                }
                ["delete", key] if key.is_ascii() => handle_delete(key, &mut stream_copy),
                _ => {
                    println!("Invalid input. Valid inputs is: get <k>, set <k> <v>, delete <k>");
                }
            }
            println!("You typed {}", input); // TODO: replace with server response
        });
    }
}
