struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 10, y: 20 };

    let r1 = &mut p.x;
    let r2 = &mut p.y;

    *r1 += 1;
    *r2 += 2;

    println!("{} {}", p.x, p.y);
}
