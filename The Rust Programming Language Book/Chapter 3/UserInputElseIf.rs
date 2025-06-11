use std::io;

fn main() {
    println!("Enter a number:");

    let mut Input = String::new();
    io::stdin().read_line(&mut Input).expect("Failed to read input");

    let Number: i32 = Input.trim().parse().expect("Invalid number");

    if Number < 0 {
        println!("Negative number");
    } else if Number == 0 {
        println!("Zero");
    } else {
        println!("Positive number");
    }
}
