fn main() {
    let Data: String = String::from("Closure Borrowing");
    let Closure = || {
        println!("Data: {}", Data); // Immutable borrowing
    };
    Closure();
}
