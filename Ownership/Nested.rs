// FileName: NestedMap.rs
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("Colors".to_string(), vec!["Red".to_string()]);
    println!("{:?}", map);
}
