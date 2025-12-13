fn main() {
    let s = String::from("Hello World");

    let substring: &str = &s[0..5];
    println!("Substracting {}", substring);
}
