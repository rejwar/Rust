use std::collections::BTreeSet;

fn main() {
    let mut Set = BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8]);
    Set.retain(|&x| x % 2 == 0); // Keep only even numbers

    println!("Updated Set: {:?}", Set);
}
