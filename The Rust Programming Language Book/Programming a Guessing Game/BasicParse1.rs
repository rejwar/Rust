use std::io;

fn main() {
    println!("Enter a number:");

    let mut Input = String::new();
    io::stdin().read_line(&mut Input).expect("Failed to read input");

    match Input.trim().parse::<i32>() {
        Ok(Number) => println!("Valid input: {}", Number),
        Err(_) => println!("Error: Invalid number format!"),
    }
}
