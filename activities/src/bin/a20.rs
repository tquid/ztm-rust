// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn power_message(state: PowerState) {
    match state {
        PowerState::Off => println!("Hard power off!"),
        PowerState::Sleep => println!("Entering sleep"),
        PowerState::Reboot => println!("Rebooting"),
        PowerState::Shutdown => println!("Shutting down"),
        PowerState::Hibernate => println!("Entering hibernation"),
    }
}
fn main() {
    let mut buffer = String::new();
    let command = match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer,
        Err(e) => panic!("Error trying to read input: {:?}", e),
    };
    match command.to_lowercase().trim() {
        "off" => power_message(PowerState::Off),
        "sleep" => power_message(PowerState::Sleep),
        "reboot" => power_message(PowerState::Reboot),
        "shutdown" => power_message(PowerState::Shutdown),
        "hibernate" => power_message(PowerState::Hibernate),
        unknown_command => println!("Unknown command {:?}", unknown_command),
    }
}
