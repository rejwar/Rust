fn main() {
    let owner = String::from("Rust");
    let reference = &owner; // Immutable borrow
    println!("Reference: {}", reference);
    println!("Owner: {}", owner); // Owner is still valid
}
