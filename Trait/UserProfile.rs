#[derive(Debug, Clone)]

struct UserProfile {
    id: i32,
    userName: String,
}

fn main() {
    let user_a = UserProfile {
        id: 101,
        userName: String::from("RustLearner"),
    };

    let user_b = user_a.clone();

    println!("User A {:?}", user_a);
    println!(" User B {:?}", user_b);
}
