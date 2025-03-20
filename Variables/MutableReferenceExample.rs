fn main() {
    let mut Counter = 10;
    {
        let CounterRef: &mut i32 = &mut Counter;
        *CounterRef += 5;
    }
    println!("Counter: {}", Counter);
}
