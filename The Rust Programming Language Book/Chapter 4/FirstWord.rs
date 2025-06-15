fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    let sentence = String::from("Rust is awesome");
    let word = first_word(&sentence);

    println!("First word: {}", word);
}
