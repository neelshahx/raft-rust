use crate::raftnet::RaftNet;
use std::io::Write;
use std::sync::Arc;

pub struct RaftServer {
    server_id: usize,
    raft_net: RaftNet,
}

impl RaftServer {
    pub fn new(server_id: usize) -> Self {
        RaftServer {
            server_id,
            raft_net: RaftNet::new(server_id),
        }
    }

    pub fn launch(self) {
        let raft_net = Arc::new(self.raft_net);

        // listener
        let raft_net_listener = Arc::clone(&raft_net);
        let handle = std::thread::spawn(move || {
            loop {
                println!("{}", raft_net_listener.receive().unwrap());
            }
        });

        // console
        std::thread::spawn(move || {
            loop {
                print!("RC {}>", self.server_id);
                std::io::stdout().flush().ok();

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).ok();
                raft_net.send(1, input.trim()).ok();
            }
        });
        handle.join().unwrap();
    }
}
