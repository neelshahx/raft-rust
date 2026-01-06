use project3::SERVERS;
use socket2::{SockRef, TcpKeepalive};
use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let n: u8 = env::args()
        .nth(1)
        .expect("Missing argument server number [0-4]")
        .parse()
        .unwrap();

    if n > 4 {
        return Err("n must be between 0 and 4".to_string().into());
    }

    let ip_port = SERVERS[usize::from(n)].1;
    println!("Connecting to {}", ip_port);
    let mut stream = TcpStream::connect(ip_port)?;
    stream.set_nodelay(true)?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    let keepalive = TcpKeepalive::new().with_time(Duration::from_secs(60));
    SockRef::from(&stream).set_tcp_keepalive(&keepalive)?;

    loop {
        print!("KV>");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        stream.write_all(input.as_bytes())?;

        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf)?;
        if n > 0 {
            println!("{}", String::from_utf8_lossy(&buf[..n]));
        } else {
            println!("No response.")
        }
    }
}
