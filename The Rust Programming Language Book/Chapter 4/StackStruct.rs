struct Point {
    X: i32,
    Y: i32,
}

fn main() {
    let P = Point { X: 10, Y: 20 };
    println!("({}, {})", P.X, P.Y);
}
