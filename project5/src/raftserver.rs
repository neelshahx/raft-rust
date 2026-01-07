use crate::clienthandler::ClientHandler;
use crate::raftconsensus::RaftConsensus;
use crate::raftconsole::RaftConsole;
use crate::raftnet::RaftNet;
use crate::shared::SenderType;
use std::sync::mpsc;

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
        let current_term = 0;
        RaftServer {
            server_id,
            is_leader,
            client_handler: ClientHandler::new(server_id),
            console: RaftConsole::new(server_id),
            net: RaftNet::new(server_id),
            consensus: RaftConsensus::new(server_id, current_term, num_servers),
        }
    }

    pub fn launch(mut self) {
        let (tx, rx) = mpsc::channel::<(SenderType, String)>();

        let tx1 = tx.clone();
        let console = self.console;
        std::thread::spawn(move || console.start(tx1));

        let tx2 = tx.clone();
        let client_handler = self.client_handler;
        std::thread::spawn(move || client_handler.listen(tx2));

        let mut consensus = self.consensus;
        for (sender_type, command) in rx {
            match sender_type {
                SenderType::CLIENT => {
                    if self.is_leader {
                        println!("Client command sent to leader: {}", command);
                        consensus.new_client_command(command);
                        consensus.update_followers();
                        // TODO: send message over raftnet
                        // for message in &consensus.outbound {
                        //
                        // }
                    }
                }
                SenderType::CONSOLE => {
                    println!("Console command: {}", command);
                    if command == "show log" {
                        consensus.print_log();
                    }
                    match command.split_once(" ") {
                        Some(("append_entries", command)) => {
                            consensus.handle_append_entries(command.to_string())
                        }
                        _ => {}
                    }
                }
                SenderType::RAFTNET => {
                    // TODO: call handle follower response
                    // TODO: call handle append entries
                }
            }
        }
    }
}
