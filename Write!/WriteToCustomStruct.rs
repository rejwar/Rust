use std::{fmt::{self, Write}, process::Output};

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f , "({} , {})" , self.x , self.y)
    }
}

fn main() {
    let p = Point {x : 5 , y:10};
    let mut output = String::new();

    write!(output , "Point : {}" , p).unwrap();
    println!("{}" ,  output);
}
