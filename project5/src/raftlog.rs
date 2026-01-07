use std::vec::Vec;

pub struct RaftLogEntry {
    term: usize,
    command: String,
}

impl RaftLogEntry {
    fn new(term: usize, command: String) -> Self {
        RaftLogEntry { term, command }
    }
}

pub struct RaftLog {
    log: Vec<RaftLogEntry>,
}

impl RaftLog {
    pub(crate) fn new() -> Self {
        RaftLog {
            log: vec![RaftLogEntry::new(0, "".to_string())],
        }
    }

    fn add_new_command(&mut self, leader_term: usize, command: String) {
        self.log.push(RaftLogEntry::new(leader_term, command));
    }

    fn append_entries(
        &mut self,
        prev_index: usize,
        prev_term: usize,
        entries: Vec<RaftLogEntry>,
    ) -> bool {
        if prev_index >= self.log.len() {
            return false;
        }
        if self.log.len() > 1 && self.log[prev_index].term != prev_term {
            return false;
        }
        let slice = &self.log[prev_index + 1..];
        if slice.is_empty() {
            for entry in entries {
                self.log.push(entry)
            }
            return true;
        }
        let mut n: usize = 0;
        for (item1, item2) in slice.iter().zip(entries.iter()) {
            if item1.term == item2.term {
                n += 1;
            } else {
                self.log.truncate(prev_index + n + 1);
                break;
            }
        }
        self.log.extend(entries.into_iter().skip(n));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialise() {
        let log = RaftLog::new();
        assert_eq!(log.log.len(), 1);
    }

    #[test]
    fn test_add_command() {
        let mut log = RaftLog::new();
        log.add_new_command(1, "set name alice".to_string());
        assert_eq!(log.log[1].term, 1);
        assert_eq!(log.log[1].command, "set name alice");
    }

    #[test]
    fn test_append_entries() {
        let mut log = RaftLog::new();
        log.add_new_command(1, "set name alice".to_string());

        let entries: Vec<RaftLogEntry> = vec![
            RaftLogEntry::new(2, "delete name".to_string()),
            RaftLogEntry::new(2, "set name bob".to_string()),
        ];

        log.append_entries(1, 1, entries);
        assert_eq!(log.log.len(), 4);
        assert_eq!(log.log[1].command, "set name alice");
        assert_eq!(log.log[2].command, "delete name");
        assert_eq!(log.log[3].command, "set name bob");
    }

    #[test]
    fn test_append_prev_index_oob() {
        let mut log = RaftLog::new();
        log.add_new_command(1, "set name alice".to_string());
        let entries = vec![RaftLogEntry::new(2, "delete name".to_string())];
        assert_eq!(log.append_entries(2, 1, entries), false);
        assert_eq!(log.log.len(), 2);
        assert_eq!(log.log[1].command, "set name alice");
    }

    #[test]
    fn test_append_prev_term_mismatch() {
        let mut log = RaftLog::new();
        log.add_new_command(1, "set name alice".to_string());
        let entries = vec![RaftLogEntry::new(2, "delete name".to_string())];
        assert_eq!(log.append_entries(1, 2, entries), false);
        assert_eq!(log.log.len(), 2);
        assert_eq!(log.log[1].command, "set name alice");
    }

    #[test]
    fn test_append_to_empty_log() {
        // Test appending when log only has the initial dummy entry
        let mut log = RaftLog::new();
        let entries = vec![
            RaftLogEntry::new(1, "set x 1".to_string()),
            RaftLogEntry::new(1, "set y 2".to_string()),
        ];
        assert!(log.append_entries(0, 0, entries));
        assert_eq!(log.log.len(), 3);
        assert_eq!(log.log[1].command, "set x 1");
        assert_eq!(log.log[2].command, "set y 2");
    }

    #[test]
    fn test_append_empty_entries() {
        // Test heartbeat scenario - appending empty entries
        let mut log = RaftLog::new();
        log.add_new_command(1, "set name alice".to_string());
        let entries: Vec<RaftLogEntry> = vec![];
        assert!(log.append_entries(1, 1, entries));
        assert_eq!(log.log.len(), 2);
        assert_eq!(log.log[1].command, "set name alice");
    }

    #[test]
    fn test_conflict_resolution_truncate() {
        // Test that conflicting entries cause truncation
        let mut log = RaftLog::new();
        log.add_new_command(1, "set x 1".to_string());
        log.add_new_command(1, "set y 2".to_string());
        log.add_new_command(2, "set z 3".to_string());

        // New leader sends conflicting entries from index 2
        let entries = vec![
            RaftLogEntry::new(3, "set a 10".to_string()),
            RaftLogEntry::new(3, "set b 20".to_string()),
        ];

        assert!(log.append_entries(1, 1, entries));
        assert_eq!(log.log.len(), 4);
        assert_eq!(log.log[2].term, 3);
        assert_eq!(log.log[2].command, "set a 10");
        assert_eq!(log.log[3].command, "set b 20");
    }

    #[test]
    fn test_partial_overlap() {
        // Test when some entries match, some are new
        let mut log = RaftLog::new();
        log.add_new_command(1, "cmd1".to_string());
        log.add_new_command(1, "cmd2".to_string());

        let entries = vec![
            RaftLogEntry::new(1, "cmd2".to_string()), // matches
            RaftLogEntry::new(2, "cmd3".to_string()), // new
            RaftLogEntry::new(2, "cmd4".to_string()), // new
        ];

        assert!(log.append_entries(1, 1, entries));
        assert_eq!(log.log.len(), 5);
        assert_eq!(log.log[2].command, "cmd2");
        assert_eq!(log.log[3].command, "cmd3");
        assert_eq!(log.log[4].command, "cmd4");
    }

    #[test]
    fn test_multiple_add_commands() {
        // Test adding multiple commands sequentially
        let mut log = RaftLog::new();
        log.add_new_command(1, "cmd1".to_string());
        log.add_new_command(1, "cmd2".to_string());
        log.add_new_command(2, "cmd3".to_string());
        log.add_new_command(2, "cmd4".to_string());

        assert_eq!(log.log.len(), 5);
        assert_eq!(log.log[1].term, 1);
        assert_eq!(log.log[2].term, 1);
        assert_eq!(log.log[3].term, 2);
        assert_eq!(log.log[4].term, 2);
    }

    #[test]
    fn test_sequential_appends() {
        // Test multiple append_entries calls in sequence
        let mut log = RaftLog::new();

        // First append (assume RPC sends prev index = 0, prev term = 0)
        let entries1 = vec![RaftLogEntry::new(1, "cmd1".to_string())];
        assert!(log.append_entries(0, 0, entries1));
        assert_eq!(log.log.len(), 2);

        // Second append
        let entries2 = vec![RaftLogEntry::new(1, "cmd2".to_string())];
        assert!(log.append_entries(1, 1, entries2));
        assert_eq!(log.log.len(), 3);

        // Third append
        let entries3 = vec![
            RaftLogEntry::new(2, "cmd3".to_string()),
            RaftLogEntry::new(2, "cmd4".to_string()),
        ];
        assert!(log.append_entries(2, 1, entries3));
        assert_eq!(log.log.len(), 5);
    }

    #[test]
    fn test_idempotent_append() {
        // Test appending the same entries multiple times
        let mut log = RaftLog::new();
        log.add_new_command(1, "cmd1".to_string());

        let entries = vec![
            RaftLogEntry::new(2, "cmd2".to_string()),
            RaftLogEntry::new(2, "cmd3".to_string()),
        ];

        // First append
        assert!(log.append_entries(1, 1, entries));
        assert_eq!(log.log.len(), 4);

        let entries = vec![
            RaftLogEntry::new(2, "cmd2".to_string()),
            RaftLogEntry::new(2, "cmd3".to_string()),
        ];

        // Appending the same entries again should be idempotent
        assert!(log.append_entries(1, 1, entries));
        assert_eq!(log.log.len(), 4);
        assert_eq!(log.log[2].command, "cmd2");
        assert_eq!(log.log[3].command, "cmd3");
    }

    #[test]
    fn test_append_with_gap() {
        // Test appending with a gap in the log (should fail)
        let mut log = RaftLog::new();
        log.add_new_command(1, "cmd1".to_string());

        // Try to append at index 3 when log only has index 1
        let entries = vec![RaftLogEntry::new(2, "cmd2".to_string())];
        assert!(!log.append_entries(3, 2, entries));
        assert_eq!(log.log.len(), 2); // Log unchanged
    }

    #[test]
    fn test_conflicting_terms_same_index() {
        // Test entries with same commands but different terms
        let mut log = RaftLog::new();
        log.add_new_command(1, "cmd1".to_string());
        log.add_new_command(2, "cmd2".to_string());
        log.add_new_command(2, "cmd3".to_string());

        // Leader from term 3 sends different entries
        let entries = vec![
            RaftLogEntry::new(3, "cmd2".to_string()), // same command, different term
            RaftLogEntry::new(3, "cmd4".to_string()), // new command
        ];

        assert!(log.append_entries(1, 1, entries));
        assert_eq!(log.log.len(), 4);
        assert_eq!(log.log[2].term, 3);
        assert_eq!(log.log[3].term, 3);
    }

    #[test]
    fn test_append_extends_existing() {
        // Test appending when new entries extend beyond existing log
        let mut log = RaftLog::new();
        log.add_new_command(1, "cmd1".to_string());
        log.add_new_command(1, "cmd2".to_string());

        let entries = vec![
            RaftLogEntry::new(1, "cmd2".to_string()), // matches existing
            RaftLogEntry::new(2, "cmd3".to_string()), // extends
            RaftLogEntry::new(2, "cmd4".to_string()), // extends
            RaftLogEntry::new(2, "cmd5".to_string()), // extends
        ];

        assert!(log.append_entries(1, 1, entries));
        assert_eq!(log.log.len(), 6);
        assert_eq!(log.log[5].command, "cmd5");
    }

    #[test]
    fn test_prev_index_at_zero() {
        // Test that prev_index=0, prev_term=0 works correctly
        let mut log = RaftLog::new();
        let entries = vec![RaftLogEntry::new(1, "first".to_string())];
        assert!(log.append_entries(0, 0, entries));
        assert_eq!(log.log.len(), 2);
        assert_eq!(log.log[1].command, "first");
    }
}
