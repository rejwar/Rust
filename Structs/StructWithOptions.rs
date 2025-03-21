struct Account {
    username: String,
    email: Option<String>,
}

fn main() {
    let account = Account {
        username: String::from("User1"),
        email: None,
    };
    println!("Username: {}, Email: {:?}", account.username, account.email);
}
