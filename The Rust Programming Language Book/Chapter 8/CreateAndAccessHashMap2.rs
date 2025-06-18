// 

use std::collections::HashMap;

fn CreateAndAccessHashMap() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 75);

    if let Some(score) = scores.get("Bob") {
        println!("Bob's score is: {}", score);
    } else {
        println!("Bob's score not found.");
    }
}
