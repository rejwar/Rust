fn main() {
    let text = String::from("Learning Rust");
    let slice = get_slice(&text);
    println!("Slice: {}", slice);
}

fn get_slice(input: &str) -> &str {
    &input[0..8] // Returns "Learning"
}
