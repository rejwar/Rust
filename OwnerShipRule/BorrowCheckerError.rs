fn main() {
    let mut number = 5;
    let ref1 = &number; // Immutable reference
    // let ref2 = &mut number; // Error: mutable reference while immutable exists
    println!("ref1: {}", ref1);
}
