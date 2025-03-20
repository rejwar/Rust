fn main() {
    let message = String::from("Hello, Rust!");
    let reference = get_reference(&message);
    println!("Reference: {}", reference);
}

fn get_reference(input: &String) -> &String {
    input
}
