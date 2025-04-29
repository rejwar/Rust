fn main() {
    let Data: String = String::from("Hello, Rust!");
    let RefData: &String = &Data; // Borrowing without taking ownership

    println!("Original Data: {}", Data);
    println!("Referenced Data: {}", RefData); // Accessing through reference
}
