use raft::shared::APP_SERVERS;
use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn Error>> {
    let server_id = parse_server_id()?;
    let ip_port = APP_SERVERS[usize::from(server_id)].1;
    println!("Client connected to app server on {}", ip_port);
    let mut stream_in = TcpStream::connect(ip_port)?;
    stream_in.set_nodelay(true)?;

    let mut stream_out = stream_in.try_clone()?;
    std::thread::spawn(move || {
        loop {
            print!("KV>");
            let _ = std::io::stdout().flush();
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
            let _ = stream_out.write_all(input.as_bytes());
        }
    });

    loop {
        let mut buf = [0u8; 1024];
        let n = stream_in.read(&mut buf)?;
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
