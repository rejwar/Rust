use std::collections::BTreeSet;

fn main() {
    let mut SetA = BTreeSet::from([1, 2, 3, 4]);
    let mut SetB = BTreeSet::from([3, 4, 5, 6]);

    println!("Union: {:?}", SetA.union(&SetB).collect::<Vec<&i32>>());
    println!("Intersection: {:?}", SetA.intersection(&SetB).collect::<Vec<&i32>>());
    println!("Difference: {:?}", SetA.difference(&SetB).collect::<Vec<&i32>>());
}
