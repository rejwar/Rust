struct Builder {
    value: String,
}

impl Builder {
    fn append(&mut self, text: &str) -> &mut Builder {
        self.value.push_str(text);
        self
    }
}

fn main() {
    let mut builder = Builder { value: String::new() };
    builder.append("Hello").append(", Rust!");
    println!("{}", builder.value);
}
