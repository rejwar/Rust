struct  User {
    Name: String,
    Age: u32,
}

fn main() {
    let UserInfo: User = User{
        Name: String::from("Alice"),
        Age: 30,
    };
    println!("Name {} Age {}" , UserInfo.Name , UserInfo.Age);
}
