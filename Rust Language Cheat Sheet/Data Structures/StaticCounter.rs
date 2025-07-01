// Question: How do you define a static variable in Rust?

struct Counter {
    value: i32,
}

// We need to use `const fn` to allow initializing in static
impl Counter {
    const fn New() -> Self {
        Self { value: 0 }
    }
}

static COUNTER: Counter = Counter::New();

fn main() {
    println!("Initial value: {}", COUNTER.value); // prints 0
}
