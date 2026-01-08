use crate::clienthandler::ClientHandler;
use crate::raftconsensus::RaftConsensus;
use crate::raftconsole::RaftConsole;
use crate::raftnet::RaftNet;
use crate::shared::{Role, Source};
use std::sync::mpsc;

pub struct RaftServer {
    console: RaftConsole,
    client_handler: ClientHandler,
    net: RaftNet,
    consensus: RaftConsensus,
}

impl RaftServer {
    pub fn new(server_id: usize, num_servers: usize, is_leader: bool) -> Self {
        let role = if is_leader {
            Role::LEADER
        } else {
            Role::FOLLOWER
        };
        RaftServer {
            client_handler: ClientHandler::new(server_id),
            console: RaftConsole::new(server_id),
            net: RaftNet::new(server_id),
            consensus: RaftConsensus::new(server_id, num_servers, role),
        }
    }

    pub fn launch(mut self) {
        let (tx, rx) = mpsc::channel::<(Source, String)>();

        let tx1 = tx.clone();
        let console = self.console;
        std::thread::spawn(move || console.start(tx1));

        let tx2 = tx.clone();
        let client_handler = self.client_handler;
        std::thread::spawn(move || client_handler.listen(tx2));

        let mut consensus = self.consensus;
        for (sender_type, command) in rx {
            match sender_type {
                Source::CLIENT => {
                    match consensus.role {
                        Role::LEADER => {
                            consensus.new_client_command(command);
                            consensus.update_followers();
                            // TODO: send message over raftnet
                            // TODO: clear outbound
                        }
                        Role::FOLLOWER => {
                            // consensus.handle_append_entries(command);
                            // consensus.respond_to_leader();
                        }
                    }
                }
                Source::CONSOLE => match command.split_once(' ') {
                    Some(("log", "")) => consensus.print_log(),
                    Some(("state", "")) => {
                        // self.consensus.print(); // outbound
                        // self.client_handler.print(); // kvstore
                    }
                    Some(("command", cmd)) => {
                        consensus.handle_append_entries(cmd);
                    }
                    Some(("update", "")) => {}
                    Some(("leader", "")) => {
                        consensus.role = Role::LEADER;
                    }
                    Some(("follower", "")) => {
                        consensus.role = Role::FOLLOWER;
                    }
                    _ => {
                        println!("Received unknown console command: {}", command);
                    }
                },
                Source::INTERNAL => {
                    // TODO: call handle follower response
                    // TODO: call handle append entries
                }
            }
        }
    }
}
