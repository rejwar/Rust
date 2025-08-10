#[derive(Debug)]
struct Point {
    x: i32 ,
    y: i32 ,
}

fn main() {
    let x = 10;
    let y: i32 = 12;

    let p = Point {x , y};
    println!("{:?}",p);
}