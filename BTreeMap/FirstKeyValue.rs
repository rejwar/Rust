use std::collections::BTreeMap;

fn main() {
    let mut scores = BTreeMap::new();
    scores.insert("Alice ", 85);
    scores.insert("Bob", 92);
    scores.insert("Charlie", 78);


    println!("First {:?}", scores.first_key_value());
    println!("Last {:?}", scores.last_key_value());
}