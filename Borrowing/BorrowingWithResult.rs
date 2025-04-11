fn main() {
    let Data: Result<String, &str> = Ok(String::from("Success Borrowing"));
    match &Data {
        Ok(Ref) => println!("Borrowed: {}", Ref),
        Err(Error) => println!("Error: {}", Error),
    }
}
