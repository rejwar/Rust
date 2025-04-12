enum Status {
    Active(bool),
    Inactive(bool),
}

fn main() {
    let UserStatus: Status = Status::Active(true);

    match UserStatus {
        Status::Active(IsActive) if IsActive => println!("The user is active."),
        Status::Inactive(IsInactive) if !IsInactive => println!("The user is inactive."),
        _ => println!("Invalid status."),
    }
}
