use std::collections::BTreeMap;

fn main() {
    let mut scores = BTreeMap::new();
    scores.insert("Alices", 85);
    scores.insert("Bob", 43);

    for (_name , score) in scores.iter_mut() {
        *score += 5;

    }

    println!("{:?}", scores);
}