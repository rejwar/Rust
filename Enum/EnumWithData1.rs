enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn main() {
    let shape = Shape::Circle(10.5);
    match shape {
        Shape::Circle(radius) => println!("Circle with radius {}", radius),
        Shape::Rectangle(width, height) => println!(
            "Rectangle with width {} and height {}", 
            width, 
            height
        ),
    }
}
