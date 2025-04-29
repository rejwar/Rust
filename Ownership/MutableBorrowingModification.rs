fn main() {
    let mut Number: i32 = 10;
    let RefNumber: &mut i32 = &mut Number;
    *RefNumber += 5;
    println!("Updated Number: {}", Number);
}
