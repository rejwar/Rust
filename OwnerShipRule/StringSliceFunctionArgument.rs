fn main() {
    let text = String::from("Hello, Rust!");
    print_slice(&text[0..5]); // Passing slice
}

fn print_slice(slice: &str) {
    println!("Slice: {}", slice);
}
