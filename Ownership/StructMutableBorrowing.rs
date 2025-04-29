struct Counter {
    Value: i32,
}

fn IncrementValue(Counter: &mut Counter) {
    Counter.Value += 1;
}

fn main() {
    let mut MyCounter: Counter = Counter { Value: 0 };
    IncrementValue(&mut MyCounter);
    println!("Updated Counter: {}", MyCounter.Value);
}
