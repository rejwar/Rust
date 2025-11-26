struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn print(&self) {
        println!("Point {} {}", self.x, self.y);
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    p.print();
}
