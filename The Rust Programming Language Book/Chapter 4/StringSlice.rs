fn main() {
    let text = String::from("Hello World");
    let hello  = &text[..5];
    let world: &str = &text [6..];

    println!("{} {}",hello , world );

}
