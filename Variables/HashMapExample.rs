use std::collections::HashMap;

fn main() {
    let mut NameToAge: HashMap<&str, u32> = HashMap::new();
    NameToAge.insert("Alice", 30);
    NameToAge.insert("Bob", 25);
    println!("NameToAge: {:?}", NameToAge);
}
