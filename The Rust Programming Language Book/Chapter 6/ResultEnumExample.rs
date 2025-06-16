// Question: How do we use the `Result` enum to safely handle errors?

fn Divide(dividend: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(dividend / divisor)
    }
}

fn UseResultEnum() {
    match Divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    match Divide(10, 0) {
        Ok(result) => println!("This won't happen"),
        Err(err) => println!("Caught error: {}", err),
    }
}
