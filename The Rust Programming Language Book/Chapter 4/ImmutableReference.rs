fn calculate_length(text: &String) -> usize {
    text.len()
}

fn main() {
    let message = String::from("Rust");
    let length = calculate_length(&message);
    println!("Length: {}", length);
}
