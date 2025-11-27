struct Counter {
    value: i32,
}

impl Counter {
    fn New() -> Self {
        Self { value: 0 }
    }

    fn inc(&mut self) {
        self.value += 1;
    }

    fn reset(&mut self) {
        self.value = 0;
    }

    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    let mut c = Counter::New();

    c.inc();
    c.inc();

    println!("After 2 inch {}", c.get());
    c.reset();
    println!("After reset is {}", c.get());
}
