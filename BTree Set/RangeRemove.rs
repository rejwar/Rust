use std::collections::BTreeSet;

fn main() {
    let mut Set = BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8]);

    let KeysToRemove: Vec<i32> = Set.range(3..7).map(|&K| K).collect();
    for Key in KeysToRemove {
        Set.remove(&Key);
    }

    println!("Updated Set: {:?}", Set);
}
