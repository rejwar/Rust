struct Counter {
    Value: i32,
}

fn main() {
    let mut MyCounter: Counter = Counter { Value: 0 };
    increment(&mut MyCounter);
    println!("Counter Value: {}", MyCounter.Value);
}

fn increment(Counter: &mut Counter) {
    Counter.Value += 1;
}
