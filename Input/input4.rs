use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a floating point number");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: f64 = input.trim().parse().expect("Please enter a valid number ");

    println!("You entered {}", number);    
}