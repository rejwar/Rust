use std::collections::HashMap;

fn main {
    let mut map = HashMap::new();
    map.insert(1,  String::from("Data"));

    if let Some(val) = map.get(&1) {
        println!("Borrowed value {}" ,val);
    }
}
