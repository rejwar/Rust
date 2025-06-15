fn main() {
    let greeting: &str = "Hello, Rust!";
    let slice = &greeting[..5]; // "Hello"

    println!("{}", slice);
}
