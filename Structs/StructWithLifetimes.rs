struct Text<'a> {
    content: &'a str,
}

fn main() {
    let message = String::from("Hello, Rust!");
    let text = Text { content: &message };
    println!("Content: {}", text.content);
}
