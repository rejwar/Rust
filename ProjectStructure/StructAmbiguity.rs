mod models {
    pub struct User {
        pub Name: String,
    }
}

mod entities {
    pub struct User {
        pub ID: u32,
    }
}

fn main() {
    let Person = models::User { Name: String::from("Alice") };
    let Account = entities::User { ID: 12345 };

    println!("User: {}", Person.Name);
    println!("Account ID: {}", Account.ID);
}
