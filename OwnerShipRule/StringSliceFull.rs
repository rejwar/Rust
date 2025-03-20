fn main() {
    let s = String::from("Hello, Rust!");
    let slice = &s[..]; // Slice the entire string
    println!("Slice: {}", slice);
}
