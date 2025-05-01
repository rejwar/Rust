struct  Circle {
    radius: f64,
}

impl Circle {
    fn Area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

fn StructAssiociatedFunction() {
    let c = Circle { radius : 3.0};
    println!("Area : {:.2}" , c.Area());
}
