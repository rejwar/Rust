fn main() {
    let mut Count = 0;
    let mut Increment = || { Count += 1; println!("Counter: {}", Count); };

    Increment();
    Increment();
}
