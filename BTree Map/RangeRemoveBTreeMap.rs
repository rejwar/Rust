use std::collections::BTreeMap;

fn main() {
    let mut Map = BTreeMap::new();
    for i in 0..10 {
        Map.insert(i, i * 100);
    }

    let KeysToRemove: Vec<i32> = Map.range(3..7).map(|(K, _)| *K).collect();
    for Key in KeysToRemove {
        Map.remove(&Key);
    }

    println!("Updated Map: {:?}", Map);
}
