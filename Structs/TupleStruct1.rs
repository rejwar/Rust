struct Circle {
    radius:f64,
}

impl Circle {
    fn New(radius: f64 ) -> Self{
        Circle { radius }
    }

    fn Area(&self) ->f64 {
        3.1416 * self.radius * self.radius
    }
}

fn main() {
    let c = Circle::New(5.0);
    println!("Area: {}", c.Area());
}
