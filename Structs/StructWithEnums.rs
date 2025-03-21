enum Status {
    Active,
    Inactive,
}

struct User {
    name: String,
    status: Status,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        status: Status::Active,
    };
    println!("User: {}, Status: Active", user.name);
}
