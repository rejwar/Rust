struct  user {
    name: String,
}

impl user {
    fn New(name: &str) -> Self {
        user {name: name.to_string()
        }
    }

    impl user {
        fn Greet(&self) {
            println!("Hello {}", self.name);
        }
    }
}