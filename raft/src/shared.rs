use serde::{Deserialize, Serialize};

pub const DEBUG: bool = true;

#[macro_export]
macro_rules! log {
      ($($arg:tt)*) => {
          if $crate::shared::DEBUG {
              eprintln!($($arg)*);
          }
      }
  }

pub const APP_SERVERS: [(u8, &str); 6] = [
    (0, "0.0.0.0:0"),
    (1, "127.0.0.1:21000"),
    (2, "127.0.0.1:22000"),
    (3, "127.0.0.1:23000"),
    (4, "127.0.0.1:24000"),
    (5, "127.0.0.1:25000"),
];

pub const RAFT_SERVERS: [(u8, &str); 6] = [
    (0, "0.0.0.0:0"),
    (1, "127.0.0.1:11000"),
    (2, "127.0.0.1:12000"),
    (3, "127.0.0.1:13000"),
    (4, "127.0.0.1:14000"),
    (5, "127.0.0.1:15000"),
];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InternalMessage {
    ClientCommand { addr: String, payload: String },
    ConsoleCommand(String),
    RaftNet(String),
    AppendEntriesRequest { server_id: usize, payload: String },
    AppendEntriesResponse { server_id: usize, payload: String },
}

#[derive(Debug, PartialEq)]
pub enum Role {
    Leader,
    Follower,
}

pub fn asc_sort_median(match_index: Vec<usize>) -> usize {
    let mut match_index = match_index[1..].to_vec();
    match_index.sort();
    if match_index.is_empty() {
        0
    } else if match_index.len() % 2 == 0 {
        match_index[match_index.len() / 2 - 1]
    } else {
        match_index[match_index.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_match_index() {
        let match_index = Vec::from([0]);
        assert_eq!(asc_sort_median(match_index), 0);
    }

    #[test]
    fn test_match_index_len1() {
        let match_index = Vec::from([0, 1]);
        assert_eq!(asc_sort_median(match_index), 1);
    }

    #[test]
    fn test_match_index_len2() {
        let match_index = Vec::from([0, 2, 1]);
        assert_eq!(asc_sort_median(match_index), 1);
    }

    #[test]
    fn test_match_index_len3() {
        let match_index = Vec::from([0, 3, 2, 1]);
        assert_eq!(asc_sort_median(match_index), 2);
    }
}
