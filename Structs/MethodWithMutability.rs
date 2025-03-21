struct Counter {
    count: u32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    counter.increment();
    println!("Count: {}", counter.count);
}
