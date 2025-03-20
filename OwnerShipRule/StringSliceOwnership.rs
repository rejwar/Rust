fn main() {
    let text = String::from("Ownership rules");
    let slice = &text[..9]; // Borrow string as slice
    println!("Slice: {}", slice);
}
