use crate::shared::InternalMessage;
use std::io::Write;
use std::sync::mpsc::Sender;

pub struct RaftConsole {
    server_id: usize,
}

impl RaftConsole {
    pub fn new(server_id: usize) -> Self {
        RaftConsole { server_id }
    }

    pub fn listen(&self, tx: Sender<InternalMessage>) {
        loop {
            print!("RC {}>", self.server_id);
            let _ = std::io::stdout().flush();
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
            let _ = tx.send(InternalMessage::ConsoleCommand(input.trim().to_string()));
        }
    }
}
