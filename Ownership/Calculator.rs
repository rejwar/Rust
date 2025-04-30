use std::io;

fn main() {
    println!("Welcome to the Rust CLI Calculator!");

    let mut input = String::new();
    println!("Enter first number: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num1: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    println!("Enter second number: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num2: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    println!("Choose operation (+, -, *, /): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation = input.trim();

    let result = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Invalid operation. Please enter +, -, *, or /.");
            return;
        }
    };

    println!("Result: {}", result);
}

