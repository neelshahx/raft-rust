use raft::shared::configure_stream;
use raft::shared::APP_SERVERS;
use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn Error>> {
    let server_id = parse_server_id()?;

    let ip_port = APP_SERVERS[usize::from(server_id)].1;
    println!("Connecting to app server {}", ip_port);
    let mut stream = TcpStream::connect(ip_port)?;
    configure_stream(&stream)?;

    let mut stream2 = stream.try_clone()?;
    std::thread::spawn(move || {
        loop {
            print!("KV>");
            let _ = std::io::stdout().flush();
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
            let _ = stream2.write_all(input.as_bytes());
        }
    });

    loop {
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf)?;
        if n > 0 {
            println!("{}", String::from_utf8_lossy(&buf[..n]));
        }
    }
}

fn parse_server_id() -> Result<usize, Box<dyn Error>> {
    let id: usize = env::args()
        .nth(1)
        .ok_or("Missing argument server number [1-5]")?
        .parse()?;

    if id == 0 || id > 5 {
        return Err("Server ID must be between 1 and 5".into());
    }

    Ok(id)
}
