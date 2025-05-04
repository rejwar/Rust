fn Divide(a: i32 , b: i32) -> Result<i32 , String> {
    if b == 0 {
        Err ( "Cannot divide by Zero " .to_string())
    } else {
        Ok(b / b)
    }
}

fn ResultExample() {
    match Divide(10, 2) {
        Ok(Result) => println!("Result {}" , Result),
        Err(e) => println!("Error : {}" , e),
        
    }
}
