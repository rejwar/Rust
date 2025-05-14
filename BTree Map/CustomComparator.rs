use std::collections::BTreeMap;
use std::cmp::Reverse;

fn main() {
    let mut Map: BTreeMap<Reverse<i32>, &str> = BTreeMap::new();
    
    Map.insert(Reverse(1), "Lowest Priority");
    Map.insert(Reverse(10), "Highest Priority");
    Map.insert(Reverse(5), "Medium Priority");

    for (Key, Value) in &Map {
        println!("{:?} -> {}", Key, Value);
    }
}
