struct  User {
    name: String,

}

impl User  {
    fn New(name: &str ) -> Self {
        User {name: name.to_string()
        }
    }
}

impl  User {
    fn Greet(&self) {
        println!("Hello {}", self.name);
    }
}