trait Drawable {
    fn  draw(&self);
}

struct Circle {
    radius : f64,
}

impl Drawable for Circle {
    fn  draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

struct Square {
    side: f64,
}

impl Drawable for Square {
    fn  draw(&self) {
        println!("Drawing a square with side {}", self.side);        
    }
}

fn main() {
    let circle  = Circle { radius : 5.0};
    let square = Square {side: 4.0};

    let shapes: [&dyn Drawable ; 2 ] =[ &circle , &square];

    for shape in shapes.iter() {
        shape.draw();
    }
}
