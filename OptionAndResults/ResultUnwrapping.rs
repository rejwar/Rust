fn main() {
    let result: Result<i32, &str> = Ok(100);
    println!("Result: {}", result.unwrap());
}
