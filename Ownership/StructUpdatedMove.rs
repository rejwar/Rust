struct User {
    email: String,
    username: String,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("a&v.com"),
        username: String::from("uer1"),
        active: true,
    };

    let user2 = User {
        email: String::from("X@y.com"),
        ..user1
    };
}
