fn get_first_char(s: &str) -> &str {
    &s[0..1];
}

fn main() {
    let original_String = String::from("Rust");
    let first = get_first_char(&original_String);

    println!("First character is {}", first);
}
