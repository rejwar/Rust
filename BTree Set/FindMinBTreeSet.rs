use std::collections::BTreeSet;

fn main() {
    let Set = BTreeSet::from([5, 15, 25, 35]);
    println!("Min Element: {:?}", Set.iter().next());
}
