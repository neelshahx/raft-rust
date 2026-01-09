use raft::raftserver::RaftServer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (my_server_id, num_servers) = parse_args()?;
    let _ = RaftServer::new(my_server_id, num_servers).launch();
    Ok(())
}

fn parse_args() -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let id: usize = std::env::args()
        .nth(1)
        .ok_or("Missing argument server number [1-5]")?
        .parse()?;

    if id == 0 || id > 5 {
        return Err("Server ID must be between 1 and 5".into());
    }

    let num_servers = std::env::args()
        .nth(2)
        .ok_or("Missing argument number of servers")?
        .parse()?;

    Ok((id, num_servers))
}
