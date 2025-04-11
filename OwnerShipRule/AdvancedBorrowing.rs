fn main() {
    let Text: String = String::from("Complex Borrowing!");
    let Ref: &String = &Text; // Immutable reference
    println!("{}", Ref);
    // Changing ownership not allowed here without mutable borrow
}
