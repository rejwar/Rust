fn main() {
    let text = String::from("Hello, World!");
    let slice = &text[..5]; // Shorthand for "Hello"
    println!("Slice: {}", slice);
}
