use socket2::{Domain, SockRef, Socket, TcpKeepalive, Type};
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub const APP_SERVERS: [(u8, &str); 6] = [
    (0, "0.0.0.0:0"),
    (1, "127.0.0.1:21000"),
    (2, "127.0.0.1:22000"),
    (3, "127.0.0.1:23000"),
    (4, "127.0.0.1:24000"),
    (5, "127.0.0.1:25000"),
];

pub const RAFT_SERVERS: [(u8, &str); 6] = [
    (0, "0.0.0.0:0"),
    (1, "127.0.0.1:11000"),
    (2, "127.0.0.1:12000"),
    (3, "127.0.0.1:13000"),
    (4, "127.0.0.1:14000"),
    (5, "127.0.0.1:15000"),
];

pub enum Source {
    CONSOLE,
    CLIENT,
    INTERNAL,
}

#[derive(Debug, PartialEq)]
pub enum Role {
    LEADER,
    FOLLOWER,
}

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

pub fn asc_sort_median(match_index: Vec<usize>) -> usize {
    let mut match_index = match_index[1..].to_vec();
    match_index.sort();
    if match_index.is_empty() {
        0
    } else if match_index.len() % 2 == 0 {
        match_index[match_index.len() / 2 - 1]
    } else {
        match_index[match_index.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_match_index() {
        let match_index = Vec::from([0]);
        assert_eq!(asc_sort_median(match_index), 0);
    }

    #[test]
    fn test_match_index_len1() {
        let match_index = Vec::from([0, 1]);
        assert_eq!(asc_sort_median(match_index), 1);
    }

    #[test]
    fn test_match_index_len2() {
        let match_index = Vec::from([0, 2, 1]);
        assert_eq!(asc_sort_median(match_index), 1);
    }

    #[test]
    fn test_match_index_len3() {
        let match_index = Vec::from([0, 3, 2, 1]);
        assert_eq!(asc_sort_median(match_index), 2);
    }
}
