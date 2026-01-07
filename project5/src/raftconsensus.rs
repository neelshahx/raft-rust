use crate::raftlog::{RaftLog, RaftLogEntry};
use serde::{Deserialize, Serialize};

pub struct RaftConsensus {
    server_id: usize,
    num_servers: usize,
    is_leader: bool,
    current_term: usize,
    log: RaftLog,
    next_index: Vec<usize>,
    match_index: Vec<usize>,
    pub outbound: Vec<String>,
}

impl RaftConsensus {
    pub fn new(server_id: usize, num_servers: usize, is_leader: bool) -> Self {
        RaftConsensus {
            server_id,
            num_servers,
            is_leader,
            current_term: 1,
            log: RaftLog::new(),
            next_index: vec![1; num_servers + 1],
            match_index: vec![0; num_servers + 1],
            outbound: vec![],
        }
    }

    // LEADER FUNCTIONS
    pub fn new_client_command(&mut self, command: String) {
        assert!(self.is_leader);
        self.log.add_new_command(self.current_term, command);
    }

    pub fn update_followers(&mut self) {
        assert!(self.is_leader);
        for follower_id in 1..self.num_servers + 1 {
            if self.server_id == follower_id {
                continue;
            }
            let next_index = self.next_index[follower_id];
            let prev_index = next_index - 1;
            let prev_term = self.log.entries[prev_index].term;
            let entries = self.log.entries[next_index..next_index + 1].to_vec();
            let message = AppendEntriesRequest {
                term: self.current_term,
                leader_id: self.server_id,
                prev_index,
                prev_term,
                entries,
            };
            self.send(format!(
                "{} {}",
                follower_id,
                serde_json::to_string(&message).unwrap()
            ));
        }
    }

    pub fn handle_follower_response(&mut self, message: &str) {
        let message: AppendEntriesResponse = serde_json::from_str(message).unwrap();
        if message.success {
            self.next_index[message.follower_id] = message.match_index;
            self.next_index[message.follower_id] = message.match_index + 1;
        }
        todo!("handle term mismatch")
    }

    // FOLLOWER FUNCTIONS
    pub fn handle_append_entries(&mut self, message: &str) -> (usize, bool) {
        let message: AppendEntriesRequest = serde_json::from_str(message).unwrap();
        (
            message.leader_id,
            self.log
                .append_entries(message.prev_index, message.prev_term, message.entries),
        )
    }

    pub fn respond_to_leader(&mut self, leader_id: usize, success: bool) {
        let message = AppendEntriesResponse {
            follower_id: self.server_id,
            match_index: self.log.entries.len() - 1,
            term: self.current_term,
            success,
        };
        self.send(format!(
            "{} {}",
            leader_id,
            serde_json::to_string(&message).unwrap()
        ));
    }

    // SHARED FUNCTIONS

    pub fn send(&mut self, message: String) {
        self.outbound.push(message);
    }

    pub fn accept(&self) {
        todo!("handle append entry request or response and delegate to appropriate function");
    }
    pub fn print_log(&self) {
        println!("{:#?}", self.log);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppendEntriesRequest {
    pub term: usize,
    pub leader_id: usize,
    pub prev_index: usize,
    pub prev_term: usize,
    pub entries: Vec<RaftLogEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppendEntriesResponse {
    pub follower_id: usize,
    pub match_index: usize,
    pub term: usize,
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_replication() {
        let mut leader = RaftConsensus::new(1, 2, true);
        let mut follower = RaftConsensus::new(2, 2, false);
        leader.new_client_command("set name alice".to_string());
        leader.update_followers();
        for message in &leader.outbound {
            match message.split_once(' ') {
                Some((id_str, json_str)) => {
                    let follower_id: usize = id_str.parse().unwrap();
                    if follower_id == 2 {
                        let (leader_id, success) = follower.handle_append_entries(json_str);
                        follower.respond_to_leader(leader_id, success);
                    }
                }
                _ => {}
            }
        }
        leader.outbound.clear();
        for message in &follower.outbound {
            match message.split_once(' ') {
                Some((id_str, json_str)) => {
                    let leader_id: usize = id_str.parse().unwrap();
                    if leader_id == 1 {
                        leader.handle_follower_response(json_str);
                    }
                }
                _ => {}
            }
        }
        follower.outbound.clear();
        assert_eq!(leader.log, follower.log);
        assert_eq!(leader.next_index[2], 2);
    }
}
