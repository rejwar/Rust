use std::collections::HashMap;

fn main() {
    let mut Scores = HashMap::new();
    Scores.insert("Alice", 10);
    Scores.insert("Bob", 15);
    println!("Scores: {:?}", Scores);
}
