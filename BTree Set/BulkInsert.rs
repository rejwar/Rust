use std::collections::BTreeSet;

fn main() {
    let mut Set = BTreeSet::new();
    Set.extend([1, 2, 3, 4, 5]);

    println!("Bulk Inserted Set: {:?}", Set);
}
