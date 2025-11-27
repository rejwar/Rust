struct Rect {
    width: f64,
    height: f64,
}

impl Rect {
    fn new(width: f64, height: f64) {
        Self { width, height }
    }
}

impl area for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let r = Rect::new(3.14, 4.40);
    println!("Area is {}", r.area);
}
