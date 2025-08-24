fn main() {
    let p = Box::new(Point {X: 10 , Y: 20});
    println!("Point = ({}, {})", p.X , p.Y);

    p.X += 10;
    p.Y += 30;
    println!("Updated Point ({} , {})", p.X , p.Y);
}