struct User{
    name: String,
    age: u32 ,
}

fn DestructStructBinding() {
    let user = User{
        name: "Alice".to_string(),
        age: 30,
    };

    let User {name, age} = user;
    println!("Name: {} , age ={}", name, age);

}

fn main() {
    DestructStructBinding();
}

