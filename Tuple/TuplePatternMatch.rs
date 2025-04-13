fn IdentifyPoint(point: (i32 , i32)) {
    match point{
        (0,0) => println!("Origin"),
        (x,0) => println!("X-Axis at {}",x),
        (0,y) => println!("Y-Axis at {}" , y),
        (x,y) => println!("Point at ({} , {})" ,x,y),
    }
}

fn main()
{
    IdentifyPoint((3,0));
}
