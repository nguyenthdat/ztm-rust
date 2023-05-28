enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn print_power_state(state: PowerState) {
    use PowerState::*;
    match state {
        Off => print!("turning off"),
        Sleep => print!("sleeping"),
        Reboot => print!("rebooting"),
        Shutdown => print!("shutting down"),
        Hibernate => print!("hibernating"),
    }
}

fn main() {
    use std::io;
    let mut buffer = String::new();
    let user_input_status = io::stdin().read_line(&mut buffer);
    if user_input_status.is_ok() {
        match PowerState::new(&buffer) {
            Some(state) => print_power_state(state),
            None => print!("Invalid power state"),
        }
    } else {
        print!("error reading input")
    }
}
