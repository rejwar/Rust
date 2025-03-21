enum Point {
    Coordinates(i32, i32),
    Origin,
}

fn main() {
    let point = Point::Coordinates(10, 20);
    match point {
        Point::Coordinates(x, y) => println!("Point: ({}, {})", x, y),
        Point::Origin => println!("Point is at the origin."),
    }
}
