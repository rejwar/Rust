struct Counter {
    value: u32,
}

fn main() {
    let mut counter = Counter { value: 0 };
    counter.value += 1;
    println!("Counter: {}", counter.value);
}
