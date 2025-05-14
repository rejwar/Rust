use std::collections::BTreeSet;

fn main() {
    let SetA = BTreeSet::from([1, 2, 3]);
    let SetB = BTreeSet::from([1, 2, 3, 4, 5]);

    println!("Is Subset? {}", SetA.is_subset(&SetB));
}
