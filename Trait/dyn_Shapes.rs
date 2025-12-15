trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
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

fn main() {
    let circle_instance = Circle { radius: 5.0 };
    let rect_instance = Rectangle {
        width: 10.0,
        height: 4.0,
    };

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(circle_instance), Box::new(rect_instance)];

    println!("-------------- Calculating Areas using trait -----");

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape is {} Area {:.2}", i + 1, shape.area());
    }

    println!("\n No Explanation sir ");
}
