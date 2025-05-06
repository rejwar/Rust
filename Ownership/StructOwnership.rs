struct User {
    Name: String,
}

fn main() {
    let User1 = User { Name: String::from("Alice") };
    println!("User Name: {}", User1.Name);
}
