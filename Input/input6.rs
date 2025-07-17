use std::io; 

fn main() {
    let mut input = String::new();

    println!("Enter your choice (start / stop / quit)");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();

    match input {
        "Start" => println!("Starting the process..."),
        "stop" => println!("Stoping the process"),
        "quit" => println!("Quitting the program"),
        _=> println!("Invalid input"),
    }
}