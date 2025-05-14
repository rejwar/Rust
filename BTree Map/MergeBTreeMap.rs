use std::collections::BTreeMap;

fn main() {
    let mut Map1 = BTreeMap::new();
    Map1.insert(1, "One");
    Map1.insert(2, "Two");

    let mut Map2 = BTreeMap::new();
    Map2.insert(3, "Three");
    Map2.insert(4, "Four");

    Map1.append(&mut Map2);
    
    println!("Merged Map: {:?}", Map1);
}
