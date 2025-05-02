fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
