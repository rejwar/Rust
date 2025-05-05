fn Fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    Fibonacci(n - 1) + Fibonacci(n - 2) // ✅ O(2ⁿ) Recursive Call
}

fn main() {
    println!("Fibonacci(5): {}", Fibonacci(5));
}
