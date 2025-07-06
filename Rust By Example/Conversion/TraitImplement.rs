use std::sync::mpsc::RecvTimeoutError;

#[derive(Debug)]


struct  Point {


x:i32,
y:i32,

}

impl From<(i32 , i32 )> for Point {
    fn from(value: (i32 , i32 )) -> Self {
        Point { x: value.0, y: value.1 }
    }
}

fn main() {
    let tuple = (10,20);
    let Point =Point::from(tuple);
    println!("Point {:?}", Point);
}
