use std::collections::HashSet;

fn main() {
    let mut fruits = HashSet::new();
    fruits.insert("apple");
    fruits.insert("banana");
    fruits.insert("cherry");
    fruits.insert("date");
    fruits.insert("elderberry");
    
    println!("Fruits in the set: {:?}" , fruits);
}
