// What is a basic struct in Rust and how do you use it?

// Define a basic struct with named fields
struct Person {
    name: String,
    age: u8,
}

fn ShowPersonInfo() {
    // Create an instance of the struct
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Access fields
    println!("Name: {}, Age: {}", person.name, person.age);
}
