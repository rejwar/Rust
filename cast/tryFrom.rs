use std::{convert::TryFrom, result}; 
#[derive(Debug)]

struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value %2 == 0{
            Ok(EvenNumber(value))
        } else {
            Err("Not an even Number")
        }
    }
}

fn main() {
    let result = EvenNumber::try_from(42);
    println!("TryFrom {:?}", result);
}