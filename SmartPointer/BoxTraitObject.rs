trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}

struct Square {
    s: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.1416 * self.r * self.r
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.s * self.s
    }
}

fn UseBoxTraitObject() {
    let shapes: Vec<Box<dyn Area>> = vec![Box::new(Circle { r: 3.0 }), Box::new(Square { s: 2.0 })];

    for s in shapes {
        println!(" Area is {}", s.area());
    }
}

fn main() {
    UseBoxTraitObject();
}
