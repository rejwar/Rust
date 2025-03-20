fn main() {
    let string1 = String::from("Hello");
    let result = longest(string1.as_str(), "Rust");
    println!("Longest: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
