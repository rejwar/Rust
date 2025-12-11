struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("user1"),
        email: String::from("U1@gmail.com"),
        active: true,
    };

    let user2 = User {
        username: String::from("user2"),
        ..user1
    };

    println!(" User 1 active {}", user1.active);
}
