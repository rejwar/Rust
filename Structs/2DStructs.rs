struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
}

fn main() {
    let mut a = Vec2::new(3.0, 4.0);
    let b = Vec2::new(1.0, 2.0);

    let c = a.add(&b);
    println!("c = ({}, {})", c.x, c.y);

    println!("a length = {}", a.length());
    a.scale(2.0);
    println!("a after scale = ({}, {})", a.x, a.y);
}
//
