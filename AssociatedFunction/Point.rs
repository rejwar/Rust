// A simple struct to hold a value.
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Associated Function: Used like a constructor.
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    // Associated Function: Creates a Point at the origin (0, 0).
    pub fn origin() -> Point {
        Point::new(0, 0)
    }

    // Method: Calculates the distance from the origin. Takes &self.
    pub fn distance_from_origin(&self) -> f64 {
        let x_sq = (self.x as f64).powi(2);
        let y_sq = (self.y as f64).powi(2);
        (x_sq + y_sq).sqrt()
    }
}

fn main() {
    // Calling an associated function (like a constructor)
    let p1 = Point::new(3, 4);

    // Calling another associated function
    let p0 = Point::origin();

    // Calling a method on an instance
    println!("p1 এর স্থানাঙ্ক: ({}, {})", p1.x, p1.y);
    println!("p1 এর দূরত্ব: {:.2}", p1.distance_from_origin());
    println!("p0 এর স্থানাঙ্ক: ({}, {})", p0.x, p0.y);
}
