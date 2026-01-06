use project3::config::APP_SERVERS;
use project3::network::configure_stream;
use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn Error>> {
    let n: u8 = env::args()
        .nth(1)
        .expect("Missing argument server number [1-5]")
        .parse()?;

    if n == 0 || n > 5 {
        return Err("n must be between 1 and 5".to_string().into());
    }

    let ip_port = APP_SERVERS[usize::from(n)].1;
    println!("Connecting to {}", ip_port);
    let mut stream = TcpStream::connect(ip_port)?;
    configure_stream(&stream)?;

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
