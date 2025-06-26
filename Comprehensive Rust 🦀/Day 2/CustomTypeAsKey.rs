// Question: How do you use a struct as a HashMap key?

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut map = HashMap::new();
    map.insert(Point { x: 1, y: 2 }, "A");
    map.insert(Point { x: 3, y: 4 }, "B");

    println!("{:?}", map);
}
