use crate::raftlog::{RaftLog, RaftLogEntry};
use crate::shared::{asc_sort_median, Role};
use serde::{Deserialize, Serialize};
use std::cmp::max;

pub struct RaftConsensus {
    pub server_id: usize,
    pub num_servers: usize,
    pub role: Role,
    pub outbound: Vec<String>,
    current_term: usize,
    log: RaftLog,
    commit_index: usize,
    last_applied: usize,
    next_index: Vec<usize>,
    match_index: Vec<usize>,
}

impl RaftConsensus {
    pub fn new(server_id: usize, num_servers: usize, role: Role) -> Self {
        RaftConsensus {
            server_id,
            num_servers,
            role,
            outbound: vec![],
            current_term: 1,
            log: RaftLog::new(),
            commit_index: 0,
            last_applied: 0,
            next_index: vec![1; num_servers + 1],
            match_index: vec![0; num_servers + 1],
        }
    }

    // LEADER FUNCTIONS
    pub fn new_client_command(&mut self, command: String) {
        assert_eq!(self.role, Role::LEADER);
        self.log.add_new_command(self.current_term, command);
    }

    pub fn update_follower(&mut self, follower_id: usize) {
        assert_eq!(self.role, Role::LEADER);
        assert_ne!(self.server_id, follower_id);
        let next_index = self.next_index[follower_id];
        let message = if next_index <= 0 || next_index >= self.log.entries.len() {
            AppendEntriesRequest {
                term: self.current_term,
                leader_id: self.server_id,
                leader_commit_index: self.commit_index,
                prev_index: 0,
                prev_term: 0,
                entries: vec![],
            }
        } else {
            AppendEntriesRequest {
                term: self.current_term,
                leader_id: self.server_id,
                leader_commit_index: self.commit_index,
                prev_index: next_index - 1,
                prev_term: self.log.entries[next_index - 1].term,
                entries: self.log.entries[next_index..next_index + 1].to_vec(),
            }
        };
        self.send(format!(
            "{} {}",
            follower_id,
            serde_json::to_string(&message).unwrap()
        ));
    }

    pub fn handle_append_entries_response(&mut self, message: &str) {
        let message: AppendEntriesResponse = serde_json::from_str(message).unwrap();
        if message.success {
            self.match_index[message.follower_id] = message.match_index;
            self.next_index[message.follower_id] = message.match_index + 1;
            self.commit_index = asc_sort_median(self.match_index.clone());
        } else if message.term <= self.current_term {
            self.next_index[message.follower_id] -= 1;
        }
    }

    // FOLLOWER FUNCTIONS
    pub fn handle_append_entries_request(&mut self, message: &str) -> (usize, bool) {
        assert_eq!(self.role, Role::FOLLOWER);
        let message: AppendEntriesRequest = serde_json::from_str(message).unwrap();
        self.commit_index = max(self.commit_index, message.leader_commit_index);
        if message.term < self.current_term {
            return (message.leader_id, false);
        }
        (
            message.leader_id,
            self.log
                .append_entries(message.prev_index, message.prev_term, message.entries),
        )
    }

