use std::fmt::{Display , Debug};

struct wrapper<T: Display + Debug> {
    value: T,
}

impl <T: Display + Debug> wrapper<T> {
    fn describe(&self){
        println!("Value is {}", self.value);
        println!("Debig is {}", self.value);
    }
}