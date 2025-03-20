struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 25,
    };
    let ref_name = &person.name; // Immutable reference
    println!("Name: {}", ref_name);
}
