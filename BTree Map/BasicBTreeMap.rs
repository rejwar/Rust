use std::collections::BTreeMap;

fn main() {
    let mut Map = BTreeMap::new();

    Map.insert(3, "Alice");
    Map.insert(1, "Charlie");
    Map.insert(2, "Bob");

    println!("Sorted Map: {:?}", Map);
}
