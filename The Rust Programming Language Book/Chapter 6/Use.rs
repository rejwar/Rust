// Question: How can we use `if let` to simplify pattern matching?

fn UseIfLet() {
    let value = Some("Rust");

    if let Some(lang) = value {
        println!("You're learning {}", lang);
    }
}

fn main() {
    UseIfLet();
}
