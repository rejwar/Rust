enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn main() {
    let MyShape:Shape = Shape::Circle(10.5);

    match MyShape {
        Shape::Circle(Radius ) => println!("Circle with Radius {}", Radius),
        Shape::Rectangle(Width, Height) => println!("Rectangle with width {} and height {}", Width , Height),
    }
}
