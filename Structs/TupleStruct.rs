struct Point(i32, i32);

fn main() {
    let point = Point(10, 20);
    println!("Point: ({}, {})", point.0, point.1);
}
