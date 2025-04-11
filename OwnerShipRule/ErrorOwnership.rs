fn main() {
    let ResultData: Result<String, &str> = Ok(String::from("Success!"));
    match ResultData {
        Ok(Data) => println!("Result: {}", Data), // Ownership used
        Err(Error) => println!("Error: {}", Error),
    }
}
