fn ValidateUser(Name: &str, IsAdmin: bool, IsVerified: bool) -> bool {
    Name != "" && IsAdmin && IsVerified // All conditions must be true
}

fn main() {
    let UserName: &str = "Alice";
    let IsAdmin: bool = true;
    let IsVerified: bool = false;

    let IsValidUser: bool = ValidateUser(UserName, IsAdmin, IsVerified);
    println!("Is Valid User: {}", IsValidUser);
}
