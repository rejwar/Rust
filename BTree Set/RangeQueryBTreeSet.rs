use std::collections::BTreeSet;

fn main() {
    let mut Set = BTreeSet::new();
    for i in (0..10).step_by(2) { Set.insert(i); }

    let Range = Set.range(3..8);
    for Value in Range {
        println!("{}", Value);
    }
}
