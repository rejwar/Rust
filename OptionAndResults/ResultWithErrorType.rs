fn main() {
    let result: Result<i32, String> = Err(String::from("Custom error message"));
    match result {
        Ok(v) => println!("Success: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
