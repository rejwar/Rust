use core::error;
use std::string;

fn parse_number(input: &str) -> Result<i32 , String> {
    match input.trim().parse::<i32>() {
        Ok(Num) =>Ok(Num),
        Err(_) =>Err("Failed to parse Number  ". to_string()),
    }
}

fn main() {
    match parse_number("43"){
        Ok(value) => println!("Parsed :{}", value),
        Err(error) => println!("Error :{}", error),
    }

}
