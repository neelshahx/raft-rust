use crate::clienthandler::ClientHandler;
use crate::raftconsole::RaftConsole;
use crate::raftlog::RaftLog;
use crate::raftnet::RaftNet;
use std::sync::mpsc;
use crate::raftconsensus::RaftConsensus;

pub struct RaftServer {
    server_id: usize,
    is_leader: bool,
    console: RaftConsole,
    client_handler: ClientHandler,
    net: RaftNet,
    consensus: RaftConsensus,
}

impl RaftServer {
    pub fn new(server_id: usize, num_servers: usize, is_leader: bool) -> Self {
        RaftServer {
            server_id,
            is_leader,
            client_handler: ClientHandler::new(server_id),
            console: RaftConsole::new(server_id),
            net: RaftNet::new(server_id),
            consensus: RaftConsensus::new(server_id, num_servers),
        }
    }

    pub fn launch(self) {
        let (tx, rx) = mpsc::channel::<String>();

        let tx1 = tx.clone();
        let console = self.console;
        std::thread::spawn(move || console.start(tx1));

        let tx2 = tx.clone();
        let client_handler = self.client_handler;
        std::thread::spawn(move || client_handler.listen(tx2));


        for recv in rx {
            println!("{}", recv);


        }
    }
}
