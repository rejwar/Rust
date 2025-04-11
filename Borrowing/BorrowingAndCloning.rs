fn main() {
    let Data: String = String::from("Borrowing And Cloning");
    let Ref: String = Data.clone(); // Cloning to avoid ownership issues
    println!("Original: {}", Data);
    println!("Clone: {}", Ref);
}
