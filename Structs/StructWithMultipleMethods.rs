struct Circle {
    radius: f64,
}

impl Circle {
    fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    println!("Diameter: {}, Circumference: {}", circle.diameter(), circle.circumference());
}
