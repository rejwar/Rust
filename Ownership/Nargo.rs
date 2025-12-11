struct User {
    name: String,
}
fn main() {
    let user = creat_user();
    println!("User {}", user.name);
}

fn creat_user() -> User {
    User {
        name: String::from("Nargo"),
    }
}
