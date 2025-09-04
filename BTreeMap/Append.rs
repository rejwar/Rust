use std::collections::BTreeMap;

fn main() {
    let mut map1: BTreeMap<i32 , &str> = BTreeMap::new();

    map1.insert(30, "Sajib");
    map1.insert(40, "Kader");
    map1.insert(60, "kala");

    let mut map2: BTreeMap<i32 , &str> = BTreeMap::new();

    map2.insert(71, "Cih");
    map2.insert(72,"Kullu" );
    map2.insert(73, "value");

    map1.append(&mut map2);

    println!("Marged is {:?}",map1);
}