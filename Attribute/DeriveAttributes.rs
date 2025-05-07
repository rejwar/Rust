#[derive(Debug, Clone, PartialEq)]
struct User {
    Name: String,
    Age: u32,
}

fn main() {
    let User1 = User { Name: String::from("Alice"), Age: 25 };
    let User2 = User1.clone();

    println!("{:?}", User1);
    println!("User1 == User2: {}", User1 == User2);
}
