use std::collections::BTreeSet;

fn main() {
    let mut Set = BTreeSet::new();
    Set.insert(42);

    if Set.contains(&42) {
        println!("42 is in the set!");
    }
}
