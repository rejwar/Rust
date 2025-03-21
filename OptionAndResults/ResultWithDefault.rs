fn main() {
    let result: Result<i32, &str> = Err("Error");
    println!("Result: {}", result.unwrap_or(0)); // Use default
}
