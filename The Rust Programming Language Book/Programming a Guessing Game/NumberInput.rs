use std::io;

fn main() {
    println!("Enter a number:");

    let mut Input = String::new();
    io::stdin().read_line(&mut Input).expect("Failed to read line");

    let Number:i32= Input.trim().parse().expect("Please enter a valid number");
    println!("You entered: {}", Number);
}

