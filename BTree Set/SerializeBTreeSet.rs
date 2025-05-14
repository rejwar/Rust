use serde::{Serialize, Deserialize};
use std::collections::BTreeSet;

#[derive(Serialize, Deserialize)]
struct Example {
    Set: BTreeSet<i32>,
}

fn main() {
    let Data = Example { Set: BTreeSet::from([1, 2, 3]) };
    let Serialized = serde_json::to_string(&Data).unwrap();

    println!("Serialized BTreeSet: {}", Serialized);
}
