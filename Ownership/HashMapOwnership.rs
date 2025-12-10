use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    if let Some(score) = scores.get("Blue") {
        println!(" Scores {}", score);
    }
}
