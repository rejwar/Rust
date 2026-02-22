// use std::collections::HashSet;

fn main() {
    let set_a: HashSet<_> = ["Apple", "Banana", "Mango"].into_iter().collect();

    let set_b: HashSet<_> = ["Mango", "Orange", "Grapes"].into_iter().collect();

    let common_fruits: HashSet<_> = set_a.intersection(&set_b).collect();
    println!(" Insertion {:?}", common_fruits);

    let all_fruits: HashSet<_> = set_a.union(&set_b).collect();
    println!(" Union {:?}", all_fruits);

    let unique_to_a: HashSet<_> = set_a.difference(&set_b).collect();
    println!(" Difference (A-B) : {:?}", unique_to_a);
}

use std::collections::HashSet;
