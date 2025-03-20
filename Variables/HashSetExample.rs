use std::collections::HashSet;

fn main() {
    let mut UniqueNumbers = HashSet::new();
    UniqueNumbers.insert(1);
    UniqueNumbers.insert(2);
    UniqueNumbers.insert(1); // Duplicate won't be added
    println!("UniqueNumbers: {:?}", UniqueNumbers);
}
