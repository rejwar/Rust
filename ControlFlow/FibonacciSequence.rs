fn Fibonacci (n:u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        Fibonacci(n-1) + Fibonacci(n-2)
    }
}

fn main() {
    println!("Fibonacci (5) is : {} ",Fibonacci(5));
}
