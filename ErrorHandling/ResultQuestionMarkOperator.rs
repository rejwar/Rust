fn try_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".into())
    } else {
        Ok(a / b)
    }
}

fn try_operator() -> Result<(), String> {
    let result = try_divide(10, 0)?;  // ? অপারেটর ব্যবহার
    
    // যদি বিভাজন সফল হয় (যা এই ক্ষেত্রে হবে না)
    println!("Division result: {}", result);
    Ok(())
}

fn main() {
    match try_operator() {
        Ok(()) => println!("Operation succeeded"),
        Err(e) => println!("Error: {}", e),
    }
}
