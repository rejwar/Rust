// FileName: ReturnMap.rs
use std::collections::HashMap;
fn get_map() -> HashMap<String, i32> {
    HashMap::new()
}
fn main() {
    let mut m = get_map();
    m.insert("Key".into(), 1);
}
