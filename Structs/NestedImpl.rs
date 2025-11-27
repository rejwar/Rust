struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

impl User {
    fn IsAdult(&self) -> bool {
        self.age >= 18
    }
}
