use socket2::{Domain, SockRef, Socket, TcpKeepalive, Type};
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub fn configure_stream(stream: &TcpStream) -> std::io::Result<()> {
    stream.set_nodelay(true)?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    let keepalive = TcpKeepalive::new().with_time(Duration::from_secs(60));
    SockRef::from(&stream).set_tcp_keepalive(&keepalive)?;
    Ok(())
}

pub fn make_streaming_socket(ip_port: &str) -> Result<Socket, Box<dyn Error>> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    socket.set_tcp_nodelay(true)?;
    let addr: SocketAddr = ip_port.parse()?;
    socket.bind(&addr.into())?;
    socket.listen(128)?;
    Ok(socket)
}
