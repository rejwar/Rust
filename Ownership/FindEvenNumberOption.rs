use std::fmt::Error;

fn Divide(Numerator : i32 , Denonibator:i32 ) -> Result<i32 , String > {
    if Denonibator == 0 {
        Err ( String :: from("Division by Zero error"))
    } else {
 Ok(Numerator / Denonibator)
    
 }
}
 fn main() {
    match Divide(10, 2 ) {
        Ok(Result) => println!("Result {}", Result),
        Err(Error) => println!("Error {}", Error),
    }
 }
