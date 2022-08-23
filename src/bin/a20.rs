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
use std::io::Write;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(buf: &str) -> Result<PowerState, String> {
        let lower = buf.to_lowercase();
        match lower.as_str() {
            "off" => Ok(PowerState::Off),
            "sleep" => Ok(PowerState::Sleep),
            "reboot" => Ok(PowerState::Reboot),
            "shutdown" => Ok(PowerState::Shutdown),
            "hibernate" => Ok(PowerState::Hibernate),
            _ => Err("bad action".to_owned()),
        }
    }
}

fn get_input() -> Result<String, String> {
    let mut in_buffer = String::new();
    if !io::stdin().read_line(&mut in_buffer).is_ok() {
        return Err("can't get input".to_owned());
    }
    Ok(in_buffer.trim().to_owned())
}

fn to_action_message(state: PowerState) -> Result<String, String> {
    match state {
        PowerState::Off => Ok("turning off".to_owned()),
        PowerState::Sleep => Ok("sleeping".to_owned()),
        PowerState::Reboot => Ok("rebooting in moments".to_owned()),
        PowerState::Shutdown => Ok("shutting down".to_owned()),
        PowerState::Hibernate => Ok("entering hibernation".to_owned()),
    }
}

fn main_loop() -> Result<(), String> {
    let input = get_input();
    match input {
        Ok(dat) => {
            let state = PowerState::new(&dat)?;
            let action = to_action_message(state)?;
            println!("{:?}", action);
        }
        Err(msg) => return Err(msg),
    }
    Ok(())
}

fn main() {
    print!("Power> ");
    io::stdout().flush().unwrap();
    match main_loop() {
        Err(msg) => println!("error: {:?}", msg),
        Ok(_) => (),
    }
}
