use socket2::{SockRef, TcpKeepalive};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
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
