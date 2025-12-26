struct Person {
    name: String,
    age: u8,
    is_active: bool,
}

fn main() {
    let user1 = Person {
        name: String::from("Rifat"),
        age: 30,
        is_active: true,
    };
    println!("Name: {}", user1.name);
}