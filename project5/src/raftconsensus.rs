use crate::raftlog::RaftLog;

pub struct RaftConsensus {
    server_id: usize,
    // current_term: usize,
    // voted_for: usize,
    log: RaftLog,
    // commit_index: usize,
    // last_applied: usize,
    next_index: Vec<usize>,
    // match_index: Vec<usize>,
    outbound: Vec<String>,
    inbound: Vec<String>,
}

impl RaftConsensus {
    pub fn new(server_id: usize, num_servers: usize) -> Self {
        RaftConsensus {
            server_id,
            log: RaftLog::new(),
            next_index: vec![0; num_servers + 1],
            outbound: vec![],
            inbound: vec![],
        }
    }

    // LEADER FUNCTIONS
    fn new_client_command() {
        todo!();
    }

    fn update_follower() {
        todo!();
    }

    fn handle_follower_response() {
        todo!();
    }

    fn advance_next_index() {
        todo!();
    }

    // FOLLOWER FUNCTIONS
    fn handle_append_entries() {
        todo!();
    }

    fn respond_to_leader() {
        todo!();
    }

    fn send(&self) {
        todo!();
    }

    fn accept(&self) {
        todo!();
    }
}
