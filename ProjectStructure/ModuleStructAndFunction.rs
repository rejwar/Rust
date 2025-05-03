mod user {
    pub struct  User {
        pub Name: String,
        pub Age: u32,
    }

    pub fn CreateUser() -> User {
        User {
            Name: String::from("Alice"),
            Age: 30,
        }
    }
}

fn main() {
    let NewUser = user::CreateUser();
    println!("User : {} , Age{} " , NewUser.Name , NewUser.Age);
}