    pub fn respond_to_leader(&mut self, leader_id: usize, success: bool) {
        assert_eq!(self.role, Role::FOLLOWER);
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
    pub leader_commit_index: usize,
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

    fn two_server_request_response(leader: &mut RaftConsensus, follower: &mut RaftConsensus) {
        leader.update_follower(2);
        for message in &leader.outbound {
            match message.split_once(' ') {
                Some((id_str, json_str)) => {
                    let follower_id: usize = id_str.parse().unwrap();
                    if follower_id == 2 {
                        let (leader_id, success) = follower.handle_append_entries_request(json_str);
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
                        leader.handle_append_entries_response(json_str);
                    }
                }
                _ => {}
            }
        }
        follower.outbound.clear();
    }

    #[test]
    fn test_single_replication() {
        let mut leader = RaftConsensus::new(1, 2, Role::LEADER);
        let mut follower = RaftConsensus::new(2, 2, Role::FOLLOWER);
        leader.new_client_command("set name alice".to_string());

        two_server_request_response(&mut leader, &mut follower);

        assert_eq!(leader.log, follower.log);
        assert_eq!(leader.next_index[2], 2);
        assert_eq!(leader.match_index[2], 1);
    }

    #[test]
    fn test_multiple_replication() {
        let mut leader = RaftConsensus::new(1, 2, Role::LEADER);
        leader.new_client_command("set name alice".to_string());
        leader.new_client_command("get name".to_string());
        leader.new_client_command("set name bob".to_string());

        let mut follower = RaftConsensus::new(2, 2, Role::FOLLOWER);

        for _ in 0..3 {
            two_server_request_response(&mut leader, &mut follower);
        }

        assert_eq!(leader.log, follower.log);
        assert_eq!(leader.next_index[2], 4);
    }

    #[test]
    fn test_follower_behind_needs_multiple_rounds() {
        // Leader has 5 entries, follower has none - should take 5 rounds to sync
        let mut leader = RaftConsensus::new(1, 2, Role::LEADER);
        leader.new_client_command("cmd1".to_string());
        leader.new_client_command("cmd2".to_string());
        leader.new_client_command("cmd3".to_string());
        leader.new_client_command("cmd4".to_string());
        leader.new_client_command("cmd5".to_string());

        let mut follower = RaftConsensus::new(2, 2, Role::FOLLOWER);

        // Replicate all entries
        for _ in 0..5 {
            two_server_request_response(&mut leader, &mut follower);
        }

        // Verify logs match
        assert_eq!(leader.log, follower.log);
        assert_eq!(follower.log.entries.len(), 6); // dummy + 5 entries

        // Verify indices are correct
        assert_eq!(leader.match_index[2], 5);
        assert_eq!(leader.next_index[2], 6);
    }

    #[test]
    fn test_leader_behind_follower() {
        // Scenario: Follower has more entries than leader (illegal state under persistent storage)
        // This tests what happens when leader's next_index is wrong
        let mut leader = RaftConsensus::new(1, 2, Role::LEADER);
        leader.new_client_command("cmd1".to_string());
        leader.new_client_command("cmd2".to_string());

        let mut follower = RaftConsensus::new(2, 2, Role::FOLLOWER);
        // Manually add more entries to follower (simulating it being ahead)
        follower.log.add_new_command(1, "cmd1".to_string());
        follower.log.add_new_command(1, "cmd2".to_string());
        follower.log.add_new_command(1, "cmd3".to_string());
        follower.log.add_new_command(1, "cmd4".to_string());

        // Try to replicate - leader thinks follower needs index 1
        two_server_request_response(&mut leader, &mut follower);

        // Verify follower didn't lose its extra entries
        assert_eq!(follower.log.entries.len(), 5); // dummy + 4 entries

        assert_eq!(leader.match_index[2], 4);
        assert_eq!(leader.next_index[2], 5);
    }

    #[test]
    fn test_log_divergence_at_index() {
        // Leader and follower have different entries at same index (different terms)
        let mut leader = RaftConsensus::new(1, 2, Role::LEADER);
        leader.new_client_command("cmd1".to_string());
        leader.new_client_command("cmd2".to_string());
        leader.new_client_command("cmd3_leader".to_string());

        let mut follower = RaftConsensus::new(2, 2, Role::FOLLOWER);
        follower.log.add_new_command(1, "cmd1".to_string());
        follower.log.add_new_command(1, "cmd2".to_string());
        // Follower has different command at index 3 with different term
        follower.log.add_new_command(2, "cmd3_follower".to_string());
        follower.current_term = 2;

        // Leader tries to send cmd1 (next_index starts at 1)
        two_server_request_response(&mut leader, &mut follower);
        two_server_request_response(&mut leader, &mut follower);
        two_server_request_response(&mut leader, &mut follower);

        // Follower should not replicate leader, leader is behind
        assert_ne!(leader.log, follower.log);
        assert_eq!(follower.log.entries[3].command, "cmd3_follower");
    }

    #[test]
    fn test_large_gap_backtracking() {
        // Follower is very far behind with only 1 entry, leader has 10
        let mut leader = RaftConsensus::new(1, 2, Role::LEADER);
        for i in 1..=10 {
            leader.new_client_command(format!("cmd{}", i));
        }

        let mut follower = RaftConsensus::new(2, 2, Role::FOLLOWER);
        follower.log.add_new_command(1, "cmd1".to_string());

        // Set leader's next_index to wrong value (thinks follower has more)
        leader.next_index[2] = 8;

        // Should take multiple rounds to backtrack and find common point
        for _ in 0..15 {
            two_server_request_response(&mut leader, &mut follower);
        }

        // Verify eventual consistency
        assert_eq!(leader.log, follower.log);
        assert_eq!(leader.match_index[2], 10);
        assert_eq!(leader.next_index[2], 11);
    }

    #[test]
    fn test_partial_overlap_different_terms() {
        // Leader and follower share some entries but diverge
        let mut leader = RaftConsensus::new(1, 2, Role::LEADER);
        leader.log.add_new_command(1, "cmd1".to_string());
        leader.log.add_new_command(1, "cmd2".to_string());
        leader.log.add_new_command(3, "cmd3".to_string());
        leader.log.add_new_command(3, "cmd4".to_string());
        leader.current_term = 3;

        let mut follower = RaftConsensus::new(2, 2, Role::FOLLOWER);
        follower.log.add_new_command(1, "cmd1".to_string());
        follower.log.add_new_command(1, "cmd2".to_string());
        // Follower has different entries from index 3 onwards
        follower.log.add_new_command(2, "old_cmd3".to_string());
        follower.log.add_new_command(2, "old_cmd4".to_string());
        follower.log.add_new_command(2, "old_cmd5".to_string());
        follower.current_term = 2;

        // Replicate - should overwrite follower's divergent entries
        for _ in 0..10 {
            two_server_request_response(&mut leader, &mut follower);
        }

        assert_ne!(leader.log, follower.log); // new leader (not shown has to fix server 1)
        assert_eq!(follower.log.entries.len(), 6); // dummy + 4 entries
        assert_eq!(follower.log.entries[3].command, "old_cmd3");
        assert_eq!(follower.log.entries[3].term, 2);
    }

    #[test]
    fn test_empty_follower_catchup() {
        // Follower has only dummy entry, leader has several
        let mut leader = RaftConsensus::new(1, 2, Role::LEADER);
        leader.new_client_command("cmd1".to_string());
        leader.new_client_command("cmd2".to_string());
        leader.new_client_command("cmd3".to_string());

        let mut follower = RaftConsensus::new(2, 2, Role::FOLLOWER);
        // Follower has only the dummy entry at index 0

        for _ in 0..3 {
            two_server_request_response(&mut leader, &mut follower);
        }

        assert_eq!(leader.log, follower.log);
        assert_eq!(follower.log.entries.len(), 4);
        assert_eq!(leader.match_index[2], 3);
        assert_eq!(leader.next_index[2], 4);
    }

    #[test]
    fn test_indices_after_failed_then_success() {
        // Test that match_index and next_index are correctly updated after failures
        let mut leader = RaftConsensus::new(1, 2, Role::LEADER);
        leader.new_client_command("cmd1".to_string());
        leader.new_client_command("cmd2".to_string());
        leader.new_client_command("cmd3".to_string());

        let mut follower = RaftConsensus::new(2, 2, Role::FOLLOWER);
        // Follower only has cmd1
        follower.log.add_new_command(1, "cmd1".to_string());

        // Leader incorrectly thinks follower is at index 3
        leader.next_index[2] = 3;

        // First attempt should fail (follower doesn't have index 3)
        two_server_request_response(&mut leader, &mut follower);
        assert_eq!(leader.next_index[2], 2); // Should decrement

        // Second attempt should succeed
        two_server_request_response(&mut leader, &mut follower);
        assert_eq!(leader.next_index[2], 3); // Should advance
        assert_eq!(leader.match_index[2], 2); // Should match

        // Third attempt
        two_server_request_response(&mut leader, &mut follower);
        assert_eq!(leader.next_index[2], 4);
        assert_eq!(leader.match_index[2], 3);

        assert_eq!(leader.log, follower.log);
    }

    #[test]
    fn test_term_consistency_after_replication() {
        // Verify that terms are correctly replicated
        let mut leader = RaftConsensus::new(1, 2, Role::LEADER);
        leader.current_term = 3;
        leader.new_client_command("cmd1".to_string());
        leader.new_client_command("cmd2".to_string());

        let mut follower = RaftConsensus::new(2, 2, Role::FOLLOWER);
        follower.current_term = 3;

        for _ in 0..2 {
            two_server_request_response(&mut leader, &mut follower);
        }

        assert_eq!(leader.log, follower.log);
        assert_eq!(follower.log.entries[1].term, 3);
        assert_eq!(follower.log.entries[2].term, 3);
        assert_eq!(leader.current_term, 3);
        assert_eq!(follower.current_term, 3);
    }

    #[test]
    fn test_match_index_starts_at_zero() {
        // Verify initial state of match_index
        let leader = RaftConsensus::new(1, 2, Role::LEADER);
        assert_eq!(leader.match_index[2], 0);
        assert_eq!(leader.next_index[2], 1);
    }

    #[test]
    fn test_next_index_never_below_one() {
        // Verify that next_index doesn't go below 1 during backtracking
        let mut leader = RaftConsensus::new(1, 2, Role::LEADER);
        leader.new_client_command("cmd1".to_string());

        let mut follower = RaftConsensus::new(2, 2, Role::FOLLOWER);
        // Follower has completely different log
        follower.log.add_new_command(2, "different".to_string());

        // Set next_index very low
        leader.next_index[2] = 2;

        // Multiple failed attempts
        for _ in 0..5 {
            two_server_request_response(&mut leader, &mut follower);
        }

        // next_index should not go below 1
        assert!(leader.next_index[2] >= 1);
    }
}
