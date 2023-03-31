use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        let state: String = state.trim().to_lowercase();

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

fn print_power_action(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("turning off"),
        Sleep => println!("sleeping"),
        Reboot => println!("rebooting"),
        Shutdown => println!("shutting down"),
        Hibernate => println!("hibernating"),
    }
}

fn main() {
    let mut buffer: String = String::new();
    println!("Enter power state:");

    let user_input_status = io::stdin().read_line(&mut buffer);
    if user_input_status.is_ok() {
        match PowerState::new(&buffer) {
            Some(state) => print_power_action(state),
            None => println!("invalid power state"),
        }
    } else {
        println!("An error occur");
    }
}
