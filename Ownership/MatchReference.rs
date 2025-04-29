fn main() {
    let Status: &str = "Active";

    match Status {
        "Active" => println!("User is Active"),
        "Inactive" => println!("User is Inactive"),
        _ => println!("Unknown Status"),
    }
}
