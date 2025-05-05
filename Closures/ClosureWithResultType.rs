fn ExecuteOperation<F: Fn(i32, i32) -> Result<i32, String>>(Operation: F) {
    match Operation(10, 0) {
        Ok(Result) => println!("Result: {}", Result),
        Err(Error) => println!("Error: {}", Error),
    }
}

fn main() {
    let Divide = |a: i32, b: i32| {
        if b == 0 {
            Err(String::from("Division by zero error"))
        } else {
            Ok(a / b)
        }
    };

    ExecuteOperation(Divide);
}
