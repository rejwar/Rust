fn main() {
    let value: Option<i32> = Some(42);
    match value {
        Some(number) => println!("Number: {}", number),
        None => println!("No value found"),
    }
}
