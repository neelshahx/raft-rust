use crate::raftlog::RaftLog;

pub struct RaftConsensus {
    server_id: usize,
    current_term: usize,
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
        self.log.add_new_command(self.current_term, command); // TODO: validate command?
    }

    pub fn update_followers(&self) {
        todo!();
    }

    pub fn handle_follower_response() {
        todo!();
    }

    pub fn advance_next_index() {
        todo!();
    }

    // FOLLOWER FUNCTIONS
    pub fn handle_append_entries() {
        todo!();
    }

    pub fn respond_to_leader() {
        todo!();
    }

    pub fn send(&self) {
        todo!();
    }

    pub fn accept(&self) {
        todo!();
    }
    pub fn print_log(&self) {
        println!("{:?}", self.log);
    }
}
