
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}


const ORIGIN: Point = Point::new(0, 0);

fn main() {
    println!("Origin: ({}, {})", ORIGIN.x, ORIGIN.y);
}
