struct Builder {
    text: String,
}

impl Builder {
    fn add(&mut self, word: &str) -> &mut Self {
        self.text.push_str(word);
        self
    }
}

fn main() {
    let mut builder = Builder { text: String::new() };
    builder.add("Hello ").add("World!");
    println!("{}", builder.text);
}
