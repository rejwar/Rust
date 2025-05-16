use std::{fmt::Write, process::Output};

fn main () {
    let mut output = String::new();

    write!(output , "First Value: {}, Second value : {}" , 10,20).unwrap();
    println!("{}" ,output);
}
