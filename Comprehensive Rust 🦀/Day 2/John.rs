struct User {
    Name: String,
    Age: u32,
}

fn GetUserNameById(Id: u32) -> Option<User> {
    match Id {
        1 => Some(User {
            Name: "John" .to_string(),
            Age: 30,
        }),

        2 => Some(User{
            Name: "Alice".to_string(),
            Age: 25,
        }),
        _=> None
    }
}

fn main() {
    match GetUserNameById(1) {
        Some(User) => println!("Found user : {}", User.Name),
        None => println!("User not found"),
    }
}
