use std::collections::BTreeSet;

fn main() {
    let SetA = BTreeSet::from([1, 2, 3, 4]);
    let SetB = BTreeSet::from([3, 4, 5, 6]);

    let Difference = SetA.symmetric_difference(&SetB);
    println!("Symmetric Difference: {:?}", Difference.collect::<Vec<&i32>>());
}
