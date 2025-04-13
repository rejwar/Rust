struct Rectangle (i32, i32);

fn GetArea (rect: Rectangle) -> i32
{
rect.0 * rect.1
}

fn main()
{
    let rect = Rectangle(10,20);
    println!("Area: {}", GetArea(rect));
}
