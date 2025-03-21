fn main() {
    let value: Option<i32> = Some(42);
    match value {
        Some(v) => println!("Value: {}", v),
        None => println!("No value found"),
    }
}
