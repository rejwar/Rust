// Question: How can you create an infinite Fibonacci iterator in Rust?

struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        let result = self.curr;
        self.curr = self.next;
        self.next = new_next;

        Some(result)
    }
}

fn FibonacciGenerator() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    for (i, val) in FibonacciGenerator().take(10).enumerate() {
        println!("fib({}) = {}", i, val);
    }
}
