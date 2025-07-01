

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let x = 10;
    let y = 20;

    // Since variable names match field names, we can use shorthand:
    let p = Point { x, y }; // same as Point { x: x, y: y }

    println!("Point is at ({}, {})", p.x, p.y);
}
