fn main() {
    let Data: String = String::from("Hello, Immutable Borrowing!");
    let Ref: &String = &Data; // Immutable borrowing
    println!("Borrowed Data: {}", Ref);
}
