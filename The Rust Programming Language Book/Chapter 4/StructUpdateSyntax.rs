struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p1 = Person {
        name: String::from("Alice"),
        age: 25,
    };

    let p2 = Person {
        name: String::from("Bob"),
        ..p1
    };

    println!("p2: {}, {}", p2.name, p2.age);
    // println!("p1: {}", p1.name); ❌ Error: moved
}
