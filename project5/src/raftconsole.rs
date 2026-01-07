use std::io::Write;
use std::sync::mpsc::Sender;

pub struct RaftConsole {
    server_id: usize,
}

impl RaftConsole {
    pub(crate) fn new(server_id: usize) -> Self {
        RaftConsole { server_id }
    }

    pub(crate) fn start(&self, tx: Sender<String>) {
        loop {
            std::io::stdout().flush().ok();
            print!("RC {}>", self.server_id);
            std::io::stdout().flush().ok();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            tx.send(input.trim().to_string()).ok();
        }
    }
}
