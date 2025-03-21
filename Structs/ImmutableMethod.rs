struct Person {
    name: String,
}

impl Person {
    fn greet(&self) {
        println!("Hello, my name is {}!", self.name);
    }
}

fn main() {
    let person = Person { name: String::from("Alice") };
    person.greet();
}
