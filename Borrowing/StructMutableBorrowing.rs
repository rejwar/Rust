struct Counter {
    Value: i32,
}

fn IncrementCounter(Counter: &mut Counter) {
    Counter.Value += 1;
}

fn main() {
    let MutCounter: Counter = Counter { Value: 0 };
    IncrementCounter(&mut MutCounter);
    println!("Counter Value: {}", MutCounter.Value);
}
