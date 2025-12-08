#[derive(Debug, Copy, Clone)]

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    use_point(p1);
    println!(" Back in main p1 is {:?}", p1);
}

fn use_point(p: Point) {
    println!("Inside function {:?} ", p);
}
