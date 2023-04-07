use colored::Colorize;
use crossbeam_channel::{unbounded, Receiver};
use std::thread::{self, JoinHandle};

enum LightMsg {
    ChangeColor(u8, u8, u8),
    Disconnect,
    On,
    Off,
}

#[derive(Debug)]
enum LightStatus {
    Off,
    On,
}

fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    thread::spawn(move || {
        let mut light_status = LightStatus::Off;

        loop {
            if let Ok(msg) = receiver.recv() {
                match msg {
                    LightMsg::ChangeColor(r, g, b) => {
                        println!("Color changed to: {}", "      ".on_truecolor(r, g, b));
                        match light_status {
                            LightStatus::Off => println!("Light is OFF"),
                            LightStatus::On => println!("Light is ON"),
                        }
                    }
                    LightMsg::On => {
                        println!("Turned light on");
                        light_status = LightStatus::On;
                    }
                    LightMsg::Off => {
                        println!("Turned light off");
                        light_status = LightStatus::Off
                    }
                    LightMsg::Disconnect => {
                        println!("Disconnecting");
                        light_status = LightStatus::Off;
                        break;
                    }
                }
            } else {
                println!("channel disconnected");
                light_status = LightStatus::Off;
                break;
            }
        }

        light_status
    })
}

fn main() {
    let (s, r) = unbounded();
    let light = spawn_light_thread(r);

    s.send(LightMsg::On).expect("Error sending message");
    s.send(LightMsg::ChangeColor(255, 0, 0))
        .expect("Error sending message");
    s.send(LightMsg::ChangeColor(0, 128, 0))
        .expect("Error sending message");
    s.send(LightMsg::ChangeColor(0, 0, 255))
        .expect("Error sending message");
    s.send(LightMsg::Off).expect("Error sending message");
    s.send(LightMsg::Disconnect).expect("Error sending message");

    match light.join() {
        Ok(value) => println!("{:?}", value),
        Err(error) => println!("{:?}", error),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;

    #[test]
    fn light_off_when_disconnect() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        s.send(LightMsg::Disconnect).expect("channel disconnected");

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after disconnection");
        }
    }

    #[test]
    fn light_off_when_dropped() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        drop(s);

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after dropping sender");
        }
    }
}
