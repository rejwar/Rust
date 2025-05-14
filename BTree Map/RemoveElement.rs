use std::collections::BTreeMap;

fn main() {
    let mut Map = BTreeMap::new();
    Map.insert(5, "Five");
    Map.insert(10, "Ten");

    Map.remove(&5);
    println!("Updated Map: {:?}", Map);
}
