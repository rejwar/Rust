fn main() {
    let mut Counter = 0;

    let R1 = &mut Counter;
    *R1 +=2;
    
    let R2 = &mut Counter;
    *R2 += 3;

    println!("Counter -> {}", Counter);
}