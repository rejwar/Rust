struct Name {
    value: String,
}

impl Name {
    fn take_ownership(self) -> String {
        self.value
    }
}

fn main() {
    let name = Name { value: String::from("Rust") };
    let owned_value = name.take_ownership();
    println!("{}", owned_value);
}
