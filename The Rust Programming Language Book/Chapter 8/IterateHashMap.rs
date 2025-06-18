use std::collections::HashMap;

fn IterateHashMap() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 80);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}
