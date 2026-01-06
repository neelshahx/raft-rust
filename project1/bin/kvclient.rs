use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.set_nodelay(true)?;

    loop {
        print!("KV>");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        stream.write_all(input.as_bytes())?;

        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).unwrap();
        if n > 0 {
            println!("{}", String::from_utf8_lossy(&buf[..n]));
        } else {
            println!("No response.")
        }
    }
}
