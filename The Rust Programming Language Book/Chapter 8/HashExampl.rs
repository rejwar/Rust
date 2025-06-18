use std::collections::{HashMap, HashSet};

fn HashSetExample() {
    let mut set = HashSet::new();

    set.insert("Apple");
    set.insert("Banana");
    set.insert("Cherry");
    set.insert("Apple"); // Duplicate, will not be added
    set.insert("Date");
    set.insert("Elderberry");
    set.insert("Fig");
    set.insert("Grape");
    set.insert("Honeydew");
    
    for item in &set {
        println!("{}", item);
    }
    println!("Total unique items: {}", set.len());
}

fn main() {
    HashSetExample();
}
