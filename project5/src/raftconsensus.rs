use crate::raftlog::{RaftLog, RaftLogEntry};
use serde::{Deserialize, Serialize};

pub struct RaftConsensus {
    server_id: usize,
    current_term: usize,
    // voted_for: usize,
    log: RaftLog,
    // commit_index: usize,
    // last_applied: usize,
    next_index: Vec<usize>,
    // match_index: Vec<usize>,
    pub outbound: Vec<String>,
    pub inbound: Vec<String>,
}

impl RaftConsensus {
    pub fn new(server_id: usize, current_term: usize, num_servers: usize) -> Self {
        RaftConsensus {
            server_id,
            current_term,
            log: RaftLog::new(),
            next_index: vec![0; num_servers + 1],
            outbound: vec![],
            inbound: vec![],
        }
    }

    // LEADER FUNCTIONS
    pub fn new_client_command(&mut self, command: String) {
        self.log.add_new_command(self.current_term, command);
    }

    pub fn update_followers(&mut self) {
        let n = self.log.entries.len();
        let last_entry = self.log.entries.last().unwrap();
        let msg = AppendEntriesRequest {
            prev_index: n - 1, // incorrect in general case
            prev_term: self.log.entries[..n - 1].last().unwrap().term, // won't work on log of length 1
            entries: vec![last_entry.clone()],
        };
        self.send(serde_json::to_string(&msg).unwrap());
    }

    pub fn handle_follower_response(&self) {
        todo!("advance next index");
    }

    // FOLLOWER FUNCTIONS
    pub fn handle_append_entries(&mut self, message: String) {
        let msg: AppendEntriesRequest = serde_json::from_str(&message).unwrap();
        self.log.append_entries(msg.prev_index, msg.prev_term, msg.entries);
    }

    pub fn respond_to_leader(&self) {
        todo!();
    }

    pub fn send(&mut self, message: String) {
        println!("{} pushed message to outbound queue {}", self.server_id, message);
        self.outbound.push(message);
    }

    pub fn accept(&self) {
        todo!();
    }
    pub fn print_log(&self) {
        println!("{:#?}", self.log);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppendEntriesRequest {
    pub prev_index: usize,
    pub prev_term: usize,
    pub entries: Vec<RaftLogEntry>,
}
