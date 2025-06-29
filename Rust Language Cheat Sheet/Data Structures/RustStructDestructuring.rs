struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x:20 , y:15};

    let Point {x , y} = point;

    println!( " x : {}  , Y :{}", x,y);
}
