fn main() {
    let IsLoggedIn: bool = true;
    let IsAdmin: bool = true;

    if IsLoggedIn {
        if IsAdmin {
            println!("Welcome Admin!");
        } else {
            println!("Welcome User!");
        }
    } else {
        println!("Please Log In!");
    }
}
