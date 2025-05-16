fn main() {
    let A = 0b1100;
    let B = 0b1010;

    println!("AND : {:b}" , A & B);
    println!("OR : {:b}" , A | B );
    println!("XOR : {:b}" , A ^ B);
    println!("Not : {:b}" , !A);
    println!("Left Shift : {:b}" , A<<1);
}
