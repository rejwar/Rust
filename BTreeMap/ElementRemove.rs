use std::collections::BTreeMap;

fn main() {
    let mut data = BTreeMap::new();
    data.insert(1, "Packet1");
    data.insert(2, "packet2");
    data.insert(3, "packet3");

    println!("Pop first {:?}", data.pop_first());
    println!("Pop last {:?}",data.pop_last());
    println!("Remaining {:?}", data);
}