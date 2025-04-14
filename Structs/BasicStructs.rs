struct Person{
    name: String,
    age: u8,


}

fn CreatePerson() {
    let user = Person{
        name: String::from("Alice"),
        age: 30,
    };

    println!("{} is {} years old." , user.name, user.age);
}

fn main() {
    CreatePerson();
}
