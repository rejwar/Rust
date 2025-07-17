use core::num;
use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number (1,2,3,4,5,6,7)");

    io::stdin().read_line(&mut input).expect("Failed to read the line");
    let number: i32  = input.trim().parse().unwrap_or(-1);

    match number {
        1 => println!("You entered one"),
        2 => println!("YOu entered two"),
        3 => println!("You entered three"),
        _=> println!("Invalid Number"),
    }
}