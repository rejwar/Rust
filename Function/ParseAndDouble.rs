fn ParseAndDouble(input: &str ) -> Result<i32, String>
{
    match input. trim().parse::<i32>(){
        Ok(n) => Ok(n*2),
        Err(_)=> Err("Invalid Number ". to_string()),
    }
}

fn main() {
    match ParseAndDouble("42"){
        Ok(v) => println!("Double {}" , v),
        Err(e) => println!("Error: {}", e),
    }
}
