use std::collections::BTreeSet;

fn main() {
    let mut Set = BTreeSet::new();

    Set.insert(10);
    Set.insert(5);
    Set.insert(20);

    println!("Sorted Set: {:?}", Set);
}
