struct Person{
    name: String,
    age: u32,
}

fn BasicStruct() {
    let p = Person {
        name:String::from("Alice"),
        age: 30,
    };

println!("{} is {} years old ", p.name , p.age);
}
