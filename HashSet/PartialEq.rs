use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn main() {
    let mut users = HashSet::new();
    users.insert(User {
        id: 1,
        name: String::from("Sumi"),
    });
    users.insert(User {
        id: 1,
        name: String::from("Sumi Akter"),
    });

    println!(" User count  {}", users.len());
}
