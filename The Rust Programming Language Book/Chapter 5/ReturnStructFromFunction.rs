// How to return struct from function?

struct Point {
    x: i32,
    y: i32,
}

fn CreatePoint(x: i32, y: i32) -> Point {
    Point { x, y }
}

fn UseReturnedStruct() {
    let p = CreatePoint(5, 7);
    println!("Point: ({}, {})", p.x, p.y);
}
