use std::collections::BTreeMap;

fn main() {
    let mut Map = BTreeMap::new();
    Map.insert(10, "Ten");
    Map.insert(20, "Twenty");
    Map.insert(30, "Thirty");

    for (Key, Value) in &Map {
        println!("{} -> {}", Key, Value);
    }
}
