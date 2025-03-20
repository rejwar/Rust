struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let MyPoint = Point { x: 10, y: 20 };
    println!("MyPoint: x={}, y={}", MyPoint.x, MyPoint.y);
}
