fn Divide (x: i32 , y:i32) -> Result<i32, String>
{
    if y ==0{
        Err("Can not divide by zero".to_string())
    } else {
        Ok (x/y)
    }
}

fn main() {
    match Divide(10, 0) {
        Ok(Result) => println!("Result: {}" , Result),
        Err(e) => println!("Error : {}",e),
    }
}
