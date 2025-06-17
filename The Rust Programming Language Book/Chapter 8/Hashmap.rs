// 

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);

    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }
}
