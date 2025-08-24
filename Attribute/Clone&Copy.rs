#[derive(Debug, Clone, Copy)]

struct Point{
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point{x: 10 , y: 20};
    let p2 = p1;

    println!("{:?} {:?}", p1, p2);
}