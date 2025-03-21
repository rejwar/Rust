struct Container {
    value: String,
}

impl Container {
    fn update(&mut self, new_value: &str) {
        self.value = new_value.to_string();
    }
}

fn main() {
    let mut container = Container {
        value: String::from("Old Value"),
    };
    container.update("New Value");
    println!("Updated Value: {}", container.value);
}
