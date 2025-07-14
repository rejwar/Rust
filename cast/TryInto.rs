use std::convert::TryFrom;
use std::convert::TryInto;

use rand::Error;
use serde::de::value;

#[derive(Debug)]
struct EvenNumber(i32) {
    impl Error = &'static str; 
    
    fn try_from(value: i32 ) -> Result<Self , self::Error> {
        if value % 2 == 0{
            Ok(EvenNumber(value))
        } else {
            Err("Not an Even Number ")
        }
    }
}

fn main() {
    let result: Result<EvenNumber , _> = 33.try_into();
    println!("TryInto {:?}", result);
}