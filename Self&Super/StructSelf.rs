struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn info(&self) {
        println!("Name {} , age {} ", self.name, self.age);
    }
}

fn main() {
    let p = Person {
        name: String::from("Alice"),
        age: 24,
    };
    p.info();
}
