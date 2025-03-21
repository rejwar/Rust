fn main() {
    let result: Result<i32, &str> = Ok(10);
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(message) => println!("Error: {}", message),
    }
}
