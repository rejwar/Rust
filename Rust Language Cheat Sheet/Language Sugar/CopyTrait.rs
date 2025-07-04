#[derive(Copy,Clone,Debug)]

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point{x: 32, y: 33 };
    let b =a;

    println!(" a  : {:?}",a );
    println!("b : {:?}",b);
}
