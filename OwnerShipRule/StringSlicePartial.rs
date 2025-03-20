fn main() {
    let text = String::from("Hello, Rust!");
    let slice = &text[7..11]; // Partial slice: "Rust"
    println!("Slice: {}", slice);
}
