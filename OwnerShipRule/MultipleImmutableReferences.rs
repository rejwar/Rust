fn main() {
    let number = 42;
    let ref1 = &number; // First immutable reference
    let ref2 = &number; // Second immutable reference
    println!("ref1: {}, ref2: {}", ref1, ref2);
}
