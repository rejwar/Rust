struct Point {
    x: String,
    y: i32,
}

fn main() {
    let p = Point {
        x: String::from("X- Axis "),
        y: 10,
    };
    let point { x, y } = p;
    println!("Moved x is {}", x);
}
