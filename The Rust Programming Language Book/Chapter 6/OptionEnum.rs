fn Divide(Num1: f64, Num2: f64) -> Option<f64> {
    if Num2 == 0.0 {
        None 
    } else {
        Some(Num1 / Num2) //
    }
}

fn Main() {
    let Result = Divide(10.0, 2.0);
    match Result {
        Some(Value) => println!("Result: {}", Value), // 
    }

    let InvalidResult = Divide(10.0, 0.0);
    match InvalidResult {
        Some(Value) => println!("Result: {}", Value),
        None => println!("Cannot divide by zero!"), //
    }
}
