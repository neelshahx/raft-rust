// red to green, no yellow
// green to yellow to red
// 4 total states N/S/E/W
// 2 complementary states N=S, E=W

// send on 10000
// listen on 11000

use std::net::SocketAddr;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone)]
pub enum LightState {
    GR,
    YR,
    RG,
    RY,
}

struct SystemState {
    current_state: LightState,
}

impl SystemState {
    fn new() -> Self {
        SystemState {
            current_state: LightState::GR,
        }
    }

    fn update_state(&mut self, time: usize) -> Option<LightState> {
        let new_state = match self.current_state {
            LightState::GR if time > 30 => LightState::YR,
            LightState::YR if time > 5 => LightState::RG,
            LightState::RG if time > 60 => LightState::RY,
            LightState::RY if time > 5 => LightState::GR,
            _ => return None,
        };
        self.current_state = new_state.clone();
        Some(new_state)
    }
}

fn handle_light_state_change(new_state: LightState) -> std::io::Result<()> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_nonblocking(false).unwrap();
    let addr: SocketAddr = "127.0.0.1:10000".parse().unwrap();
    let states = match new_state {
        LightState::GR => {("NS-G", "EW-R")},
        LightState::YR => {("NS-Y", "EW-R")},
        LightState::RG => {("NS-R", "EW-G")},
        LightState::RY => {("NS-R", "EW-Y")}
    };
    socket.send_to(states.0.as_bytes(), addr)?;
    socket.send_to(states.1.as_bytes(), addr)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel::<String>();

    let tx1 = tx.clone();
    let clock = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            tx1.send("TICK".to_string()).unwrap();
        }
    });

    // listen for button push
    let tx2 = tx.clone();
    thread::spawn(move || {
        let socket = std::net::UdpSocket::bind("127.0.0.1:11000").unwrap();
        socket.set_nonblocking(false).unwrap();
        loop {
            let mut buf = [0u8; 1024];
            let (n, _) = socket.recv_from(&mut buf).unwrap();
            if n > 0 {
                tx2.send(String::from_utf8_lossy(&buf[..n]).into_owned())
                    .unwrap();
            }
        }
    });

    let mut t = 0;
    let mut state = SystemState::new();
    for recv in rx {
        println!("{}", recv);
        if recv == "TICK" {
            t += 1;
        }
        let result = state.update_state(t);
        if let Some(new) = result {
            handle_light_state_change(new);
            t = 0;
        }
    }

    clock.join().unwrap();

    //
    // // init the clock
    // // push to queue
    //
    // // read
    // // independent
    // let mut buf = [0u8; 1024];
    // let (n, _) = socket.recv_from(&mut buf)?;
    // println!("{}", String::from_utf8_lossy(&buf[..n]));
    Ok(())
}

// things to test
//
