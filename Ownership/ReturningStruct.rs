struct User {
    name: String,
    age: u32,
}

fn get_user() -> User {
    User {
        name: String::from("Alice"),
        age: 33,
    }
}

fn main() {
    let user_info = get_user();
    println!("Name: {}, Age: {}", user_info.name, user_info.age);
}
