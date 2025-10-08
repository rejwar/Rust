use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let red_scores = scores.get("Red");
    if let Some(&score) = red_scores {
        println!("Red has {}", score);

    }

    for (team , score) in &scores {
            println!(" team score : {} {}",team, scores);
    }
}