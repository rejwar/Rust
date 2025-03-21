struct Counter {
    count: u32,
}

impl Counter {
    fn reset(self) -> Counter {
        Counter { count: 0 }
    }
}

fn main() {
    let counter = Counter { count: 100 };
    let reset_counter = counter.reset();
    println!("Reset Count: {}", reset_counter.count);
}
