fn main() {
    let mut Word = String::from("Hello");
    Word.replace_range(0..1, "M");
    println!("Word: {}", Word);
}
