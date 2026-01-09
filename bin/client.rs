use raft::log;
use raft::shared::APP_SERVERS;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_id = parse_server_id()?;
    let ip_port = APP_SERVERS[usize::from(server_id)].1;

    loop {
        if let Err(e) = run(ip_port) {
            eprintln!("Disconnected: {}. Retrying in 2s...", e);
            std::thread::sleep(Duration::from_secs(2));
        }
    }
}

fn run(ip_port: &str) -> std::io::Result<()> {
    log!("Client connecting to app server on {}", ip_port);
    let mut stream_in = TcpStream::connect(ip_port)?;
    stream_in.set_nodelay(true)?;

    let mut stream_out = stream_in.try_clone()?;
    std::thread::spawn(move || {
        loop {
            print!("KV>");
            let _ = std::io::stdout().flush();
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
            if stream_out.write_all(input.as_bytes()).is_err() {
                break;
            }
        }
    });

    loop {
        let mut buf = [0u8; 1024];
        match stream_in.read(&mut buf) {
            Ok(0) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::ConnectionReset,
                    "server closed connection",
                ));
            }
            Ok(n) => {
                let reply = String::from_utf8_lossy(&buf[..n]);
                println!("{}", reply);
            }
            Err(e) => return Err(e),
        }
    }
}

fn parse_server_id() -> Result<usize, Box<dyn std::error::Error>> {
    let id: usize = std::env::args()
        .nth(1)
        .ok_or("Missing argument server number [1-5]")?
        .parse()?;

    if id == 0 || id > 5 {
        return Err("Server ID must be between 1 and 5".into());
    }

    Ok(id)
}
