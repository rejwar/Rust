use std::io;

fn main() {
    println!("Enter a Number ");

    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let Number:i32 = input.trim().parse().expect("Please type a number!");
    println!("You entered: {}", Number);
    
}
