use std::collections::BTreeSet;
use rayon::prelude::*;

fn main() {
    let Set = BTreeSet::from([10, 20, 30, 40, 50]);

    Set.par_iter().for_each(|Value| {
        println!("Processing: {}", Value);
    });
}
