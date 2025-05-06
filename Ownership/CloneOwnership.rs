fn main() {
    let Name = String::from("Rust");
    let NewName = Name.clone(); // ✅ Clone ownership

    println!("Old Name: {}", Name);
    println!("New Name: {}", NewName);
}
