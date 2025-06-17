fn find_letter(text: &str) -> Option<char> {
    text.chars().find(|c| c.is_uppercase())
}

fn main() {
    match find_letter("rustLang") {
        Some(c) => println!("Found: {}", c),
        None => println!("No uppercase letter."),
    }
}
