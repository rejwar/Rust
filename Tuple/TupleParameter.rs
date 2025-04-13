fn PrintCoordinates(coords: (f64 , f64)) {
    println!("X: {} , Y: {}", coords.0 , coords.1);
}

fn main()
{
    let point = (5.0 , 10.0);
    PrintCoordinates(point);
}
