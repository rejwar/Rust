use std::collections::BTreeSet;

fn main() {
    let mut Set = BTreeSet::new();
    Set.insert(100);
    Set.insert(50);
    Set.insert(75);

    for Value in &Set {
        println!("{}", Value);
    }
}
