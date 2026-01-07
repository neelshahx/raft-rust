use std::env;
use std::error::Error;
use project5::raftserver::RaftServer;

fn main() -> Result<(), Box<dyn Error>> {
    let my_server_id = parse_server_id()?;
    let raft_server = RaftServer::new(my_server_id, my_server_id == 1).launch();
    Ok(())
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
