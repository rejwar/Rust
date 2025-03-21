struct Text<'a> {
    content: &'a str,
}

impl<'a> Text<'a> {
    fn display(&self) -> &str {
        self.content
    }
}

fn main() {
    let sentence = String::from("Hello, Rust!");
    let text = Text { content: &sentence };
    println!("{}", text.display());
}
