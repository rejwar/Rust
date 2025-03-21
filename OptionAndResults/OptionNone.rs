fn main() {
    let value: Option<i32> = None;
    match value {
        Some(v) => println!("Value: {}", v),
        None => println!("No value found"),
    }
}
