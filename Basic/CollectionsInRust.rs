use std::collections::HashMap;

fn main() {
    // Vector (dynamic array)
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("Vector: {:?}", numbers);

    // HashMap (key-value store)
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 200);
    println!("HashMap: {:?}", scores);

    // String (growable UTF-8 string)
    let mut message = String::from("Hello");
    message.push_str(", Rust!");
    println!("String: {}", message);
}
