use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("Key".to_string(), 100);

    let val = map.get("Key");

    println!("Value {:?}", val);
}
