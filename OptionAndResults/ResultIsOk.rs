fn main() {
    let result: Result<i32, &str> = Ok(25);
    if result.is_ok() {
        println!("Operation succeeded!");
    }
}
