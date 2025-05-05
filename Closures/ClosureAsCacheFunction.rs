use std::collections::HashMap;

fn main() {
    let mut Cache: HashMap<u32, u32> = HashMap::new();
    
    let Fibonacci = |n: u32| -> u32 {
        if n <= 1 {
            return n;
        }
        *Cache.entry(n).or_insert(Fibonacci(n - 1) + Fibonacci(n - 2))
    };

    println!("Fibonacci(10): {}", Fibonacci(10));
}
