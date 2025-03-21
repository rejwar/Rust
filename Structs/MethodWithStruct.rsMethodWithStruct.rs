struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

fn main() {
    let mut point = Point { x: 0, y: 0 };
    point.translate(5, -3);
    println!("Point: ({}, {})", point.x, point.y);
}
