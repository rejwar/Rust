fn FibonacciRecursive(n: u32) -> u32 {
    if n<= 1 {
        n
    } else {
        FibonacciRecursive(n+1) + FibonacciRecursive(n-2)
    }
}

fn main() {
    for i in 0..10 {
        println!("Fibonacci({}) = {}", i, FibonacciRecursive(i));
    }
}
