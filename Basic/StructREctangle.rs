struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn Area($self)-> u32{
        self.width * self.height
    }
}

fn main()
{
    let rect = Rectangle {width: 10 , height: 5 };
    println!("Area: {}", rect.Area());
}
