fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    println!("Fibonacci of 10: {}", fib(10));
    println!("Fibonacci of 20: {}", fib(20));
}
