fn main() {
    let reference = create_dangling_reference();
    println!("{:?}", reference); // Error prevented by Rust
}

fn create_dangling_reference() -> &String {
    let text = String::from("This will not work");
    &text // Rust prevents this
}
