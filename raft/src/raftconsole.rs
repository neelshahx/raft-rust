use crate::shared::Source;
use std::io::Write;
use std::sync::mpsc::Sender;

pub struct RaftConsole {
    server_id: usize,
}

impl RaftConsole {
    pub fn new(server_id: usize) -> Self {
        RaftConsole { server_id }
    }

    pub fn start(&self, tx: Sender<(Source, String)>) {
        loop {
            print!("RC {}>", self.server_id);
            std::io::stdout().flush().ok();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            tx.send((Source::CONSOLE, input.trim().to_string())).ok();
        }
    }
}
