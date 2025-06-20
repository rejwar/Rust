use std::io;

fn fib(n: u32 ) -> u32 {
    if n <= 1 {n}  else {fib(n-1) + fib(n-2)}
}

fn main() {
    println!("Enter a number :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Please enter a valid number");
    let result = fib(n);
    println!("Fibonacci of {} is {}", n, result);
    println!("Fibonacci of {} is {}", n, fib(n));

}
