use crate::clientnet::ClientNet;
use crate::kvapp::KVApp;
use crate::log;
use crate::raftconsensus::{AppendEntries, RaftConsensus};
use crate::raftconsole::RaftConsole;
use crate::raftnet::RaftNet;
use crate::shared::{InternalMessage, RaftNetMessage, Role};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc};
use std::time::Duration;

pub struct RaftServer {
    server_id: usize,
    num_servers: usize,
    heard_from_leader: Arc<AtomicBool>,
}

impl RaftServer {
    pub fn new(server_id: usize, num_servers: usize) -> Self {
        RaftServer {
            server_id,
            num_servers,
            heard_from_leader: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn launch(self) {
        let mut kvapp = KVApp::new();
        let mut consensus = RaftConsensus::new(self.server_id, self.num_servers, Role::Follower);
        let client_net = Arc::new(ClientNet::new(self.server_id));
        let raft_net = Arc::new(RaftNet::new(self.server_id));
        let console = RaftConsole::new(self.server_id);

        let (tx, rx) = mpsc::channel::<InternalMessage>();
        let tx1 = tx.clone();
        std::thread::spawn(move || console.listen(tx1));
        let tx2 = tx.clone();
        let client_net2 = client_net.clone();
        std::thread::spawn(move || client_net2.listen(tx2));
        let tx3 = tx.clone();
        let raft_net2 = raft_net.clone();
        std::thread::spawn(move || raft_net2.listen(tx3));
        let tx4 = tx.clone();
        let heard_from_leader = self.heard_from_leader.clone();
        std::thread::spawn(move || election_timer(heard_from_leader, tx4));
        let tx5 = tx.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_secs(1));
            let _ = tx5.send(InternalMessage::Tick);
        });
        let tx6 = tx.clone();
        consensus.set_sender(tx6);

        for message in rx {
            match message {
                InternalMessage::ConsoleCommand(command) => {
                    if !command.trim().is_empty() {
                        match command.as_str() {
                            "state" => {
                                println!("\nheard_from_leader: {:?}", self.heard_from_leader);
                                consensus.print_consensus()
                            }
                            "log" => consensus.print_log(),
                            "leader" => consensus.role = Role::Leader,
                            "follower" => consensus.role = Role::Follower,
                            "candidate" => consensus.role = Role::Candidate,
                            _ => println!("Unknown: {}", command),
                        }
                    }
                }
                InternalMessage::StartElection => {
                    log!("Started election");
                    consensus.role = Role::Candidate;
                    consensus.current_term += 1;
                    consensus.voted_for = Some(self.server_id);
                    let mut votes = 1;
                    let majority = self.num_servers / 2 + 1;
                    let last_log_index = consensus.log.entries.len() - 1;
                    let last_log_term = consensus.log.entries[last_log_index].term;
                    let message = RaftNetMessage::RequestVote(RequestVote {
                        term: consensus.current_term,
                        candidate_id: self.server_id,
                        last_log_index,
                        last_log_term,
                    });
                    for follower_id in 1..self.num_servers + 1 {
                        if follower_id != self.server_id {
                            let payload = serde_json::to_string(&message).unwrap();
                            log!("Sending RequestVote {} {:#?}", follower_id, payload);
                            raft_net.send(follower_id, &message).unwrap();
                        }
                    }
                }
                InternalMessage::IncomingRaftMessage(raft_message) => match raft_message {
                    RaftNetMessage::RequestVote(request_vote) => {
                        println!("Vote request received {:?}", request_vote);
                        self.heard_from_leader.store(true, Ordering::Relaxed);
                    }
                    RaftNetMessage::RequestVoteResponse(request_vote_response) => {}
                    RaftNetMessage::AppendEntries(append_entries) => {
                        self.heard_from_leader.store(true, Ordering::Relaxed);
                    }
                    _ => {}
                },
                InternalMessage::AppendEntries {server_id, payload} => {
                    raft_net.send(server_id, &RaftNetMessage::AppendEntries(payload)).unwrap();
                }
                InternalMessage::Tick => {
                    // heartbeats/catchup followers
                    if consensus.role == Role::Leader {
                        for server_id in 1..self.num_servers + 1 {
                            if server_id != self.server_id {
                                consensus.update_follower(server_id);
                            }
                        }
                    }
                }
                _ => {}
            }

            if consensus.role == Role::Leader || consensus.role == Role::Candidate {
                self.heard_from_leader.store(true, Ordering::Relaxed);
            }



            // update state machine
            while consensus.commit_index > kvapp.last_applied {
                let command = consensus.log.entries[kvapp.last_applied + 1]
                    .command
                    .clone();
                log! {"applied command {}", command}
                kvapp.apply_command(command);
                kvapp.last_applied += 1;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestVote {
    term: usize,
    candidate_id: usize,
    last_log_index: usize,
    last_log_term: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestVoteResponse {
    term: usize,
    voteGranted: bool,
}

pub fn election_timer(heard_from_leader: Arc<AtomicBool>, tx: Sender<InternalMessage>) {
    let mut rng = rand::rng();
    loop {
        heard_from_leader.store(false, Ordering::Relaxed);
        std::thread::sleep(Duration::from_secs(rng.random_range(3..=5)));
        if !heard_from_leader.load(Ordering::Relaxed) {
            let _ = tx.send(InternalMessage::StartElection);
            std::thread::sleep(Duration::from_secs(5));
        }
    }
}
