fn main() {
    let mut value = String::from("Rust");
    let immutable_ref = &value; // Immutable reference
    println!("Immutable Reference: {}", immutable_ref);

    let mutable_ref = &mut value; // Mutable reference
    mutable_ref.push_str(" Programming");
    println!("Mutable Reference: {}", mutable_ref);
}
