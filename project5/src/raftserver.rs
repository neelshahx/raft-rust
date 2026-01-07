use crate::clienthandler::ClientHandler;
use crate::raftconsole::RaftConsole;
use crate::raftlog::RaftLog;
use crate::raftnet::RaftNet;
use std::sync::mpsc;

pub struct RaftServer {
    server_id: usize,
    is_leader: bool,
    client_handler: ClientHandler,
    console: RaftConsole,
    net: RaftNet,
    log: RaftLog,
}

impl RaftServer {
    pub fn new(server_id: usize, is_leader: bool) -> Self {
        RaftServer {
            server_id,
            is_leader,
            client_handler: ClientHandler::new(server_id),
            console: RaftConsole::new(server_id),
            net: RaftNet::new(server_id),
            log: RaftLog::new(),
        }
    }

    pub fn launch(self) {
        let (tx, rx) = mpsc::channel::<String>();

        let tx1 = tx.clone();
        let client_handler = self.client_handler;
        std::thread::spawn(move || client_handler.listen(tx1));

        let tx2 = tx.clone();
        let console = self.console;
        std::thread::spawn(move || console.start(tx2));

        // need client handler

        // raft internal listener

        for recv in rx {
            println!("{}", recv);

            // if recv == show log

            // handle add (L) or update (F)
            // add to log
        }

    }
}
