use crate::clientnet::ClientNet;
use crate::kvstore::KVStore;
use crate::raftconsensus::RaftConsensus;
use crate::raftconsole::RaftConsole;
use crate::raftnet::RaftNet;
use crate::shared::{Role, Source};
use std::sync::mpsc;

pub struct RaftServer {
    client_handler: ClientNet,
    kvstore: KVStore,
    consensus: RaftConsensus,
    net: RaftNet,
    console: RaftConsole,
}

impl RaftServer {
    pub fn new(server_id: usize, num_servers: usize, is_leader: bool) -> Self {
        let role = if is_leader {
            Role::LEADER
        } else {
            Role::FOLLOWER
        };
        RaftServer {
            client_handler: ClientNet::new(server_id),
            kvstore: KVStore::new(),
            consensus: RaftConsensus::new(server_id, num_servers, role),
            net: RaftNet::new(server_id),
            console: RaftConsole::new(server_id),
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
                            for follower_id in 1..consensus.num_servers + 1 {
                                if follower_id == consensus.server_id {
                                    continue;
                                }
                                consensus.update_follower(follower_id);
                            }
                            // TODO: send message over raftnet
                            // TODO: clear outbound
                        }
                        Role::FOLLOWER => {
                            // consensus.handle_append_entries(command);
                            // consensus.respond_to_leader();
                        }
                    }
                }
                Source::CONSOLE => {
                    if !command.trim().is_empty() {
                        match command.split_once(' ') {
                            Some(("cmd", args)) => {
                                consensus.new_client_command(args.to_string());
                                consensus.update_follower(2);
                            }
                            Some(("req", args)) => {
                                let (leader_id, success) =
                                    consensus.handle_append_entries_request(args);
                                consensus.respond_to_leader(leader_id, success)
                            }
                            Some(("rep", json)) => consensus.handle_append_entries_response(json),
                            None => match command.as_str() {
                                "state" => consensus.print_consensus(),
                                "log" => consensus.print_log(),
                                "leader" => consensus.role = Role::LEADER,
                                "follower" => consensus.role = Role::FOLLOWER,
                                _ => println!("Unknown: {}", command),
                            },
                            _ => println!("Unknown: {}", command),
                        }
                    }
                }
                Source::INTERNAL => {
                    // TODO: call handle follower response
                    // TODO: call handle append entries
                }
            }
        }
    }
}
