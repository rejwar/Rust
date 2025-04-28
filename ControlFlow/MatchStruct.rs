struct  Point {
    x :i32, 
    y : i32 ,
}

fn StructMatchExample (p: Point) {
    match p {
        Point { x , y: 0} => println!("Point lies on x asis at {}" , x),
        Point { x: 0 , y} => println!("Point lies in y axis at {}", y),
        Point { x, y} => println!("Point lies at {} {}" , x , y),
    }
}

fn main() {
    let p1 = Point {x: 5 , y:0};
    StructMatchExample(p1);

    let p2 = Point{x:0 , y : 7};
    StructMatchExample(p2);

    let p3 =Point{x:2 , y: 9};
    StructMatchExample(p3);
}
