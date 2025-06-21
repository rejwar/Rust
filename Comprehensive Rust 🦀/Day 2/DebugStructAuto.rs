#[derive(Debug)]

struct  UserProfile {
    username: String,
    age: u8,
}

fn main() {
    let user = UserProfile {
        username: String::from("MD"),
        age:25,

    };
    println!("{:?}", user);
    dbg!(&user);
}
