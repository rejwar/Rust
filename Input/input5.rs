use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your age");

    io::stdin().read_line(&mut input).expect("Please enter a valid number ");

    let age: u8 = input.trim().parse().expect("please enter a valid number");

    if age >= 18 {
        println!("You are an adult ");
    } else {
        println!("You are a minor");
    }
    
}