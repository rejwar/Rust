struct Counter {
    count: u32,
}

fn main() {
    let mut counter = Counter { count: 0 };
    let reference = &mut counter;
    reference.count += 1;
    println!("Count: {}", reference.count);
}
