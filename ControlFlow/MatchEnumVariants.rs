enum Shape {
    Circle (f64),
    Rectangle {width: f64 , height:f64},
    Triangle (f64, f64 , f64),
}

fn EnumMatchExample (shape: Shape) {
    match  shape{
        Shape:: Circle(radius) => println!("Circle With radius {}", radius),
        Shape::Rectangle { width, height } => println!("Rectangle {} x {}" , width, height),
        Shape::Triangle(a,b ,c ) => println!("Triangle sides : {} , {} ,{} ", a,b,c),

    }
}

fn main() {
    EnumMatchExample(Shape::Circle((5.0)));
    EnumMatchExample(Shape::Rectangle{width:3.0 , height: 4.0});
    EnumMatchExample(Shape::Triangle(3.0, 4.0, 5.0));
}
