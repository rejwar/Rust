struct  User {
    name : String,
    age : u32,
}

fn CreateUser() {
    let u = User {
        name : String::from("Alice"),
        age: 30,
    };

    println!(" is years old {} {}" , u.name , u.age);

}

fn main() {
    CreateUser();
}
