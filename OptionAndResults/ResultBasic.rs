fn main() {
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(v) => println!("Success: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
