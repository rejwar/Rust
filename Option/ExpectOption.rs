fn main() {
    let item = Some("Rust");
    println!("Item: {}", item.expect("Item not found"));

    let none_item: Option<&str> = None;
}
