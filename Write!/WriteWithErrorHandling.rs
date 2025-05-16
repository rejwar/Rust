use std::{fmt::Write, process::Output};

fn main() {
    let mut output = String::new();

    match write!(output, "Value: {}" , 42) {
        Ok(_) => println!("Write successful Output : {}" , output),
        Err(e) => println!("Error writing to string : {}" , e),
        
    }
}
