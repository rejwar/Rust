struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

fn main() {
    let circle = Circle::new(5.0);
    println!("Radius: {}", circle.radius);
}
