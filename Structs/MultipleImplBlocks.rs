struct  Point{
    x: i32 ,
    y: i32,

}

impl Point {
    fn New(x: i32 , y:i32 ) -> Self {
        Self { x, y}
    }
}

impl Point {
    fn DistanceFromOrigin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

fn MultipleImpl () {
    let pt = Point::New(3, 4);
    println!("Distance {}", pt.DistanceFromOrigin());
}
