struct Dog {
    name: String,
    age: u8,
}

fn mutable_struct() {
    let mut dog = Dog {
        name: String::from("Rex"),
        age: 5,
    };

    dog.age += 1;

    println!("{} is now {} years old", dog.name, dog.age);
}
