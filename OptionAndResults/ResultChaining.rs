fn main() {
    let result: Result<i32, &str> = Ok(10);
    let final_result = result.and(Ok(20)); // Chains results
    println!("{:?}", final_result);
}
