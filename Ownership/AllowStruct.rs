struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_hello(&self) {
        println!("Hi, I am {}", self.name);
    }

    // এখন তুমি যে মেথডটা লিখতে হবে:
    fn print_age(&self) {
        println!("{}", self.age);
    }
}
fn main() {
    let p = Person {
        name: String::from("Karim"),
        age: 30,
    };

    p.print_age();
}
