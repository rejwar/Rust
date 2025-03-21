enum Status {
    Active,
    Inactive,
    Pending,
}

fn main() {
    let status = Status::Pending;
    match status {
        Status::Active => println!("User is active."),
        Status::Inactive => println!("User is inactive."),
        Status::Pending => println!("User registration is pending."),
    }
}
