use std::io;

fn main() {
    println!("Enter a number:");

    let mut Input = String::new();
    io::stdin().read_line(&mut Input).expect("Failed to read input");

    let Num: i32 = Input.trim().parse().expect("Invalid number");

    if Num % 2 == 0 {
        println!("{} is even", Num);
    } else {
        println!("{} is odd", Num);
    }
}
