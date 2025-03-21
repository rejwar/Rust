fn process(value: i32) -> Result<i32, &'static str> {
    if value > 0 {
        Ok(value * 2)
    } else {
        Err("Invalid value")
    }
}

fn main() {
    match process(5) {
        Ok(v) => println!("Success: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
