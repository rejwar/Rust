struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn Print(&self) {
        println!("Point {} {} ", self.x, self.y);
    }
}

fn main() {
    let p = Point::new(10, 20);
    p.Print();
}
