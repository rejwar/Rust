// Question: How does `Result` enum help handle errors safely?

fn Divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn UseResultEnum() {
    match Divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match Divide(10, 0) {
        Ok(_) => println!("Will not happen"),
        Err(e) => println!("Caught error: {}", e),
    }
}
