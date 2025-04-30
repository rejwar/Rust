fn ModifyValue(mut Number: i32 ) {
    Number += 10;
    println!("Inside Function {}", Number);
}

fn main() {
    let Numebr: i32 = 20;
    ModifyValue(Numebr);
    println!("Outside Fumction {}",Number);
}
