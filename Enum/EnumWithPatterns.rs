enum Action {
    Insert(String),
    Update(u32),
    Delete,
}

fn main() {
    let action = Action::Update(42);
    match action {
        Action::Insert(data) => println!("Inserting: {}", data),
        Action::Update(id) => println!("Updating ID: {}", id),
        Action::Delete => println!("Deleting"),
    }
}
