struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };
    let Point { x, y } = point;
    println!("x: {}, y: {}", x, y);
}
