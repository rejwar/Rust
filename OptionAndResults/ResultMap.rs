fn main() {
    let result: Result<i32, &str> = Ok(10);
    let new_result = result.map(|v| v * 2); // Multiply by 2 if Ok
    println!("{:?}", new_result);
}
