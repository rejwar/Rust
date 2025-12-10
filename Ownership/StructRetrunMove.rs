struct User {
    name: String,
}

fn main() {
    let user = create_user();
    println!("User is {}", user.name);
}

fn create_user() -> User {
    User {
        name: String::from("Rifat"),
    }
}
