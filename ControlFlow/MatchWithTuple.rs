fn main() {
    let point = (3, 4);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis: x={}", x),
        (0, y) => println!("On y-axis: y={}", y),
        (x, y) => println!("Point: ({}, {})", x, y),
    }
}
