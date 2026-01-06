use std::vec::Vec;

#[derive(Clone)]
pub struct LogEntry {
    term_index: usize,
    command: String,
}

impl LogEntry {
    fn new(term_index: usize, command: String) -> Self {
        LogEntry {
            term_index,
            command,
        }
    }
}

pub struct RaftLog {
    log: Vec<LogEntry>,
}

impl RaftLog {
    fn new() -> Self {
        RaftLog {
            log: vec![LogEntry::new(0, "".to_string())],
        }
    }

    fn add_new_command(&mut self, leader_term: usize, command: String) {
        self.log
            .push(LogEntry::new(leader_term, command));
    }

    fn append_entries(
        &mut self,
        prev_index: usize,
        prev_term: usize,
        entries: Vec<LogEntry>,
    ) -> bool {
        if prev_index >= self.log.len() {
            return false;
        }
        if self.log.len() > 1 && self.log[prev_index].term_index != prev_term {
            return false;
        }
        let slice = &self.log[prev_index+1..];
        if slice.is_empty() {
            for entry in entries {
                self.log.push(entry)
            }
            return true;
        }
        let mut n: usize = 0;
        for (item1, item2) in slice.iter().zip(entries.iter()) {
            if item1.term_index == item2.term_index {
                n += 1;
            } else {
                self.log.truncate(prev_index + n + 1);
                break;
            }
        }
        for item in &entries[n..] {
            self.log.push(item.clone());
        }
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
        assert_eq!(log.log[1].term_index, 1);
        assert_eq!(log.log[1].command, "set name alice");
    }

    #[test]
    fn test_append_entries() {
        let mut log = RaftLog::new();
        log.add_new_command(1, "set name alice".to_string());

        let entries: Vec<LogEntry> = vec![
            LogEntry::new(2, "delete name".to_string()),
            LogEntry::new(2, "set name bob".to_string()),
        ];

        log.append_entries(1, 1, entries);
        assert_eq!(log.log.len(), 4);
        assert_eq!(log.log[1].command, "set name alice");
        assert_eq!(log.log[2].command, "delete name");
        assert_eq!(log.log[3].command, "set name bob");
    }

    #[test]
    fn test_ae_prev_index_oob() {
        let mut log = RaftLog::new();
        log.add_new_command(1, "set name alice".to_string());
        let entries = vec![LogEntry::new(2, "delete name".to_string())];
        assert_eq!(log.append_entries(2, 1, entries), false);
        assert_eq!(log.log.len(), 2);
        assert_eq!(log.log[1].command, "set name alice");
    }


    #[test]
    fn test_ae_prev_term_mismatch() {
        let mut log = RaftLog::new();
        log.add_new_command(1, "set name alice".to_string());
        let entries = vec![LogEntry::new(2, "delete name".to_string())];
        assert_eq!(log.append_entries(1, 2, entries), false);
        assert_eq!(log.log.len(), 2);
        assert_eq!(log.log[1].command, "set name alice");
    }
}
