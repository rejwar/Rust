trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn Print_area<T: Shape>(shape: &T) {
    println!("THe area is {:.2}", shape.area());
}

fn main() {
    let my_circle = Circle { radius: 5.0 };
    let my_rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    println!("--- Circle Area Calculation---");

    println!("Directly calculated Area {:.2}", my_circle.area());

    Print_area(&my_circle);

    println!("--- Directly Calculated Area {:.2}", my_rectangle.area());

    Print_area(&my_rectangle);
}
