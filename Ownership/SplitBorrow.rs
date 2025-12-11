struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 32, y: 40 };

    let r1 = &mut p.x;
    let r2 = &mut p.y;

    *r1 += 1;
    *r2 += 2;

    println!(" The value of r1 is  {}", r1);
    println!("The value of r2 is {}", r2);
}
