use std::collections::BTreeSet;

fn main() {
    let Set = BTreeSet::from([10, 20, 30, 40, 50]);

    let Middle = Set.iter().nth(Set.len() / 2);
    println!("Middle Element: {:?}", Middle);
}
