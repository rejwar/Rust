struct Circle {
    radius: f64,
}

impl Circle {
    fn new(r: f64) -> Circle {
        // associated function
        Circle { radius: r }
    }
}
fn main() {
    let c = Circle::new(5.0);
    println!("Circle radius: {}", c.radius);
}
