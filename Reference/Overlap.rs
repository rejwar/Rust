fn main() {
    let mut Counter =0;

    let R1 = &mut Counter;
    let R2 = &mut Counter;

    *R1 +=1;
    *R2 +=2;

    println!(" {}", Counter);
}