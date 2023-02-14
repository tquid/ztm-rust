// Topic: Channels
//
// Summary:
//   Using the existing code, create a program that simulates an internet-of-things
//   remote control light bulb. The color of the light can be changed remotely.
//   Use threads and channels to communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//   * Display whether the light is on or off on each color change
// * Turn off the light when disconnecting from it
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates
// * Disconnection can be accomplished by dropping the sender, or
//   by telling the thread to self-terminate
// * Use `cargo test --bin a39` to test your program to ensure all cases are covered

use crossbeam_channel::{unbounded, Receiver};
use std::thread::{self, JoinHandle};

enum LightMsg {
    // Add additional variants needed to complete the exercise
    ChangeColor(u8, u8, u8),
    PowerOn,
    PowerOff,
    Disconnect,
}

enum LightStatus {
    Off,
    On,
}

fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    // Add code here to spawn a thread to control the light bulb
    let handle = thread::spawn(move || {
        let mut status = LightStatus::Off;
        loop {
            match receiver.recv() {
                Ok(msg) => match msg {
                    LightMsg::ChangeColor(r, g, b) => {
                        println!("Changing color to ({}, {}, {})", r, g, b);
                        println!("Light is currently {}", match &status {
                            LightStatus::Off => "off",
                            LightStatus::On => "on",
                        });
                    },
                    LightMsg::PowerOn => {
                        status = LightStatus::On;
                        println!("Powered on");
                    },
                    LightMsg::PowerOff => {
                        status = LightStatus::Off;
                        println!("Powered off");
                    },
                    LightMsg::Disconnect => {
                        println!("channel disconnected");
                        status = LightStatus::Off;
                        break;
                    },
                },
                Err(_) => {
                    println!("connection dropped");
                    status = LightStatus::Off;
                    break;
                }
            }
        }
        return status;
    });
    handle
}

fn main() {
    let (light_tx, light_rx) = unbounded();
    let light = spawn_light_thread(light_rx);
    light_tx.send(LightMsg::ChangeColor(0, 0, 100)).unwrap();
    light_tx.send(LightMsg::PowerOff).expect("Failed to power off");
    light_tx.send(LightMsg::Disconnect).expect("Failed to disconnect?!");
    light.join().expect("Something is amiss!");
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
