use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 23);
    scores.insert("Bob", 96);

    if let Some(score) = scores.get("Alice"){
        println!("Alice's score {}", score);
    }

    println!("{:?}",scores);
}
