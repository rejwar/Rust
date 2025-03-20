struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = create_point(10, 20);
    println!("Point: ({}, {})", point.x, point.y);
}

fn create_point(x: i32, y: i32) -> Point {
    Point { x, y }
}
