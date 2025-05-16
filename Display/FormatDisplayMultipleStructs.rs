use std::fmt;

#[derive(Debug)]
struct  MinMax(i64 , i64);

impl fmt::Display for MinMax {
    fn fmt (&self , f: &mut fmt:: Formatter) -> fmt::Result {
        write!(f , "({} ,{})", self.0 , self.1)
    }
}

#[derive(Debug)] 
struct Point2D {
    x : f64,
    y : f64,
}

impl fmt:: Display for Point2D {
    fn fmt( &self , f: &mut fmt:: Formatter ) -> fmt::Result {
        write!(f , "x:{}  , y: {}" , self.x , self.y)
    }
}

fn main() {
    let minmax = MinMax (0, 14);
    println!("Compare strutures :");
    println!("Display MinMax {}" , minmax);
}
