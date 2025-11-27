struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn inc(&mut self) {
        self.value += 1;
    }

    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    let mut c = Counter::new();
    c.inc();
    c.inc();
    println!("Counter = {}", c.get());
}
