use std::collections::BTreeMap;

fn main() {
    let mut Map = BTreeMap::new();
    for i in (0..10).step_by(2) { Map.insert(i, i * 10); }

    let Range = Map.range(2..8);
    for (Key, Value) in Range {
        println!("{} -> {}", Key, Value);
    }
}
