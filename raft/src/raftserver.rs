use crate::clientnet::ClientNet;
use crate::kvapp::KVApp;
use crate::raftconsensus::RaftConsensus;
use crate::raftconsole::RaftConsole;
use crate::raftnet::RaftNet;
use crate::shared::{InternalMessage, Role};
use std::sync::{mpsc, Arc};

pub struct RaftServer {
    server_id: usize,
    num_servers: usize,
}

impl RaftServer {
    pub fn new(server_id: usize, num_servers: usize) -> Self {
        RaftServer {
            server_id,
            num_servers,
        }
    }

    pub fn launch(self) {
        let kvapp = KVApp::new();
        let mut consensus = RaftConsensus::new(self.server_id, self.num_servers, Role::Follower);
        let client_net = Arc::new(ClientNet::new(self.server_id));
        let raft_net = Arc::new(RaftNet::new(self.server_id));
        let console = RaftConsole::new(self.server_id);

        let (tx, rx) = mpsc::channel::<InternalMessage>();

        let tx1 = tx.clone();
        std::thread::spawn(move || console.listen(tx1));
        let tx2 = tx.clone();
        std::thread::spawn(move || client_net.clone().listen(tx2));
        let tx3 = tx.clone();
        std::thread::spawn(move || raft_net.clone().listen(tx3));
        consensus.set_sender(tx);

        for message in rx {
            match message {
                InternalMessage::ConsoleCommand(command) => {
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
                _ => {}
            }

            // TODO: send heartbeat
            // match sender_type {
            //     Source::ClientNet => match consensus.role {
            //         Role::Leader => {
            //             consensus.new_client_command(command);
            //             for follower_id in 1..consensus.num_servers + 1 {
            //                 if follower_id == consensus.server_id {
            //                     continue;
            //                 }
            //                 consensus.update_follower(follower_id);
            //             }
            //         }
            //         Role::Follower => {
            //             todo!("Route to leader");
            //             // problem: we're getting a local message, need to send it over raft net
            //             // raft_net_sender.send(consensus.leader_id, command);
            //         },
            //     },
            //     Source::RaftNet => match command.split_once(' ') {
            //         Some(("append_entries_response", message)) => {
            //             consensus.handle_append_entries_response(message);
            //         }
            //         Some(("append_entries_request", message)) => {
            //             let (leader_id, success) = consensus.handle_append_entries_request(message);
            //             consensus.respond_to_leader(leader_id, success);
            //         }
            //         _ => {
            //             eprintln!("illegal state in raft_net {}", command)
            //         }
            //     },

            // }
            //
            // for message in &consensus.outbox {
            //     match message.splitn(3, ' ').collect::<Vec<_>>().as_slice() {
            //         [server_id, message_type, message] => {
            //             raft_net_sender
            //                 .send(
            //                     server_id.parse().unwrap(),
            //                     &format!("{} {}", message_type, message),
            //                 )
            //                 .expect("raft_net send failed");
            //         }
            //         _ => {}
            //     }
            // }
            // consensus.outbox.clear();
        }
    }
}
