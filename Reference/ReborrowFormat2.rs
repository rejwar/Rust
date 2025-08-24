fn main() {
    let mut Counter = 10;

    let R1 = &mut Counter;
    {
        let R2 = &mut R1;
        *R2 +=1;
    }

    *R1 +=1;
    println!("Counter is {}", Counter);
}