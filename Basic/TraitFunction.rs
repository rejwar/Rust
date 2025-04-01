
trait Shape{
    fn Area(&self) -> f64;
}

struct Circle{
    radius: f64,
}

impl Shape for Circle {
    fn Area(&self) -> f64{
        3.1416 * self.radius * self.radius
    }
}

fn PrintArea<T: Shape> (shape: &T){
    println!("Area:{}", shape.Area())
}

fn main() {
    let circle = Circle {radius: 7.0};
    PrintArea(&circle);
}
