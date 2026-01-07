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

#[derive(Debug, PartialEq)]
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
    fn new(initial_state: LightState) -> Self {
        SystemState {
            current_state: initial_state,
        }
    }

    fn update_state(&mut self, time: usize) -> Option<&LightState> {
        let new_state = match self.current_state {
            LightState::GR if time > 30 => LightState::YR,
            LightState::YR if time > 5 => LightState::RG,
            LightState::RG if time > 60 => LightState::RY,
            LightState::RY if time > 5 => LightState::GR,
            _ => return None,
        };
        self.current_state = new_state;
        Some(&self.current_state)
    }
}

fn convert_state_to_command(state: &LightState) -> (&str, &str) {
    match state {
        LightState::GR => ("EW-R", "NS-G"),
        LightState::YR => ("NS-Y", "EW-R"),
        LightState::RG => ("NS-R", "EW-G"),
        LightState::RY => ("NS-R", "EW-Y"),
    }
}

fn update_lights(new_state: &LightState) {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_nonblocking(false).unwrap();
    let addr: SocketAddr = "127.0.0.1:10000".parse().unwrap();
    let (ns_cmd, ew_cmd) = convert_state_to_command(new_state);
    socket.send_to(ns_cmd.as_bytes(), addr).unwrap();
    socket.send_to(ew_cmd.as_bytes(), addr).unwrap();
}

fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel::<String>();

    // clock
    let tx1 = tx.clone();
    let clock = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            tx1.send("TICK".to_string()).unwrap();
        }
    });

    // button push listener
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

    // main thread = single consumer
    let mut t = 0;
    let mut state = SystemState::new(LightState::GR);
    update_lights(&LightState::GR);
    for recv in rx {
        println!("{}", recv);
        if recv == "TICK" {
            t += 1;
        }
        let result = state.update_state(t);
        if let Some(new_state) = result {
            update_lights(new_state);
            t = 0;
        }
        // TODO: handle button push
        // if recv = NS-button , check state/time, stop EW lights or wait
        // if recv = EW-button , check state/time, stop NS lights or wait
    }

    clock.join().unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_state_gr_to_yr() {
        let mut lights = SystemState::new(LightState::GR);
        assert_eq!(lights.current_state, LightState::GR);
        lights.update_state(30);
        assert_eq!(lights.current_state, LightState::GR);
        lights.update_state(31);
        assert_eq!(lights.current_state, LightState::YR);
    }

    #[test]
    fn test_system_state_yr_to_rg() {
        let mut lights = SystemState::new(LightState::YR);
        assert_eq!(lights.current_state, LightState::YR);
        lights.update_state(5);
        assert_eq!(lights.current_state, LightState::YR);
        lights.update_state(6);
        assert_eq!(lights.current_state, LightState::RG);
    }

    #[test]
    fn test_system_state_rg_to_ry() {
        let mut lights = SystemState::new(LightState::RG);
        assert_eq!(lights.current_state, LightState::RG);
        lights.update_state(60);
        assert_eq!(lights.current_state, LightState::RG);
        lights.update_state(61);
        assert_eq!(lights.current_state, LightState::RY);
    }

    #[test]
    fn test_system_state_ry_to_gr() {
        let mut lights = SystemState::new(LightState::RY);
        assert_eq!(lights.current_state, LightState::RY);
        lights.update_state(5);
        assert_eq!(lights.current_state, LightState::RY);
        lights.update_state(6);
        assert_eq!(lights.current_state, LightState::GR);
    }

    #[test]
    fn test_system_state_full_cycle() {
        let mut lights = SystemState::new(LightState::GR);

        // GR -> YR (30 seconds)
        assert_eq!(lights.current_state, LightState::GR);
        assert_eq!(lights.update_state(31), Some(&LightState::YR));

        // YR -> RG (5 seconds)
        assert_eq!(lights.update_state(6), Some(&LightState::RG));

        // RG -> RY (60 seconds)
        assert_eq!(lights.update_state(61), Some(&LightState::RY));

        // RY -> GR (5 seconds)
        assert_eq!(lights.update_state(6), Some(&LightState::GR));

        assert_eq!(lights.current_state, LightState::GR);
    }

    #[test]
    fn test_system_state_no_transition() {
        let mut lights = SystemState::new(LightState::GR);
        assert_eq!(lights.update_state(0), None);
        assert_eq!(lights.update_state(15), None);
        assert_eq!(lights.update_state(30), None);
        assert_eq!(lights.current_state, LightState::GR);
    }

    #[test]
    fn test_convert_state_to_command_gr() {
        let state = LightState::GR;
        let (cmd1, cmd2) = convert_state_to_command(&state);
        assert_eq!(cmd1, "EW-R");
        assert_eq!(cmd2, "NS-G");
    }

    #[test]
    fn test_convert_state_to_command_yr() {
        let state = LightState::YR;
        let (cmd1, cmd2) = convert_state_to_command(&state);
        assert_eq!(cmd1, "NS-Y");
        assert_eq!(cmd2, "EW-R");
    }

    #[test]
    fn test_convert_state_to_command_rg() {
        let state = LightState::RG;
        let (cmd1, cmd2) = convert_state_to_command(&state);
        assert_eq!(cmd1, "NS-R");
        assert_eq!(cmd2, "EW-G");
    }

    #[test]
    fn test_convert_state_to_command_ry() {
        let state = LightState::RY;
        let (cmd1, cmd2) = convert_state_to_command(&state);
        assert_eq!(cmd1, "NS-R");
        assert_eq!(cmd2, "EW-Y");
    }
}
