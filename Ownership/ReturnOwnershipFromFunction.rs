fn GiveOwnership() -> String {
    String::from("Rust")
}

fn main() {
    let Name = GiveOwnership(); // ✅ Ownership transferred
    println!("Name: {}", Name);
}
