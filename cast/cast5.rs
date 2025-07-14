use core::num;
use std::convert::TryFrom;


#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value %2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err("THe number is not even")
        }
    } 
}

fn main() {
    match EvenNumber::try_from(43) {
        Ok(num) => println!("{:?}", num),
        Err(msg) => println!("Error {:?}", msg),
    }
}