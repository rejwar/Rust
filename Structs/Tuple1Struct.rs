use std::num::ParseIntError;

struct  Point( i32 , i32);

fn main() {
    let Position:Point = Point(10,20);
    println!("X : {} , Y : {}" , Position.0 , Position.1);
}
