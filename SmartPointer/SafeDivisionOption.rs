use std::{collections::btree_map::Values, result};

fn divide(a:f64 , b:f64) ->Option<f64>
{
    if b ==0.0 {
        None
    } else {
        Some(a/b) 
    }
}

fn main() {
    let result = divide(10.0 , 2.0);

    match result {
        Some(Values) => println!("Result: {}" , Values),
        None => println!("Cannot divide by zero"),
    }
}
