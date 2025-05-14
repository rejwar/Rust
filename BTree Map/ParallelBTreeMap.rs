use std::collections::BTreeMap;
use rayon::prelude::*;

fn main() {
    let mut Map = BTreeMap::new();
    for i in 0..10 {
        Map.insert(i, i * 2);
    }

    Map.par_iter().for_each(|(Key, Value)| {
        println!("Processing: {} -> {}", Key, Value);
    });
}
