use std::vec::Vec;

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
        self.log.push(LogEntry::new(leader_term, command));
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
        if self.log[prev_index].term_index != prev_term {
            return false;
        }
        let slice = &self.log[prev_index + 1..];
        if slice.is_empty() {
            for entry in entries {
                self.log.push(entry)
            }
            return true;
        }
        for (item1, item2) in slice.iter().zip(entries.iter()) {
            if item1.term_index != item2.term_index {
                return false;
            }
        }
        for item in entries.into_iter().skip(slice.len()) {
            self.log.push(item);
        }
        true
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
