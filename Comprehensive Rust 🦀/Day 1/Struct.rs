struct  Point {
    x:i32,
    y:i32,
}


fn check_point(p: Point) {
    match p {
        Point { x:0 , y } => println!("Y okkho is not real {}",y),
        Point { x , y: i32} => println!("x Okkho {}" ,x),
        Point {x ,y} => println!("{}, {}", x,y),

    }
}
