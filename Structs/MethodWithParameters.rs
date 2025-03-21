struct Circle {
    radius: f64,
}

impl Circle {
    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

fn main() {
    let mut circle = Circle { radius: 5.0 };
    circle.scale(2.0);
    println!("Scaled Radius: {}", circle.radius);
}
