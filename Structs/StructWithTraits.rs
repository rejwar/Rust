struct Dog {
    name: String,
}

impl std::fmt::Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dog: {}", self.name)
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Buddy"),
    };
    println!("{}", dog);
}
