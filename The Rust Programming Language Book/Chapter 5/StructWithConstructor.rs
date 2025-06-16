// How to use associated functions (like constructors) with struct?

struct User {
    username: String,
    email: String,
}

impl User {
    // Associated function (not a method because it doesn't take &self)
    fn new(username: &str, email: &str) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
        }
    }
}

fn CreateUserInstance() {
    let user = User::new("joy", "joy@example.com");
    println!("Username: {}, Email: {}", user.username, user.email);
}
