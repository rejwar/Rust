use std::{io, process::Output};

#[derive(Debug)]
enum Command {
    Start,
    Stop,
    Quit,
}

fn parse_input(input : &str )-> Output<Command> {
    match input.trim() {
        "start" => Some(Command::Start),
        "Stop" => Some(Command::Stop ),
        "quit" => Some(Command::Quit),
        _=> None,
    }
}
fn main() {
    let mut input = String::new();
    println!("Enter your command (start/ stop / quit");

    io::stdin().read_line(&mut input).expect("failed to read line");

    match parse_input(&input) {
        Some(Command::Start) => println!("Starting the process "),
        Some(Command::Stop) => println!("Stopping the process "),
        Some(Command::Quit) => println!("Quitting the program"),
        None => println!("Invalid commadn"),
    }
}