fn main() {
    let mut number = 10;
    let reference = &mut number; // Mutable reference
    *reference += 5;
    println!("Modified Number: {}", number);
}
