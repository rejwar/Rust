struct User {
    username: String,
    email: String,
}

impl User {
    fn new(username: &str, email: &str) -> User {
        User {
            username: username.to_string(),
            email: email.to_string(),
        }
    }
}

fn main() {
    let user = User::new("Alice", "alice@example.com");
    println!("Username: {}, Email: {}", user.username, user.email);
}
