struct Point {
    X: i32 ,
    Y: i32 ,
}

fn main() {
    let P = Box::new(Point{X: 10 , Y: 20});
    println!("Point is {} {}",P.X , P.Y );
}