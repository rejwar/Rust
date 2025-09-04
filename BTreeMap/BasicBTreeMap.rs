use std::collections::BTreeMap;

fn main() {
    let mut scores: BTreeMap<&str, i32> = BTreeMap::new();
    scores.insert("Alice ", 85);
    scores.insert("Bob", 90);
    scores.insert("Dubly",  80);
    scores.insert("David ", 95);
    scores.insert("Eve", 40);


    println!("All scores {:?}", scores);

    for(name ,score) in scores.range("Bob".."Eve"){
        println!("{} -> {}", name, score);
    }
}