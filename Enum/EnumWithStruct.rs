enum Shape {
    Rectangle { width: u32, height: u32 },
    Circle { radius: f64 },
}

fn main() {
    let shape = Shape::Rectangle { width: 10, height: 20 };
    match shape {
        Shape::Rectangle { width, height } => println!("Rectangle: {} x {}", width, height),
        Shape::Circle { radius } => println!("Circle radius: {}", radius),
    }
}
