use std::fmt::{self, Write};

struct  Rectangle {
    width: u32,
    height: u32,
}

impl fmt::Display for Rectangle {
    fn fmt (&self , f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle (width : {} , height : {} )" , self.width , self.height)
    }
}

fn main () {
    let rect = Rectangle {
        width: 10,
        height: 5,
    };
    let mut output = String::new();

    write!(output , "Shape : {}" , rect). unwrap();
    println!("{}" , output);
}
