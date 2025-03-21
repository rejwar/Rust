fn main() {
    let value: Option<i32> = None;
    println!("Value: {}", value.unwrap_or(0)); // Use default
}
