fn IsValidPassword(Password: &str) -> bool {
    Password.len() >= 8 // Checks password length
}

fn main() {
    let UserPassword: &str = "rust123";
    let IsValid: bool = IsValidPassword(UserPassword);
    println!("Is Password Valid? {}", IsValid);
}
