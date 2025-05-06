fn main() {
    let Name = String::from("Rust");
    let NewName = Name; // ✅ Ownership moved

    println!("New Name: {}", NewName);
    // println!("Old Name: {}", Name); ❌ Error! Name moved.
}
