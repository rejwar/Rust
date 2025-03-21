fn main() {
    let result: Result<i32, &str> = Err("Something went wrong");
    match result {
        Ok(v) => println!("Success: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
