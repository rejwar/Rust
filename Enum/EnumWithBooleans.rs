enum LoginStatus {
    Success(bool),
    Failure(String),
}

fn main() {
    let status = LoginStatus::Failure(String::from("Invalid password"));
    match status {
        LoginStatus::Success(true) => println!("Login successful"),
        LoginStatus::Success(false) => println!("Login failed"),
        LoginStatus::Failure(error) => println!("Error: {}", error),
    }
}
