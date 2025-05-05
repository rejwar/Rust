use std::collections::HashMap;

fn main() {
    let mut Data = HashMap::new();
    Data.insert("Alice", 30);
    Data.insert("Bob", 25);

    println!("Age of Alice: {}", Data.get("Alice").unwrap()); // ✅ O(1) Lookup
}
