use std::io;

fn main() {
    println!("Enter two Numbers :");

    let mut input  = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    let num1: i32 = input.trim().parse().expect("Please type a number!");
    let num2: i32 = input2.trim().parse().expect("Please type a number!");
    
    let sum = num1 + num2;
    println!("The sum of {} and {} is: {}", num1, num2, sum);
}
