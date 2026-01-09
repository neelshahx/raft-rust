use crate::clientnet::ClientNet;
use crate::kvstore::KVStore;
use crate::raftconsensus::RaftConsensus;
use crate::raftconsole::RaftConsole;
use crate::raftnet::RaftNet;
use crate::shared::{Role, Source};
use std::sync::{mpsc, Arc};

pub struct RaftServer {
    consensus: RaftConsensus,
    kvstore: KVStore,
    client_net: Arc<ClientNet>,
    raft_net: Arc<RaftNet>,
    console: RaftConsole,
}

impl RaftServer {
    pub fn new(server_id: usize, num_servers: usize, is_leader: bool) -> Self {
        let role = if is_leader {
            Role::Leader
        } else {
            Role::Follower
        };
        RaftServer {
            consensus: RaftConsensus::new(server_id, num_servers, role),
            kvstore: KVStore::new(),
            client_net: Arc::new(ClientNet::new(server_id)),
            raft_net: Arc::new(RaftNet::new(server_id)),
            console: RaftConsole::new(server_id),
        }
    }

    pub fn run(self) {
        let (tx, rx) = mpsc::channel::<(Source, String)>();

        let tx1 = tx.clone();
        let console = self.console;
        std::thread::spawn(move || console.start(tx1));

        let tx2 = tx.clone();
        let client_net_listner = self.client_net.clone();
        std::thread::spawn(move || client_net_listner.listen(tx2));

        let tx3 = tx.clone();
        let raft_net_listener = self.raft_net.clone();
        std::thread::spawn(move || raft_net_listener.listen(tx3));

        let client_net = self.client_net.clone();
        let raft_net = self.raft_net.clone();

        let mut consensus = self.consensus;
        for (sender_type, command) in rx {
            match sender_type {
                Source::ClientNet => match consensus.role {
                    Role::Leader => {
                        consensus.new_client_command(command);
                        for follower_id in 1..consensus.num_servers + 1 {
                            if follower_id == consensus.server_id {
                                continue;
                            }
                            consensus.update_follower(follower_id);
                        }
                    }
                    Role::Follower => {
                        todo!("talk to the leader");
                    }
                },
                Source::RaftNet => match command.split_once(' ') {
                    Some(("append_entries_response", message)) => {
                        consensus.handle_append_entries_response(message);
                    }
                    Some(("append_entries_request", message)) => {
                        let (leader_id, success) = consensus.handle_append_entries_request(message);
                        consensus.respond_to_leader(leader_id, success);
                    }
                    _ => {
                        eprintln!("illegal state in raft_net {}", command)
                    }
                },
                Source::Console => {
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
                                "leader" => consensus.role = Role::Leader,
                                "follower" => consensus.role = Role::Follower,
                                _ => println!("Unknown: {}", command),
                            },
                            _ => println!("Unknown: {}", command),
                        }
                    }
                }
            }

            for message in &consensus.outbound {
                match message.splitn(3, ' ').collect::<Vec<_>>().as_slice() {
                    [server_id, message_type, message] => {
                        raft_net
                            .send(
                                server_id.parse().unwrap(),
                                &format!("{} {}", message_type, message),
                            )
                            .expect("raft_net send failed");
                    }
                    _ => {}
                }
            }
            consensus.outbound.clear();
        }
    }
}
