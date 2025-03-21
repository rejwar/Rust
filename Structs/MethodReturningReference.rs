struct Text {
    content: String,
}

impl Text {
    fn as_str(&self) -> &str {
        &self.content
    }
}

fn main() {
    let text = Text {
        content: String::from("Hello, Rust!"),
    };
    println!("Text: {}", text.as_str());
}
