fn main() {
    let result: Result<i32, &str> = Ok(5);
    let new_result = result.and_then(|v| Ok(v + 10)); // Add 10 if Ok
    println!("{:?}", new_result);
}
