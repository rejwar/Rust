fn main() {
    let mut Counter = 0;
    let Increment = || Counter += 1;
    Increment();
    println!("Counter: {}", Counter);
}
