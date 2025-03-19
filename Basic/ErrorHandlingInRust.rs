fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(4.0, 2.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // Using Option<T>
    let maybe_number: Option<i32> = Some(5);
    if let Some(number) = maybe_number {
        println!("Number: {}", number);
    } else {
        println!("No number found");
    }
}
