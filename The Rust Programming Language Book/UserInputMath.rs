use std::io;

fn main() {
    println!("Enter two numbers:");

    let mut Input1 = String::new();
    let mut Input2 = String::new();

    io::stdin().read_line(&mut Input1).expect("Failed to read line");
    io::stdin().read_line(&mut Input2).expect("Failed to read line");

    let Num1: i32 = Input1.trim().parse().expect("Invalid number");
    let Num2: i32 = Input2.trim().parse().expect("Invalid number");

    println!("Sum: {}", Num1 + Num2);
}
