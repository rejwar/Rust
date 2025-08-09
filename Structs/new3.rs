struct User {
    uername: String,
    email: String,
    active: true,
}

fn PrintUserDefined(user: &User) {
    println!("username: {}", user.username);
    println!("email: {}", user.email);
    println!("active {}", user.active);
}

fn main() {
    let alice = User {
        username: String::from("alice"),
        email: String::from("alice@gmail.com"),
        active: true,
    };

    PrintUserDefined(&alice);
}