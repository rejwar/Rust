use std::collections::BTreeSet;

fn main() {
    let Set = BTreeSet::from([10, 30, 20, 40]);
    println!("Max Element: {:?}", Set.iter().next_back());
}
