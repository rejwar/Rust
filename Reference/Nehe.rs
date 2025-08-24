fn main() {
    let mut Counter = 4;

    let R1 = &mut Counter;
    let R2 = &mut Counter;

    *R1 +=1;
    *R2 +=2;

    println!("The number is {}", Counter);
}