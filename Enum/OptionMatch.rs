fn find_item(item: Option<&str>) {
    match item {
        Some(name) => println!("Found item {}", name),
        None => println!("Item not found"),
    }
}

fn main() {
    let available_item = Some("Laptop");
    let missing_item = None;

    find_item(available_item);
    find_item(missing_item);
}
