use std::collections::HashMap;

fn Fibonacci(n: u32, Memo: &mut HashMap<u32, u32>) -> u32 {
    if n <= 1 {
        return n;
    }
    if let Some(&Value) = Memo.get(&n) {
        return Value;
    }
    let Result = Fibonacci(n - 1, Memo) + Fibonacci(n - 2, Memo);
    Memo.insert(n, Result);
    Result
}

fn main() {
    let mut Memo = HashMap::new();
    println!("Fibonacci(10): {}", Fibonacci(10, &mut Memo)); // ✅ Memoization Applied
}
