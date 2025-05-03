use core::str;

pub struct User {
    pub Name : String,
    pub Age: u32,
}

fn main() {
    let Person = User { Name: String::from("Alice") , Age: 30};
    println!("User {} , Age : {}" , Person.Name , Person.Age);
}
