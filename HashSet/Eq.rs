use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]

struct User {
    id: u32,
    name: String,
}

fn main() {
    let mut users = HashSet::new();

    let user1 = User {
        id: 1,
        name: String::from("Sumi"),
    };
    let user2 = User {
        id: 2,
        name: String::from("Rahim"),
    };
    let user3 = User {
        id: 3,
        name: String::from("Sumi"),
    };

    users.insert(user1);
    users.insert(user2);
    users.insert(user3);

    println!(" Total user in the set {}", users.len());
}
