use std::collections::BTreeMap;

fn main() {
    let mut Map = BTreeMap::new();
    
    Map.entry(1).or_insert("First Entry");
    Map.entry(2).or_insert("Second Entry");
    Map.entry(1).or_insert("Won't Override");

    println!("Final Map: {:?}", Map);
}
