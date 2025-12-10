struct Point {
    x: String,
    y: String,
}

fn main() {
    let p = Point {
        x: String::from("Axis"),
        y: 10,
    };

    let Point { x, y } = p;

    println!("Moved x {}", x);
}
