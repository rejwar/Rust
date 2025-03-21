#[derive(Default)]
struct Point(i32, i32);

fn main() {
    let point: Point = Default::default();
    println!("Point: ({}, {})", point.0, point.1);
}
