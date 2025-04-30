fn ModifyValue(mut Number: i32 ) {
    Number +=10;
    println!("Inside Function {}", Number);
}

fn main() {
    let Number: i32 = 20;
    ModifyValue(Number);
    println!("Outside Function {}", Number); 
}
