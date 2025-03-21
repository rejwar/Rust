enum Container<'a> {
    Text(&'a str),
}

fn main() {
    let text = "Rust";
    let container = Container::Text(text);
    match container {
        Container::Text(value) => println!("Text: {}", value),
    }
}
