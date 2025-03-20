fn main() {
    let mut value = String::from("Ownership Rules");
    let immutable_ref = &value; // Immutable borrow
    println!("Immutable Reference: {}", immutable_ref);

    let mutable_ref = &mut value; // Mutable borrow
    mutable_ref.push_str(" - Updated!");
    println!("Mutable Reference: {}", mutable_ref);

    // Ownership transferred
    let new_owner = value; // Move
    println!("New Owner: {}", new_owner);
}
