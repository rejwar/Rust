fn main() {
    let text: &str = "Rust language!";
    let slice = &text[0..4]; // "Rust"
    println!("Slice: {}", slice);
}
