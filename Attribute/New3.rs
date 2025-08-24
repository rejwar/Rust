#[derive(Debug, Clone)]
struct Point {
    X: i32, 
    Y: i32 ,
}

fn main() {
    let P1 = Point {X: 10 , Y: 20};
    let P2 = P1.clone();
    println!("{:?} {:?}", P1, P2);
}