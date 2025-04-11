struct Person {
    Name: String,
    Age: u32,
}

fn main() {
    let PersonData: Person = Person {
        Name: String::from("Alice"),
        Age: 30,
    };
    println!("Name: {}", PersonData.Name);
    println!("Age: {}", PersonData.Age);
}
